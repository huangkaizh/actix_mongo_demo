use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::{CookieSession};
use actix_web::{
    App, HttpServer, middleware, web
};
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use actix_demo::middleware_local::auth::Auth;
use actix_demo::common::settings::{SERVER, DB};
use actix_demo::controllers::user::*;
use actix_demo::controllers::common_controller::*;

fn main() {
    log4rs::init_file("log.yml", Default::default()).unwrap();

    let server = (*SERVER).clone();
    let db = (*DB).clone();

    let manager = MongodbConnectionManager::new(
        if None != db.mongo_username && None != db.mongo_password {
            ConnectionOptions::builder()
                .with_host(&db.mongo_host, db.mongo_port)
                .with_db(&db.mongo_db_name)
                .with_auth(&db.mongo_username.unwrap(), &db.mongo_password.unwrap())
                .build()
        } else {
            ConnectionOptions::builder()
                .with_host(&db.mongo_host, db.mongo_port)
                .with_db(&db.mongo_db_name)
                .build()
        }
    );

    let pool = Pool::builder()
        .max_size(db.mongo_pool_max_size)
        .build(manager)
        .unwrap();

    let addr = format!("{0}:{1}", server.host, server.port);
    println!("addr: {}", addr);

    HttpServer::new(move || {
        let mut app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name(&server.cookie_name)
                    .secure(false)
//                    .max_age(server.max_age)
            ))
//            .wrap(Auth {})
            .data(web::JsonConfig::default().limit(server.payload_limit_size)) // <- limit size of the payload (global configuration)
            .data(pool.clone())
            .service(web::resource("/").wrap(Auth::default()).route(web::post().to(index)))
            .service(web::resource("/verify").route(web::post().to(verify)))
            .service(web::resource("/count").to(count))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/user/{name}/{nick}").route(web::get().to(with_param)))
            .default_service(
                web::resource("")
                    .route(web::get().to(p404))
            );
        app = app.configure(|cfg| {
            cfg.service(web::resource("/welcome").wrap(Auth::default()).route(web::post().to(welcome)));
        });
        app
    })
        .bind(addr).unwrap()
        .run().unwrap();
}

