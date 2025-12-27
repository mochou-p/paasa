//

#[cfg(feature = "rust")]
mod rust_tests {
    use temp::rust::{parse, Token::*};

    #[test]
    fn rust_test_001() {
        assert_eq!(
            parse("pub fn test::<T>(&mut self, lol: &u8, lmao: Vec<Vec<T>>) {// :D :D\n}\nfn f() {}"),
            vec![
                /* pub fn test       */ Pub, Fn, FnName,
                /* ::<T>             */ DoubleColon, GenericStart, Type, GenericEnd,
                /* (                 */ ParenStart,
                /* &mut self,        */ Reference, Mut, SmallSelf, Comma,
                /* lol:  &u8         */ VarName, Colon, Reference, Type, Comma,
                /* lmao: Vec<Vec<T>> */ VarName, Colon, Type, GenericStart, Type, GenericStart, Type, GenericEnd, GenericEnd,
                /* )                 */ ParenEnd,
                /* {// :D :D\n}\n    */ ScopeStart, SlashComment, ScopeEnd,
                /* fn f() {}         */ Fn, FnName, ParenStart, ParenEnd, ScopeStart, ScopeEnd
            ]
        );
    }
}

