use actix_web::{Error, HttpResponse};
use futures::future::{ok, Future};

fn index() -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("text/html").body("Hello!"),
    ))
}

fn index2() -> Box<dyn Future<Item = &'static str, Error = Error>> {
    Box::new(ok::<_, Error>("Welcome!"))
}

fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/async", web::to_async(index))
            .route("/", web::to_async(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}