// paasa/src/rust.rs

#[cfg(not(feature = "rust"))]
compile_error!("wait what");

use super::{ParseSettings, TokenTrait};


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
fn tokenise_word(last_token: Token, word: &str) -> Token {
    use Token::*;

    // TODO: thoroughly check all whitespace handling
    if word.chars().all(|ch| ch == '\n') {
        println!("\x1b[33m------+- newlines from tokenise_word\x1b[0m");
        return Newlines;
    }
    if word.chars().all(|ch| ch == ' ') {
        return Spaces;
    }

    match last_token {
        Nothing => {
            match word {
                "//"     => SlashComment,
                "pub"    => Pub,
                "struct" => Struct,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Pub => {
            match word {
                "fn"     => Fn,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Fn => FnName,
        FnName => {
            match word {
                "<"      => GenericStart,
                "("      => ParenStart,
                "::"     => DoubleColon,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        GenericStart => Type,
        Type => {
            match word {
                "<"      => GenericStart,
                ">"      => GenericEnd,
                ","      => Comma,
                ";"      => Semicolon,
                "{"      => ScopeStart,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        GenericEnd => {
            match word {
                ">"      => GenericEnd,
                "("      => ParenStart,
                ")"      => ParenEnd,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        ParenStart => {
            match word {
                ")"      => ParenEnd,
                "&"      => Reference,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Reference => {
            match word {
                "mut"    => Mut,
                _        => Type
            }
        },
        Mut => {
            match word {
                "self"   => SmallSelf,
                _        => VarName, // TODO: this is wrong in arguments like before self, so make stuff more granular
            }
        },
        SmallSelf => {
            match word {
                ","      => Comma,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Comma => VarName,
        VarName => {
            match word {
                ":"      => Colon,
                "="      => Equals,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Colon => {
            match word {
                "&"      => Reference,
                _        => Type
            }
        },
        ParenEnd => {
            match word {
                "{"      => ScopeStart,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        ScopeStart => {
            match word {
                "//"     => SlashComment,
                "}"      => ScopeEnd,
                "pub"    => Pub,
                "let"    => Let,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        ScopeEnd => {
            match word {
                "}"      => ScopeEnd,
                "fn"     => Fn,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Struct => Type,
        Semicolon => {
            match word {
                "impl"   => Impl,
                "let"    => Let,
                "}"      => ScopeEnd,
                _        => VarName
            }
        },
        Impl => Type, // TODO: or Trait if followed with `for`
        Let => {
            match word {
                "mut"    => Mut,
                _        => VarName
            }
        },
        Equals => {
            match word {
                "false"  => Boolean,
                "true"   => Boolean,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Boolean => {
            match word {
                ";"      => Semicolon,
                _        => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        _ => { panic!("missing arm for `{last_token:?}`"); }
    }
}

fn next_token(input: &str, start: &mut usize, last_token: Token, last_non_comment_token: Token) -> Option<Token> {
    if *start == input.len() {
        return None;
    }

    if last_token == Token::SlashComment {
        let Some(i) = input[*start..].find('\n') else {
            return None;
        };

        *start += i;
    }

    println!("\x1b[34m{}\x1b[7m{}\x1b[0m", &input[..*start], &input[*start..]);
    let start_char = input.chars().nth(*start).unwrap();

    let end = {
        if let Some(i) = input[*start+1..].find(end_of_word_searcher(start_char)) {
            *start + i + 1
        } else {
            input.len()
        }
    };

    let word           = &input[*start..end];
    println!("word     = `{word}`");

    let token          = tokenise_word(last_non_comment_token, word);
    println!("token    = `{token:?}`");

    println!();

    *start = end;
    Some(token)
}

fn _parse(input: &str, settings: ParseSettings) -> Vec<Token> {
    let mut tokens                 = vec![];
    let mut start                  = 0;
    let mut last_start             = 0;
    let mut last_token             = Token::default();
    let mut last_non_comment_token = Token::default();

    while let Some(token) = next_token(input, &mut start, last_token, last_non_comment_token) {
        assert_ne!(start, last_start, "infinite logic loop detected");

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
            (!is_whitespace && !is_newline && !is_comment)
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
    }

    tokens
}

pub fn parse(input: &str) -> Vec<Token> {
    _parse(input, ParseSettings::default())
}

pub fn parse_with_settings(input: &str, settings: ParseSettings) -> Vec<Token> {
    _parse(input, settings)
}

