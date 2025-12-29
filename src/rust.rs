// paasa/src/rust.rs

#[cfg(not(feature = "rust"))]
compile_error!("wait what");

use super::{ParseSettings, ParseResult, TokenTrait, TokenError, TokenResult, NextTokenResult};


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

impl TokenTrait for Token {
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

// TODO: refactor to keep it scaleable, for example check for things like brackets before the big match.
//       also, check for invalid keywords. for example Let after Let is invalid, as it expects a VarName
fn tokenise_word<'a>(last_token: Token, word: &'a str) -> TokenResult<'a, Token> {
    use {Token::*, TokenError::*};

    // TODO: thoroughly check all whitespace handling
    if word.chars().all(|ch| ch == '\n') {
        println!("\x1b[33m------+- newlines from tokenise_word\x1b[0m");
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

fn next_token<'a>(input: &'a str, start: &mut usize, last_token: Token, last_non_comment_token: Token) -> NextTokenResult<'a, Token> {
    if *start == input.len() {
        return Ok(None);
    }

    if last_token == Token::SlashComment {
        let Some(i) = input[*start..].find('\n') else {
            return Ok(None);
        };

        *start += i;
    }

    println!("\x1b[34m{}\x1b[91;1m^\x1b[34;7m{}\x1b[0m\x1b[91;1m$\x1b[0m", &input[..*start], &input[*start..]);
    let start_char = input.chars().nth(*start).unwrap();

    let end = {
        if let Some(i) = input[*start+1..].find(end_of_word_searcher(start_char)) {
            *start + i + 1
        } else {
            input.len()
        }
    };
    let word = &input[*start..end];
    println!("word  = `{word}`");

    let token_result = tokenise_word(last_non_comment_token, word);

    let token = match token_result {
        Ok (token)       => token,
        Err(token_error) => {
            return Err(token_error);
        }
    };

    println!("token = `{token:?}`");

    println!();

    *start = end;
    Ok(Some(token))
}

fn _parse<'a>(input: &'a str, settings: ParseSettings) -> ParseResult<'a, Token> {
    let mut tokens                 = vec![];
    let mut start                  = 0;
    let mut last_start             = 0;
    let mut last_token             = Token::default();
    let mut last_non_comment_token = Token::default();

    loop {
        match next_token(input, &mut start, last_token, last_non_comment_token) {
            Ok(Some(token)) => {
                assert_ne!(start, last_start, "(dev error) infinite logic loop detected");

                last_start = start;

                let is_whitespace = token.is_whitespace();
                let is_newline    = token.is_newline();
                let is_comment    = token.is_comment();

                if
                    ( is_whitespace && settings.include_whitespaces)
                    ||
                    ( is_comment    && settings.include_comments)
                    ||
                    ( is_newline    && settings.include_newlines)
                    ||
                    !token.is_special()
                {
                    tokens.push(token);
                }

                if is_newline {
                    last_token = last_non_comment_token;
                    continue;
                }
                if !is_whitespace {
                    if !is_comment {
                        last_non_comment_token = token;
                    }
                    last_token = token;
                }
            },
            Ok(None) => {
                return Ok(tokens);
            },
            Err(token_error) => {
                return Err((tokens, token_error));
            }
        }
    }
}

pub fn parse<'a>(input: &'a str) -> ParseResult<'a, Token> {
    _parse(input, ParseSettings::default())
}

pub fn parse_with_settings<'a>(input: &'a str, settings: ParseSettings) -> ParseResult<'a, Token> {
    _parse(input, settings)
}

