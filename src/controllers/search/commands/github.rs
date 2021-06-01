pub fn get_github_url(s: &str) -> String {
    let strs = s.split_whitespace().collect::<Vec<&str>>();

    let mut url = "https://github.com".to_string();

    if strs.len() == 1 {
    } else if strs.contains(&"--me") {
        url.push_str("/NaokiM03");
    } else {
        url.push_str(&format!("/{}", strs[1]));
    }

    url
}

#[cfg(test)]
mod test_get_github_url {
    use super::*;

    #[test]
    fn test_get_github_url() {
        let actual = get_github_url("gh");
        let expected = "https://github.com";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_github_url_with_me() {
        let actual = get_github_url("gh --me");
        let expected = "https://github.com/NaokiM03";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_github_url_with_keyword() {
        let actual = get_github_url("gh NaokiM03/akashic-records");
        let expected = "https://github.com/NaokiM03/akashic-records";
        assert_eq!(actual, expected);
    }
}
