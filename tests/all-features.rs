// paasa/tests/rust.rs

use paasa::{ParseSettings, TokenTrait};


const FULL_SETTINGS: ParseSettings = ParseSettings {
    include_whitespaces: true,
    include_newlines:    true,
    include_comments:    true
};

fn remove_special<const N: usize, T: TokenTrait>(tokens: [T; N]) -> Vec<T> {
    tokens
        .into_iter()
        .filter(|token| !token.is_special())
        .collect()
}

#[cfg(feature = "rust")]
mod rust_tests {
    use paasa::rust::{parse, parse_with_settings, Token::*};


    #[test]
    fn test_rust_parse() {
        assert_eq!(
            parse(include_str!("files/rust_input.rs")).unwrap(),
            super::remove_special(include!("files/rust_output.rs"))
        );
    }

    #[test]
    fn test_rust_parse_with_settings() {
        assert_eq!(
            parse_with_settings(include_str!("files/rust_input.rs"), super::FULL_SETTINGS).unwrap(),
            include!("files/rust_output.rs")
        );
    }
}

