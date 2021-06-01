use actix_web::{App, HttpServer};

mod controllers;
use controllers::index::*;
use controllers::search::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(search))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
