// paasa/src/rust.rs

#[cfg(not(feature = "rust"))]
compile_error!("unexpected import error, feature mismatch");


#[derive(Clone, Copy, Default, Debug, PartialEq, Hash)]
pub enum Token {
    #[default]
    Nothing,

    Spaces,
    Tabs,
    Newlines,

    SlashComment,
    StarCommentStart,
    StarCommentEnd,

    Pub,
    Fn,
    FnName,
    DoubleColon,
    GenericStart,
    Type,
    GenericEnd,
    ParenStart,
    Reference,
    Pointer,
    Mut,
    SmallSelf,
    Comma,
    VarName,
    Colon,
    BigSelf,
    ParenEnd,
    SingleArrow,
    ScopeStart,
    Let,
    Equals,
    Boolean,
    Number,
    Character,
    String,
    Semicolon,
    ScopeEnd,
    Struct,
    Impl
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

        matches!(self, SlashComment | StarCommentStart | StarCommentEnd)
    }

    fn is_inline_comment(&self) -> bool {
        *self == Token::SlashComment
    }

    fn end_of_word_searcher(start_char: char) -> impl Fn(char) -> bool {
        match start_char {
            ' '                               => | ch| ch != ' ',
            ':'                               => | ch| ch != ':',
            '\n'                              => | ch| ch != '\n',
            '/'                               => | ch| ch != '/',
            '<' | '(' | '&' | '{' | '}' | ';' => |_ch| true,
            _                                 => | ch| matches!(ch, ' ' | ':' | '<' | '>' | '(' | ')' | ',' | ';')
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
                    "//"     =>  Ok(SlashComment),
                    "pub"    =>  Ok(Pub),
                    "struct" =>  Ok(Struct),
                    "fn"     =>  Ok(Fn),
                    _        => Err(UnexpectedToken(last_token, word))
                }
            },
            Pub => {
                match word {
                    "fn" =>  Ok(Fn),
                    _    => Err(UnexpectedToken(last_token, word))
                }
            },
            Fn => Ok(FnName),
            FnName => {
                match word {
                    "<" =>  Ok(GenericStart),
                    "(" =>  Ok(ParenStart),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            GenericStart => Ok(Type),
            Type => {
                match word {
                    "<" =>  Ok(GenericStart),
                    ">" =>  Ok(GenericEnd),
                    "," =>  Ok(Comma),
                    ";" =>  Ok(Semicolon),
                    "{" =>  Ok(ScopeStart),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            GenericEnd => {
                match word {
                    ">" =>  Ok(GenericEnd),
                    "(" =>  Ok(ParenStart),
                    ")" =>  Ok(ParenEnd),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            ParenStart => {
                match word {
                    ")" =>  Ok(ParenEnd),
                    "&" =>  Ok(Reference),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            Reference => {
                match word {
                    "mut" => Ok(Mut),
                    _     => Ok(Type)
                }
            },
            Mut => {
                match word {
                    "self" => Ok(SmallSelf),
                    _      => Ok(VarName) // TODO: this is wrong in arguments like before self, so make stuff more granular
                }
            },
            SmallSelf => {
                match word {
                    "," =>  Ok(Comma),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            Comma => Ok(VarName),
            VarName => {
                match word {
                    ":" =>  Ok(Colon),
                    "=" =>  Ok(Equals),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            Colon => {
                match word {
                    "&" => Ok(Reference),
                    _   => Ok(Type)
                }
            },
            ParenEnd => {
                match word {
                    "{" =>  Ok(ScopeStart),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            ScopeStart => {
                match word {
                    "//"  =>  Ok(SlashComment),
                    "}"   =>  Ok(ScopeEnd),
                    "pub" =>  Ok(Pub),
                    "let" =>  Ok(Let),
                    _     => Err(UnexpectedToken(last_token, word))
                }
            },
            ScopeEnd => {
                match word {
                    "}"  =>  Ok(ScopeEnd),
                    "fn" =>  Ok(Fn),
                    _    => Err(UnexpectedToken(last_token, word))
                }
            },
            Struct => Ok(Type),
            Semicolon => {
                match word {
                    "impl" => Ok(Impl),
                    "let"  => Ok(Let),
                    "}"    => Ok(ScopeEnd),
                    _      => Ok(VarName)
                }
            },
            Impl => Ok(Type), // TODO: or Trait if followed with `for`
            Let => {
                match word {
                    "mut" => Ok(Mut),
                    _     => Ok(VarName)
                }
            },
            Equals => {
                match word {
                    "false" | "true" =>  Ok(Boolean),
                    _                => Err(UnexpectedToken(last_token, word))
                }
            },
            Boolean => {
                match word {
                    ";" =>  Ok(Semicolon),
                    _   => Err(UnexpectedToken(last_token, word))
                }
            },
            _ => Err(ImplementationMissing(last_token))
        }
    }
}

