use r2d2_mongodb::{MongodbConnectionManager};
use r2d2::Pool;
use actix_web::web;

pub struct AppState {
    pool: web::Data<Pool<MongodbConnectionManager>>
}