use actix_web::{get, http, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header(
            http::header::LOCATION,
            "https://github.com/NaokiM03/akashic-records",
        )
        .finish()
}
