use actix_web::{Error, web, HttpResponse, Responder};
use bytes::Bytes;
use futures::stream::once;
use futures::stream::Once;
use serde::{Serialize,Deserialize};
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;


#[derive(Clone)]
pub struct AppState {
    pub count: Arc<AtomicUsize>,
}


#[derive(Debug,Serialize,Deserialize)]
pub struct MyUserDeserialized {
    userid: u32,
    friend: String,
}

#[derive(Debug,Serialize,Deserialize)]
struct MyUser<'a> {
    id: u32,
    name: &'a str,
    created_at: String,
    processed_requests: usize,
    likes: Vec<Likes<'a>>
}

#[derive(Debug,Serialize,Deserialize)]
struct Likes<'a> {
    name: &'a str,
    likeness: Likeness
}

#[derive(Debug, Serialize,Deserialize)]
enum Likeness {
    Very,
    Ok,
    Hmm
}


impl<'a> MyUser<'a> {
    fn new(id: u32, name: &'a str, processed_requests: usize) -> Self {
        let now: DateTime<Utc> = Utc::now();
        MyUser{
            id: id,
            name: name, 
            created_at: now.to_rfc2822(), 
            processed_requests: processed_requests,
            likes: Vec::new()}
    }
    fn from_deserialized(
            userid: u32, 
            friend: &'a String, 
            processed_requests: usize) -> Self {
        let now: DateTime<Utc> = Utc::now();
        MyUser{
            id: userid,
            name: friend, 
            created_at: now.to_rfc2822(), 
            processed_requests: processed_requests,
            likes: Vec::new()}
    }

    fn likes(&mut self, like: &'a str, likeness: Likeness) {
        self.likes.push(Likes{name: like, likeness: likeness})
    }

    fn to_json(&self) -> Once<Bytes, Error> {
        let body = match serde_json::to_string(self) {
            Ok(_json) => once::<Bytes, Error>(Ok(Bytes::from( _json.as_bytes() ))),
            Err(_e) => once::<Bytes, Error>(Ok(Bytes::from( "error".as_bytes() ))),
        };
        body
    }
}


pub(crate) fn index(data: web::Data<AppState>) -> HttpResponse {
    data.count.fetch_add(1, Ordering::Relaxed);
    let mut my_user = MyUser::new(1, "stefan", data.count.load(Ordering::Relaxed));
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
pub(crate) fn user(
        data: web::Data<AppState>, 
        info: web::Path<(u32, String)>) -> HttpResponse {
    data.count.fetch_add(1, Ordering::Relaxed);
    let my_user = MyUser::new(info.0, &info.1, data.count.load(Ordering::Relaxed));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(my_user.to_json()))
}

pub(crate) fn user_deserialize(
        data: web::Data<AppState>, 
        my_user: web::Path<MyUserDeserialized>) -> HttpResponse {
    data.count.fetch_add(1, Ordering::Relaxed);
    let my_user_resp = MyUser::from_deserialized(my_user.userid, &my_user.friend, data.count.load(Ordering::Relaxed));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(my_user_resp.to_json()))
}

/// deserialize `Info` from request's body, max payload size is 4kb
pub(crate) fn user_deserialize_json(
        data: web::Data<AppState>, 
        my_user: web::Json<MyUserDeserialized>) -> impl Responder {
    data.count.fetch_add(1, Ordering::Relaxed);
    let my_user_resp = MyUser::from_deserialized(
        my_user.userid, 
        &my_user.friend,
        data.count.load(Ordering::Relaxed));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(my_user_resp.to_json()))
}

