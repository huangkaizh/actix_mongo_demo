#[macro_use]
extern crate json;
#[macro_use]
extern crate mongodb;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_session::{CookieSession, Session};
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web
};
use actix_web::http::StatusCode;
use json::JsonValue;
use mongodb::{
    db::ThreadedDatabase,
};
use qstring::QString;
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use wither::Model;
use actix_demo::middleware_local::auth::Auth;
use actix_demo::common::structs::*;
use actix_demo::controller::user::*;
use actix_demo::common::settings::{SERVER, DB};

fn ok_json(json_value: JsonValue) -> String {
    let obj = object!{
        "msg" => "操作成功",
        "code" => 0,
        "data" => json_value
    };
    return json::stringify(obj)
}

/// This handler uses json extractor
fn index(item: web::Json<MyObj>, pool: web::Data<Pool<MongodbConnectionManager>>) -> HttpResponse {
    let mut my_obj = item.0;
    let db = pool.get().unwrap();
    let _my_objs_one = db.collection("my_objs").find_one(None, None).unwrap();
    my_obj.save(db.clone(), None).unwrap();
    let mut cloned_my_obj = my_obj.clone();
    cloned_my_obj.name = String::from("huangzhi");
    let my_obj_bson = bson::to_bson(&cloned_my_obj).unwrap();
    my_obj.clone().update(db.clone(), None, doc! {"$set": my_obj_bson}, None).unwrap();
    HttpResponse::Ok().json(my_obj) // <- send response
}

fn count(session: Session, pool: web::Data<Pool<MongodbConnectionManager>>, req:HttpRequest) -> actix_web::Result<&'static str> {
    println!("{:?}", req);
    let db = pool.get().unwrap();
    let _my_objs_one = db.collection("my_objs").find_one(None, None).unwrap();
    println!("_myObjsOne{:?}", _my_objs_one);
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter") ? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }
    Ok("welcome!")
}

fn welcome(_id: Identity) -> HttpResponse {
    let arr = array!["a", "b", "c"];
    HttpResponse::Ok().content_type("application/json").body(ok_json(arr))
}

fn with_param(req: HttpRequest, path: web::Path<(String, String)>) -> HttpResponse {
    println!("{:?}", req);
    let query_str = req.query_string();
    println!("query_str: {}", query_str);
    let qs = QString::from(query_str);
    let phone = qs.get("phone").unwrap();
    println!("path: {:?}", path);
    HttpResponse::Ok().content_type("text/plain").body(format!("Hello {}!You phone is {}, your nick is {}", path.0, phone, path.1))
}

fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

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
                    .name("pr-auth-cookie")
                    .secure(false)
                    .max_age(server.max_age)
            ))
//            .wrap(Auth {})
            .data(web::JsonConfig::default().limit(server.payload_limit_size)) // <- limit size of the payload (global configuration)
            .data(pool.clone())
            .service(web::resource("/").route(web::post().to(index)))
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

