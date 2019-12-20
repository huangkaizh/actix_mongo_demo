#[macro_use]
extern crate json;
#[macro_use]
extern crate mongodb;
extern crate serde;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;
extern crate wither;
#[macro_use(Model)]
extern crate wither_derive;
#[macro_use]
extern crate log;
extern crate hex;
extern crate chrono;
extern crate actix_demo;

use actix_web::{
    middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer
};
use actix_web::http::{StatusCode};
use json::JsonValue;
use mongodb::{
    ThreadedClient,
    db::{Database, ThreadedDatabase},
    coll::options::IndexModel,
    oid::ObjectId,
};
use r2d2_mongodb::{MongodbConnectionManager, ConnectionOptions};
use r2d2::Pool;
use actix_demo::middlewareLocal::auth::Auth;
use rand::Rng;
use actix_session::{CookieSession, Session};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use qstring::QString;
use actix_files as fs;
use wither::prelude::*;

use actix_demo::common::structs::*;

//#[derive(Debug, Serialize, Deserialize, Model, Clone)]
//struct MyObj {
//    /// The ID of the model.
//    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//    pub id: Option<ObjectId>,
//    name: String,
//    number: i32,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct Tel {
//    mobile: String,
//    nationcode: String,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct LoginParams {
//    username: String,
//    password: String,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct MsgObj {
//    ext: String,
//    extend: String,
//    params: [String; 2],
//    sig: String,
//    sign: String,
//    tel: Tel,
//    time: i64,
//    tpl_id: i32,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct SmsRes {
//    result: i32,
//    errmsg: String,
//    ext: String,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    sid: Option<String>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    fee: Option<i32>,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct PrResponse<T>  {
//    msg: String,
//    code: i8,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    data: Option<T>,
//}
//
//impl<T> PrResponse<T> {
//    fn ok(data: T) -> PrResponse<T> {
//        PrResponse {
//            msg: "操作成功".to_string(),
//            code: 0,
//            data: Some(data)
//        }
//    }
//    fn timeout(data: T) -> PrResponse<T> {
//        PrResponse {
//            msg: "登录超时".to_string(),
//            code: 2,
//            data: Some(data)
//        }
//    }
//    fn error(msg: String) -> PrResponse<T> {
//        PrResponse {
//            msg: msg,
//            code: 1,
//            data: None
//        }
//    }
//}


fn ok_json(json_value: JsonValue) -> String {
    let obj = object!{
        "msg" => "操作成功",
        "code" => 0,
        "data" => json_value
    };
    return json::stringify(obj)
}

const MAXAGE:i64 = 100000;

/// This handler uses json extractor
fn index(item: web::Json<MyObj>, pool: web::Data<Pool<MongodbConnectionManager>>) -> HttpResponse {
    let mut my_obj = item.0;
    let db = pool.get().unwrap();
    let my_objs_one = db.collection("my_objs").find_one(None, None).unwrap();
    my_obj.save(db.clone(), None);
    let mut cloned_my_obj = my_obj.clone();
    cloned_my_obj.name = String::from("huangzhi");
    let my_obj_bson = bson::to_bson(&cloned_my_obj).unwrap();
    my_obj.clone().update(db.clone(), None, doc! {"$set": my_obj_bson}, None).unwrap();
    HttpResponse::Ok().json(my_obj) // <- send response
}

fn count(session: Session, pool: web::Data<Pool<MongodbConnectionManager>>, req:HttpRequest) -> actix_web::Result<&'static str> {
    println!("{:?}", req);
    let db = pool.get().unwrap();
    let my_objs_one = db.collection("my_objs").find_one(None, None).unwrap();
    println!("myObjsOne{:?}", my_objs_one);
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

fn welcome1(_id: Identity) -> HttpResponse {
    let arr = array!["a", "b", "c"];
    HttpResponse::Ok().content_type("application/json").body(ok_json(arr))
}

fn login(id: Identity, item: web::Json<LoginParams>) -> HttpResponse {
    let login_params = item.0;
    println!("loginParams: {:?}", login_params);
    if login_params.username == "huangzhi" {
        let timestamp: i64 = chrono::Utc::now().timestamp() + MAXAGE;
        let random_num = rand::thread_rng().gen_range(100000000, 999999999);
        let id_str = format!("username={}&timestamp={}&random_num={}", login_params.username, timestamp, random_num);
        id.remember(id_str.to_owned()); // <- remember identity
        HttpResponse::Ok().json(login_params)
    } else {
        let err_res  = PrResponse::<String>::error("用户名或密码错误".to_owned());
        HttpResponse::Ok().json(err_res)
    }
}

fn logout(id: Identity) -> HttpResponse {
    id.forget();                      // <- remove identity
    HttpResponse::Ok().finish()
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

    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
            .with_db("actixDemoDb")
//            .with_auth("root", "password")
            .build()
    );

    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .unwrap();

    HttpServer::new(move || {
        let mut app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("pr-auth-cookie")
                    .secure(false)
                    .max_age(MAXAGE)
            ))
//            .wrap(Auth {})
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .data(pool.clone())
            .service(web::resource("/").route(web::post().to(index)))
//            .service(web::resource("/verify").route(web::post().to(verify)))
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
        .bind("127.0.0.1:8080").unwrap()
        .run().unwrap();
}