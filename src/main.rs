use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;

use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct MyObj {
    name: &'static str,
    bytes: &'static [u8]
}

impl MyObj {

    pub fn new(name: &'static str) -> Self {
        MyObj{name: name, bytes: &[]}
    }

    pub fn bytes(&mut self) -> &'static [u8] {
        let self_serialized = serde_json::to_string(self);
        match self_serialized {
            Ok(_json) =>
                self.bytes = b"{\"ok\": 1}" ,
            Err(_e) => 
                self.bytes = b"{\"nok\": 0}"
        };
        self.bytes
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}



fn index() -> HttpResponse {
    let mut my_user = MyObj::new("stefan");
    let body = once::<Bytes, Error>(Ok(Bytes::from_static( my_user.bytes() )));

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