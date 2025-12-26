//

#[cfg(feature = "rust")]
mod rust_tests {
    use temp::rust::{tokenise_line, Token};

    #[test]
    fn rust_test_001() {
        let line        = "pub fn test::<T>(&mut self, lol: &u8, lmao: Vec<Vec<T>>) { // :D :O";
        let tokens      = tokenise_line(line);
        let expectation = {
            use Token::*;

            vec![
                /* pub fn test       */ Pub, Fn, FnName,
                /* ::<T>             */ DoubleColon, GenericStart, Type, GenericEnd,
                /* (                 */ ParenStart,
                /* &mut self,        */ Reference, Mut, SmallSelf, Comma,
                /* lol:  &u8         */ VarName, Colon, Reference, Type, Comma,
                /* lmao: Vec<Vec<T>> */ VarName, Colon, Type, GenericStart, Type, GenericStart, Type, GenericEnd, GenericEnd,
                /* )                 */ ParenEnd,
                /* { // :D :O        */ ScopeStart, Comment
            ]
        };

        assert_eq!(tokens, expectation);
    }
}

