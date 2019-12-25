use actix_web::{
    Error, web, HttpResponse, HttpRequest
};
use actix_session::{Session};
use actix_web::http::StatusCode;
use actix_files as fs;
use mongodb::{
    db::ThreadedDatabase,
};
use r2d2::Pool;
use r2d2_mongodb::{MongodbConnectionManager};
use wither::Model;
use crate::models::common_models::MyObj;
use crate::common::pr_response::PrResponse;

pub fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// This handler uses json extractor
pub fn index(item: web::Json<MyObj>, pool: web::Data<Pool<MongodbConnectionManager>>) -> HttpResponse {
    let mut my_obj = item.0;
    let db = pool.get().unwrap();
    let _my_objs_one = db.collection("my_objs").find_one(None, None).unwrap();
    my_obj.save(db.clone(), None).unwrap();
    let mut cloned_my_obj = my_obj.clone();
    cloned_my_obj.name = String::from("huangzhi");
    let my_obj_bson = bson::to_bson(&cloned_my_obj).unwrap();
    my_obj.clone().update(db.clone(), None, doc! {"$set": my_obj_bson}, None).unwrap();
    HttpResponse::Ok().json(PrResponse::ok(my_obj)) // <- send response
}

pub fn count(session: Session, pool: web::Data<Pool<MongodbConnectionManager>>, req:HttpRequest) -> actix_web::Result<&'static str> {
    println!("{:?}", req);
    let db = pool.get().unwrap();
    let _my_objs_one = db.collection("my_objs").find_one(None, None).unwrap();
    println!("_my_objs_one: {:#?}", _my_objs_one);
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