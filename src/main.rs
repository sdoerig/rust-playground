use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;

use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct MyObj<'a> {
    name: &'a str,
    likes: Vec<MyLikes<'a>>
}
#[derive(Debug,Serialize)]
pub struct MyLikes<'a> {
    name: &'a str,

}

impl<'a> MyObj<'a> {
    pub fn new(name: &'a str) -> Self {
        MyObj{name: name, likes: Vec::new()}
    }

    pub fn likes(&mut self, like: &'a str) {
        self.likes.push(MyLikes{name: like})
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

fn index() -> HttpResponse {
    let mut my_user = MyObj::new("stefan");
    my_user.likes("pizza");
    my_user.likes("salad");
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