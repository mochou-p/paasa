// paasa/tests/rust.rs

use paasa::TokenTrait;


fn remove_special<const N: usize, T: TokenTrait>(tokens: [T; N]) -> Vec<T> {
    tokens
        .into_iter()
        .filter(|token| !token.is_special())
        .collect()
}

#[cfg(feature = "rust")]
mod rust_tests {
    use paasa::ParseSettings;
    use paasa::rust::{parse, parse_with_settings, Token::*};


    const SETTINGS: ParseSettings = ParseSettings {
        include_whitespaces: true,
        include_newlines:    true,
        include_comments:    true
    };

    #[test]
    fn test_rust_parse() {
        assert_eq!(
            parse(include_str!("files/rust_input.rs")),
            super::remove_special(include!("files/rust_output.rs"))
        );
    }

    #[test]
    fn test_rust_parse_with_settings() {
        assert_eq!(
            parse_with_settings(include_str!("files/rust_input.rs"), SETTINGS),
            include!("files/rust_output.rs")
        );
    }
}

