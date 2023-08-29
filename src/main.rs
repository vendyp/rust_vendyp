#![feature(error_generic_member_access)]

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use qstring::QString;
use serde_derive::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/say")]
async fn say_hello(req: HttpRequest) -> String {
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let name = qs.get("name").unwrap();

    format!("Hello, {}!", name)
}

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[get("/info")]
async fn info(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.name)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(say_hello)
            .service(info)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
