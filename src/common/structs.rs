use serde_derive::{Deserialize, Serialize};
use mongodb::{
    oid::ObjectId,
};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PrResponse<T>  {
    pub msg: String,
    pub code: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> PrResponse<T> {
    pub fn ok(data: T) -> PrResponse<T> {
        PrResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: Some(data)
        }
    }
    pub fn timeout(data: T) -> PrResponse<T> {
        PrResponse {
            msg: "登录超时".to_string(),
            code: 2,
            data: Some(data)
        }
    }
    pub fn error(msg: String) -> PrResponse<T> {
        PrResponse {
            msg: msg,
            code: 1,
            data: None
        }
    }
}