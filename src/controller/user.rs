use actix_identity::{Identity};
use crate::common::structs::{LoginParams, PrResponse, MsgObj, Tel, SmsRes};
use actix_web::{
    HttpResponse, web
};
use rand::Rng;
use crate::common::settings::{ DB, SERVER, VERIFY};
use sha2::{Sha256, Digest};

pub fn login(id: Identity, item: web::Json<LoginParams>) -> HttpResponse {
    let login_params = item.0;
    println!("loginParams: {:?}", login_params);
    info!("loginParams: {:?}", login_params);
    if login_params.username == "huangzhi" {
        let timestamp: i64 = chrono::Utc::now().timestamp() + (*SERVER).max_age;
        let random_num = rand::thread_rng().gen_range(100000000, 999999999);
        let id_str = format!("username={}&timestamp={}&random_num={}", login_params.username, timestamp, random_num);
        id.remember(id_str.to_owned()); // <- remember identity
        HttpResponse::Ok().json(login_params)
    } else {
        let err_res  = PrResponse::<String>::error("用户名或密码错误".to_owned());
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