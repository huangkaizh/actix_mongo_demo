use serde::{Deserialize, Serialize};
use mongodb::{
    oid::ObjectId,
};
use wither::Model;

#[derive(Debug, Deserialize, Clone)]
pub struct Verify {
    pub appkey: String,
    pub sign: String,
    pub tpl_id: i32,
    pub minutes: String,
    pub sdkappid: i32
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub debug: bool,
    pub port: i32,
    pub host: String,
    pub max_age: i64,
    pub payload_limit_size: usize,
    pub cookie_name: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Db {
    pub mongo_host: String,
    pub mongo_port: u16,
    pub mongo_db_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_password: Option<String>,
    pub mongo_pool_max_size: u32
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub port: i32,
    pub host: String,
    pub max_age: i64,
    pub mongo_host: String,
    pub mongo_port: u16,
    pub mongo_db_name: String
}

#[derive(Debug, Model, Serialize, Deserialize, Clone)]
pub struct MyObj {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub number: i32,
}