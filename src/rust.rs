//

#[cfg(not(feature = "rust"))]
compile_error!("wait what");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    StartOfLine,
    Whitespace,

    Comment,
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
    Semicolon
}

fn searcher(start_char: char) -> impl Fn(char) -> bool {
    match start_char {
        ' '             => | ch| ch != ' ',
        ':'             => | ch| ch != ':',
        '<' | '(' | '&' => |_ch| true,
        _               => | ch| matches!(ch, ' ' | ':' | '<' | '>' | '(' | ')' | ',')
    }
}

fn tokenise_word(last_token: Token, word: &'static str) -> Token {
    use Token::*;

    if word.chars().all(|ch| ch == ' ') {
        return Whitespace;
    }

    match last_token {
        StartOfLine => {
            match word {
                "pub"  => Pub,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Pub => {
            match word {
                "fn"   => Fn,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Fn => FnName,
        FnName => {
            match word {
                "::"   => DoubleColon,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        DoubleColon => {
            match word {
                "<"    => GenericStart,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        GenericStart => Type,
        Type => {
            match word {
                "<"    => GenericStart,
                ">"    => GenericEnd,
                ","    => Comma,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        GenericEnd => {
            match word {
                ">"    => GenericEnd,
                "("    => ParenStart,
                ")"    => ParenEnd,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        ParenStart => {
            match word {
                "&"    => Reference,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Reference => {
            match word {
                "mut"  => Mut,
                _      => Type
            }
        },
        Mut => {
            match word {
                "self" => SmallSelf,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        SmallSelf => {
            match word {
                ","    => Comma,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Comma => VarName,
        VarName => {
            match word {
                ":"    => Colon,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        Colon => {
            match word {
                "&"    => Reference,
                _      => Type
            }
        },
        ParenEnd => {
            match word {
                "{"    => ScopeStart,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        ScopeStart => {
            match word {
                "//"   => Comment,
                _      => { panic!("tokenise {last_token:?} _ arm `{word}`"); }
            }
        },
        _ => { panic!("tokenise _ arm `{last_token:?}`"); }
    }
}

fn next_token(line: &'static str, start: &mut usize, last_token: Token) -> Option<Token> {
    if last_token == Token::Comment {
        return None;
    }

    println!("{}\x1b[7m{}\x1b[0m", &line[..*start], &line[*start..]);
    let start_char     = line.chars().nth(*start).unwrap();

    let Some(mut end)  = line[*start+1..].find(searcher(start_char)) else { return None; };
    end               += 1;
    let word           = &line[*start..*start + end];
    println!("word     = `{word}`");

    let token          = tokenise_word(last_token, word);
    println!("token    = `{token:?}`");

    println!();

    *start += end;
    Some(token)
}

pub fn tokenise_line(line: &'static str) -> Vec<Token> {
    let mut tokens     = vec![];
    let mut start      = 0;
    let mut last_start = 0;
    let mut last_token = Token::StartOfLine;

    while let Some(token) = next_token(line, &mut start, last_token) {
        if start == last_start {
            panic!("infinite loop detected");
        }
        last_start = start;

        if !matches!(token, Token::Whitespace) {
            last_token = token;
            tokens.push(token);
        }
    }

    tokens
}

