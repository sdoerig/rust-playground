use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use futures::future::{ok, Future};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index() -> impl Responder {
    MyObj { name: "user" }
}

fn index2() -> Box<dyn Future<Item = &'static str, Error = Error>> {
    Box::new(ok::<_, Error>("Welcome!"))
}

fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(index))
            .route("/", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}