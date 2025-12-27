//

#[cfg(feature = "rust")]
mod rust_tests {
    use temp::rust::{parse, parse_with_settings, ParseSettings, Token::*};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                concat!(
                    "pub fn test::<T>(&mut self, lol: &u8, lmao: Vec<Vec<T>>) {// :D :D\n",
                    "}\n",
                    "fn f() {}"
                )
            ),
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

    #[test]
    fn test_parse_with_settings() {
        assert_eq!(
            parse_with_settings(
                concat!(
                    "pub fn test::<T>(&mut self, lol: &u8, lmao: Vec<Vec<T>>) {// :D :D\n",
                    "}\n",
                    "fn f() {}"
                ),
                ParseSettings {
                    include_whitespaces: true,
                    include_newlines:    true,
                    include_comments:    true // TODO: false
                }
            ),
            vec![
                /* pub fn test       */ Pub, Spaces, Fn, Spaces, FnName,
                /* ::<T>             */ DoubleColon, GenericStart, Type, GenericEnd,
                /* (                 */ ParenStart,
                /* &mut self,        */ Reference, Mut, Spaces, SmallSelf, Comma, Spaces,
                /* lol:  &u8         */ VarName, Colon, Spaces, Reference, Type, Comma, Spaces,
                /* lmao: Vec<Vec<T>> */ VarName, Colon, Spaces, Type, GenericStart, Type, GenericStart, Type, GenericEnd, GenericEnd,
                /* )                 */ ParenEnd, Spaces,
                /* {// :D :D\n}\n    */ ScopeStart, SlashComment, Newlines, ScopeEnd, Newlines,
                /* fn f() {}         */ Fn, Spaces, FnName, ParenStart, ParenEnd, Spaces, ScopeStart, ScopeEnd
            ]
        );
    }
}

