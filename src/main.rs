use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Query {
    cmd: String,
}

#[get("/search")]
async fn search(query: web::Query<Query>) -> impl Responder {
    HttpResponse::Ok().body(&query.cmd)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(search))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
