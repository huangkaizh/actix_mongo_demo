use serde::{Deserialize, Serialize};
use mongodb::{
    oid::ObjectId,
};
use wither::Model;

#[derive(Debug, Model, Serialize, Deserialize, Clone)]
pub struct MyObj {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tel {
    pub mobile: String,
    pub nationcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgObj {
    pub ext: String,
    pub extend: String,
    pub params: [String; 2],
    pub sig: String,
    pub sign: String,
    pub tel: Tel,
    pub time: i64,
    pub tpl_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsRes {
    pub result: i32,
    pub errmsg: String,
    pub ext: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<i32>,
}