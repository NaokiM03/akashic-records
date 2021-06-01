use actix_web::{get, http, web, HttpResponse, Responder};

mod commands;
use commands::{get_command, github::*, google::*, twitter::*, Query};

#[get("/search")]
async fn search(query: web::Query<Query>) -> impl Responder {
    let cmd_first_str = get_command(&query.cmd);
    let url = match cmd_first_str {
        "tw" => get_twitter_url(&query.cmd),
        "gh" => get_github_url(&query.cmd),
        _ => get_google_url(&query.cmd),
    };
    HttpResponse::TemporaryRedirect()
        .header(http::header::LOCATION, url)
        .finish()
}
