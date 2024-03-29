use actix_web::{error, web, FromRequest, HttpResponse, guard};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use actix_web::middleware::Logger;
use env_logger;

mod user;
use crate::user::{index,user,user_deserialize, user_deserialize_json};
use crate::user::{AppState,MyUserDeserialized};

mod middleware;
use crate::middleware::SayHi;


pub fn main() {
    use actix_web::{App, HttpServer};

    let data = AppState {
        count: Arc::new(AtomicUsize::new(0)),
    };
    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .data(data.clone())
        .route("/async", web::to_async(index))
        .route("/users/{userid}/{friend}", // <- define path parameters
            web::get().to(user))
        .route("/users_deserialize/{userid}/{friend}", // <- define path parameters
            web::get().to(user_deserialize))
        .service(
            web::resource("/users")
                .data(
                    // change json extractor configuration
                    web::Json::<MyUserDeserialized>::configure(|cfg| {
                        cfg.limit(4096).error_handler(|err, _req| {
                            // <- create custom error response
                            error::InternalError::from_response(
                                err,
                                HttpResponse::Conflict().finish(),
                            )
                            .into()
                        })
                    }),
                )
                .guard(guard::Header("content-type", "application/json"))
                .route(web::post().to(user_deserialize_json))
        ).service(web::resource("/middleware").wrap(SayHi).route(web::get().to(index)))
        
        )
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}