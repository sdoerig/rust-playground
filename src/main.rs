use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;
use futures::stream::Once;
use serde::{Serialize,Deserialize};
use chrono::{DateTime, Utc};
use actix_web::{web};


#[derive(Debug,Serialize,Deserialize)]
pub struct MyUser<'a> {
    id: u32,
    name: &'a str,
    created_at: String,
    likes: Vec<Likes<'a>>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Likes<'a> {
    name: &'a str,
    likeness: Likeness
}

#[derive(Debug, Serialize,Deserialize)]
pub enum Likeness {
    Very,
    Ok,
    Hmm
}


impl<'a> MyUser<'a> {
    pub fn new(id: u32, name: &'a str) -> Self {
        let now: DateTime<Utc> = Utc::now();
        MyUser{
            id: id,
            name: name, 
            created_at: now.to_rfc2822(), 
            likes: Vec::new()}
    }

    pub fn likes(&mut self, like: &'a str, likeness: Likeness) {
        self.likes.push(Likes{name: like, likeness: likeness})
    }

    pub fn to_json(&self) -> Once<Bytes, Error> {
        let body = match serde_json::to_string(self) {
            Ok(_json) => once::<Bytes, Error>(Ok(Bytes::from( _json.as_bytes() ))),
            Err(_e) => once::<Bytes, Error>(Ok(Bytes::from( "error".as_bytes() ))),
        };
        body
    }
}

fn index() -> HttpResponse {
    let mut my_user = MyUser::new(1, "stefan");
    my_user.likes("pizza", Likeness::Very);
    my_user.likes("salad", Likeness::Very);
    my_user.likes("C#", Likeness::Hmm);

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(my_user.to_json()))
}

/// extract path info from "/users/{userid}/{friend}" url
/// {userid} -  - deserializes to a u32
/// {friend} - deserializes to a String
fn user(info: web::Path<(u32, String)>) -> HttpResponse {
    let my_user = MyUser::new(info.0, &info.1);
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(my_user.to_json()))
}

fn user_deserialize(my_user: web::Path<MyUser>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(my_user.to_json()))
}



pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
        .route("/async", web::to_async(index))
        .route("/users/{userid}/{friend}", // <- define path parameters
            web::get().to(user))
        .route("/users_deserialize/{userid}/{friend}", // <- define path parameters
            web::get().to(user_deserialize))
        )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}