extern crate actix_web;
extern crate actix_service;

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use actix_identity::{RequestIdentity, IdentityItem};
use qstring::QString;
//use rand::prelude::random;
use rand::Rng;
use actix_web::HttpMessage;

const maxAge:i64 = 100000;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth {
    maxAge: i64
}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            maxAge
        }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Auth
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service, maxAge: self.maxAge })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    maxAge: i64
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from auth. You requested: {}", req.path());
        let identity = req.get_identity();

        if let Some(idStr) = identity {
            let mut qs = QString::from(idStr.as_str());
            let mut username = qs.get("username").unwrap();
            let mut timestampStr = qs.get("timestamp").unwrap();
            let mut timestamp = timestampStr.to_string().parse::<i64>().unwrap();
            let mut randomNum = qs.get("randomNum").unwrap();
            let mut timestamp_now: i64 = chrono::Utc::now().timestamp();
            if timestamp > timestamp_now {
                {
                    let mut timestamp_new: i64 = timestamp_now + self.maxAge;
                    let mut randomNum = rand::thread_rng().gen_range(100000000, 999999999);
                    let mut idStr_new = format!("username={}&timestamp={}&randomNum={}", username, timestamp_new, randomNum);
                    req.extensions_mut()
                        .insert(IdentityItem { id: Some(idStr_new), changed: true });
                }
                Box::new(self.service.call(req).and_then(|res| {
                    println!("Hi from auth response");
                    Ok(res)
                }))
            } else {
                Box::new(futures::future::err(actix_web::error::ErrorRequestTimeout("登录超时")))
            }
        } else {
            Box::new(futures::future::err(actix_web::error::ErrorUnauthorized("未登录")))
        }
    }
}