pub fn get_google_url(s: &str) -> String {
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
