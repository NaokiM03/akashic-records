use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header(
            http::header::LOCATION,
            "https://github.com/NaokiM03/akashic-records",
        )
        .finish()
}

#[derive(Deserialize)]
struct Query {
    cmd: String,
}

fn get_command(query_string: &str) -> &str {
    query_string
        .split_whitespace()
        .collect::<Vec<&str>>()
        .first()
        .unwrap_or(&query_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_with_no_whitespace() {
        let actual = get_command("foo");
        let expected = "foo";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_with_one_whitespace() {
        let actual = get_command("foo bar");
        let expected = "foo";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_with_some_whitespace() {
        let actual = get_command("foo  bar");
        let expected = "foo";
        assert_eq!(actual, expected);
    }
}

#[get("/search")]
async fn search(query: web::Query<Query>) -> impl Responder {
    let command = get_command(&query.cmd).to_owned();
    HttpResponse::Ok().body(command)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(search))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
