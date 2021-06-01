use serde::Deserialize;

pub mod github;
pub mod google;
pub mod twitter;

#[derive(Deserialize)]
pub struct Query {
    pub cmd: String,
}

pub fn get_command(s: &str) -> &str {
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
