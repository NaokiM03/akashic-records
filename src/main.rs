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

fn get_command(s: &str) -> &str {
    s.split_whitespace()
        .collect::<Vec<&str>>()
        .first()
        .unwrap_or(&s)
}

#[cfg(test)]
mod test_get_command {
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

fn get_google_search_url(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<&str>>().join("+");
    format!("https://google.com/search?q={}", s)
}

#[cfg(test)]
mod test_get_google_search_url {
    use super::*;

    #[test]
    fn test_get_google_search_url() {
        let actual = get_google_search_url("foo");
        let expected = "https://google.com/search?q=foo";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_google_search_url_with_one_whitespace() {
        let actual = get_google_search_url("foo bar");
        let expected = "https://google.com/search?q=foo+bar";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_google_search_url_with_some_whitespace() {
        let actual = get_google_search_url("foo  bar");
        let expected = "https://google.com/search?q=foo+bar";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_google_search_url_with_three_words() {
        let actual = get_google_search_url("foo bar foobar");
        let expected = "https://google.com/search?q=foo+bar+foobar";
        assert_eq!(actual, expected);
    }
}

#[get("/search")]
async fn search(query: web::Query<Query>) -> impl Responder {
    let cmd_first_str = get_command(&query.cmd).to_owned();
    let url = match cmd_first_str {
        _ => get_google_search_url(&query.cmd),
    };
    HttpResponse::TemporaryRedirect()
        .header(http::header::LOCATION, url)
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(search))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
