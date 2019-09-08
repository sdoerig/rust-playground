use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;

use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct MyObj {
    name: &'static str
}

impl MyObj {

    pub fn new(name: &'static str) -> Self {
        MyObj{name: name}
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

fn index() -> HttpResponse {
    let my_user = MyObj::new("stefan");
    let body = match my_user.to_json() {
        Ok(_json) => once::<Bytes, Error>(Ok(Bytes::from( _json.as_bytes() ))),
        Err(_e) => once::<Bytes, Error>(Ok(Bytes::from( "error".as_bytes() ))),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(body))
}

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/async", web::to_async(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}