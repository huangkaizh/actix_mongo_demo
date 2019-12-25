use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use actix_identity::{RequestIdentity, IdentityItem};
use qstring::QString;
use rand::Rng;
use actix_web::HttpMessage;
use crate::common::settings::{SERVER};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth {
    max_age: i64
}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            max_age: (*SERVER).max_age
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
        ok(AuthMiddleware { service, max_age: self.max_age })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    max_age: i64
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
        println!("identity: {:#?}", identity);
        if let Some(id_str) = identity {
            let qs = QString::from(id_str.as_str());
            let username = qs.get("username").unwrap();
            let timestamp_str = qs.get("timestamp").unwrap();
            let timestamp = timestamp_str.to_string().parse::<i64>().unwrap();
            let timestamp_now: i64 = chrono::Utc::now().timestamp();
            if timestamp > timestamp_now {
                {
                    let timestamp_new: i64 = timestamp_now + self.max_age;
                    let random_num = rand::thread_rng().gen_range(100000000, 999999999);
                    let id_str_new = format!("username={}&timestamp={}&random_num={}", username, timestamp_new, random_num);
                    req.extensions_mut()
                        .insert(IdentityItem { id: Some(id_str_new), changed: true });
                }
                Box::new(self.service.call(req).and_then(|res| {
                    println!("Hi from auth response");
                    Ok(res)
                }))
            } else {
                Box::new(futures::future::err(actix_web::error::ErrorUnauthorized("timeout")))
            }
        } else {
            Box::new(futures::future::err(actix_web::error::ErrorUnauthorized("unauthorized")))
        }
    }
}