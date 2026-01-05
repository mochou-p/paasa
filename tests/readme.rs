// paasa/tests/readme.rs

#[cfg(feature = "rust")]
mod project_readme {
    #[test]
    fn example() {
        use paasa::{parse, rust::Token::{self, *}};

        let tokens      = parse::<Token>("fn hey() {}");
        let expectation = vec![Fn, FnName, ParenStart, ParenEnd, ScopeStart, ScopeEnd];

        assert_eq!(tokens, Ok(expectation));
    }
}
