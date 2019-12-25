use actix_identity::{Identity};
use crate::common::pr_response::{PrResponse, error};
use crate::models::user::{LoginParams, MsgObj, Tel, SmsRes};
use actix_web::{
    HttpResponse, web, HttpRequest
};
use rand::Rng;
use crate::common::settings::{ SERVER, VERIFY};
use sha2::{Sha256, Digest};
use qstring::QString;
use crate::common::functions::ok_json;

pub fn login(id: Identity, item: web::Json<LoginParams>) -> HttpResponse {
    let login_params = item.0;
    println!("loginParams: {:?}", login_params);
    info!("loginParams: {:?}", login_params);
    if login_params.username == "huangzhi" {
        let timestamp: i64 = chrono::Utc::now().timestamp() + (*SERVER).max_age;
        let random_num = rand::thread_rng().gen_range(100000000, 999999999);
        let id_str = format!("username={}&timestamp={}&random_num={}", login_params.username, timestamp, random_num);
        id.remember(id_str.to_owned()); // <- remember identity
        HttpResponse::Ok().json(PrResponse::ok(login_params))
    } else {
        let err_res  = error("username or password is wrong".to_owned());
        HttpResponse::Ok().json(err_res)
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();                      // <- remove identity
    HttpResponse::Ok().finish()
}

pub fn verify() -> HttpResponse {
    let mut hasher = Sha256::new();
    let timestamp: i64 = chrono::Utc::now().timestamp();
    let random_num = rand::thread_rng().gen_range(0, 1000000000);
    let verify_code = rand::thread_rng().gen_range(1000, 9999).to_string();
    let str = format!("appkey={}&random={}&time={}&mobile={}",
                      (*VERIFY).appkey,
                      random_num,
                      timestamp,
                      "18870581083"
    );
    println!("str: {}", str);
    hasher.input(str.as_bytes());
    let result = hasher.result();
    let sig = hex::encode(&result);
    println!("sig: {}", sig);
    let verify = (*VERIFY).clone();
    let msg_box = MsgObj {
        ext: "".to_string(),
        extend: "".to_string(),
        params: [verify_code, verify.minutes],
        sig,
        sign: verify.sign,
        tel: Tel {
            mobile: "18870581083".to_string(),
            nationcode: "86".to_string(),
        },
        time: timestamp,
        tpl_id: verify.tpl_id,
    };

    let client = reqwest::Client::new();
    let url_str = format!("https://yun.tim.qq.com/v5/tlssmssvr/sendsms?sdkappid={}&random={}", verify.sdkappid, random_num);
    client
        .post(&url_str)
        .json(&msg_box)
        .send()
        .and_then(move |mut response| {
            response.json()
                .map(|sms_res: SmsRes| {
                    info!("{:#?}", sms_res);
                    HttpResponse::Ok().json(sms_res)
                })
        }).unwrap()
}

pub fn welcome(_id: Identity) -> HttpResponse {
    let arr = array!["a", "b", "c"];
    HttpResponse::Ok().content_type("application/json").body(ok_json(arr))
}

pub fn with_param(req: HttpRequest, path: web::Path<(String, String)>) -> HttpResponse {
    println!("{:?}", req);
    let query_str = req.query_string();
    println!("query_str: {}", query_str);
    let qs = QString::from(query_str);
    let phone = qs.get("phone").unwrap();
    println!("path: {:#?}", path);
    HttpResponse::Ok().content_type("text/plain").body(format!("Hello {}!You phone is {}, your nick is {}", path.0, phone, path.1))
}