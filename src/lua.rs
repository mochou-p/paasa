// paasa/src/lua.rs

#[cfg(not(feature = "lua"))]
compile_error!("unexpected import error, feature mismatch");


#[derive(Clone, Copy, Default, Debug, PartialEq, Hash)]
pub enum Token {
    #[default]
    Nothing,

    Spaces,
    Tabs,
    Newlines,

    DashComment,
    BracketCommentStart,
    BracketCommentEnd,

    Function,
    FunctionName,
    ParenStart,
    ParenEnd,
    Return,
    End
}

impl super::TokenTrait for Token {
    fn is_whitespace(&self) -> bool {
        use Token::*;

        matches!(self, Spaces | Tabs)
    }

    fn is_newline(&self) -> bool {
        *self == Token::Newlines
    }

    fn is_comment(&self) -> bool {
        use Token::*;

        matches!(self, DashComment | BracketCommentStart | BracketCommentEnd)
    }

    fn is_inline_comment(&self) -> bool {
        *self == Token::DashComment
    }

    fn end_of_word_searcher(start_char: char) -> impl Fn(char) -> bool {
        match start_char {
            ' '        => | ch| ch != ' ',
            '\n'       => | ch| ch != '\n',
            '-'        => | ch| ch != '-',
            '(' | ')'  => |_ch| true,
            _          => | ch| matches!(ch, ' ' | '(' | '\n')
        }
    }

    fn tokenise_word<'a>(last_token: Self, word: &'a str) -> super::TokenResult<'a, Self> {
        use {Token::*, super::TokenError::*};

        // TODO: move whitespace handling to lib.rs

        if word.chars().all(|ch| ch == '\n') {
            return Ok(Newlines);
        }

        if word.chars().all(|ch| ch == ' ') {
            return Ok(Spaces);
        }

        match last_token {
            Nothing => {
                match word {
                    "--"       =>  Ok(DashComment),
                    "function" =>  Ok(Function),
                    _          => Err(UnexpectedToken(last_token, word))
                }
            },
            Function => Ok(FunctionName),
            FunctionName => {
                match word {
                    "(" =>  Ok(ParenStart),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            ParenStart => {
                match word {
                    ")" =>  Ok(ParenEnd),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            ParenEnd => {
                match word {
                    "return" =>  Ok(Return),
                    _        => Err(UnexpectedToken(last_token, word))
                }
            },
            Return => {
                match word {
                    "end" =>  Ok(End),
                    _     => Err(UnexpectedToken(last_token, word))
                }
            },
            _ => Err(ImplementationMissing(last_token))
        }
    }
}

