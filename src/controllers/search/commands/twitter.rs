pub fn get_twitter_url(s: &str) -> String {
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
