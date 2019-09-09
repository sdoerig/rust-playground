use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;
use serde::Serialize;
use chrono::{DateTime, Utc};


#[derive(Debug,Serialize)]
pub struct MyUser<'a> {
    name: &'a str,
    created_at: String,
    likes: Vec<Likes<'a>>
}

#[derive(Debug,Serialize)]
pub struct Likes<'a> {
    name: &'a str,
    likeness: Likeness
}

#[derive(Debug, Serialize)]
pub enum Likeness {
    Very,
    Ok,
    Hmm
}


impl<'a> MyUser<'a> {
    pub fn new(name: &'a str) -> Self {
        let now: DateTime<Utc> = Utc::now();
        MyUser{
            name: name, 
            created_at: now.to_rfc2822(), 
            likes: Vec::new()}
    }

    pub fn likes(&mut self, like: &'a str, likeness: Likeness) {
        self.likes.push(Likes{name: like, likeness: likeness})
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

fn index() -> HttpResponse {
    let mut my_user = MyUser::new("stefan");
    my_user.likes("pizza", Likeness::Very);
    my_user.likes("salad", Likeness::Very);
    my_user.likes("C#", Likeness::Hmm);
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