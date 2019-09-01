use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

fn incr_counter(data: &web::Data<AppStateWithCounter>) {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
}

fn _index(data: web::Data<AppStateWithCounter>) -> String {
    incr_counter(&data);
    let counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

fn _hello(data: web::Data<AppStateWithCounter>) -> String {
    incr_counter(&data);
       
    format!("hello") // <- response with count
}


fn main() {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
    App::new()
        .register_data(counter.clone()) // <- register the created data
        .route("/", web::get().to(_index))
        .route("/hello", web::get().to(_hello))
        })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}