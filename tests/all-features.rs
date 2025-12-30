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
mod rust {
    use paasa::{parse, parse_with_settings};
    use paasa::rust::Token::{self, *};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse::<Token>(include_str!("files/rust_input.rs")).unwrap(),
            super::remove_special(include!("files/rust_output.rs"))
        );
    }

    #[test]
    fn test_parse_with_settings() {
        assert_eq!(
            parse_with_settings::<Token>(include_str!("files/rust_input.rs"), super::FULL_SETTINGS).unwrap(),
            include!("files/rust_output.rs")
        );
    }
}

mod project_readme {
    #[cfg(feature = "rust")]
    mod rust {
        #[test]
        fn example() {
            use paasa::{parse, rust::Token::{self, *}};

            let tokens      = parse::<Token>("fn hey() {}");
            let expectation = vec![Fn, FnName, ParenStart, ParenEnd, ScopeStart, ScopeEnd];

            assert_eq!(tokens, Ok(expectation));
        }
    }
}

