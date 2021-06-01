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

fn get_google_url(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<&str>>().join("+");
    format!("https://google.com/search?q={}", s)
}

#[cfg(test)]
mod test_get_google_url {
    use super::*;

    #[test]
    fn test_get_google_url() {
        let actual = get_google_url("foo");
        let expected = "https://google.com/search?q=foo";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_google_url_with_one_whitespace() {
        let actual = get_google_url("foo bar");
        let expected = "https://google.com/search?q=foo+bar";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_google_url_with_some_whitespace() {
        let actual = get_google_url("foo  bar");
        let expected = "https://google.com/search?q=foo+bar";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_google_url_with_three_words() {
        let actual = get_google_url("foo bar foobar");
        let expected = "https://google.com/search?q=foo+bar+foobar";
        assert_eq!(actual, expected);
    }
}

fn get_twitter_url(s: &str) -> String {
    let strs = s.split_whitespace().collect::<Vec<&str>>();

    if strs.len() == 1 {
        "https://twitter.com/home".to_string()
    } else {
        let is_no_reply = strs.contains(&"--no-reply");
        let is_live = strs.contains(&"--now");

        // "%20OR%20%40i" == " OR @i"
        let mut url = format!("https://twitter.com/search?q={}%20OR%20%40i", strs[1]);
        if is_no_reply {
            // "%20-filter%3Areplies" == " -filter:replies"
            url.push_str("%20-filter%3Areplies");
        }
        if is_live {
            url.push_str("&f=live");
        }

        url
    }
}

#[cfg(test)]
mod test_get_twitter_url {
    use super::*;

    #[test]
    fn test_get_twitter_url() {
        let actual = get_twitter_url("tw");
        let expected = "https://twitter.com/home";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_twitter_url_with_keyword() {
        let actual = get_twitter_url("tw foo");
        let expected = "https://twitter.com/search?q=foo%20OR%20%40i";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_twitter_url_with_keyword_and_no_reply() {
        let actual = get_twitter_url("tw foo --no-reply");
        let expected = "https://twitter.com/search?q=foo%20OR%20%40i%20-filter%3Areplies";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_twitter_url_with_keyword_and_now() {
        let actual = get_twitter_url("tw foo --now");
        let expected = "https://twitter.com/search?q=foo%20OR%20%40i&f=live";
        assert_eq!(actual, expected);
    }
}

#[get("/search")]
async fn search(query: web::Query<Query>) -> impl Responder {
    let cmd_first_str = get_command(&query.cmd);
    let url = match cmd_first_str {
        "tw" => get_twitter_url(&query.cmd),
        _ => get_google_url(&query.cmd),
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
