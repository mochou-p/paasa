// paasa/src/lib.rs

#[cfg(not(any(feature = "rust")))]
compile_error!("atleast one feature needs to be enabled. available features: [rust]");

#[cfg(feature = "rust")]
pub mod rust;

use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;


type     TokenResult<'a, T> = Result<       T,           TokenError<'a, T> >;
type NextTokenResult<'a, T> = Result<Option<T>,          TokenError<'a, T> >;
type     ParseResult<'a, T> = Result<   Vec<T>, (Vec<T>, TokenError<'a, T>)>;

pub trait TokenTrait: Clone + Copy + Default + Debug + PartialEq + Hash {
    fn is_whitespace(&self) -> bool;
    fn is_newline   (&self) -> bool;
    fn is_comment   (&self) -> bool;

    fn is_special(&self) -> bool {
        self.is_whitespace() || self.is_newline() || self.is_comment()
    }

    // NOTE: these could be somehow private i guess?
    fn     is_slash_comment(&self)                           -> bool;
    fn    tokenise_word<'a>(last_token: Self, word: &'a str) -> TokenResult<'a, Self>;
    fn end_of_word_searcher(start_char: char)                -> impl Fn(char) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum TokenError<'a, T: TokenTrait> {
    UnexpectedToken(T, &'a str),
    ImplementationMissing(T)
}

impl<T: TokenTrait> Display for TokenError<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnexpectedToken(before_token, after_word) => {
                    format!("there is no rule for \"{after_word}\" following `{before_token:?}`")
                },
                Self::ImplementationMissing(token) => {
                    format!("implementation missing for `{token:?}`")
                }
            }
        )
    }
}

impl<T: TokenTrait> Error for TokenError<'_, T> {}

// NOTE: maybe a mask later
#[derive(Clone, Copy, Default)]
pub struct ParseSettings {
    pub include_whitespaces: bool,
    pub include_newlines:    bool,
    pub include_comments:    bool
}

fn next_token<'a, T: TokenTrait>(input: &'a str, start: &mut usize, last_token: T, last_non_comment_token: T) -> NextTokenResult<'a, T> {
    if *start == input.len() {
        return Ok(None);
    }

    if last_token.is_slash_comment() {
        let Some(i) = input[*start..].find('\n') else {
            return Ok(None);
        };

        *start += i;
    }

    #[cfg(test)]
    println!("\x1b[34m{}\x1b[91;1m^\x1b[34;7m{}\x1b[0m\x1b[91;1m$\x1b[0m", &input[..*start], &input[*start..]);

    let start_char = input.chars().nth(*start).unwrap();

    let end = {
        if let Some(i) = input[*start+1..].find(T::end_of_word_searcher(start_char)) {
            *start + i + 1
        } else {
            input.len()
        }
    };
    let word = &input[*start..end];

    #[cfg(test)]
    println!("word  = `{word}`");

    let token_result = T::tokenise_word(last_non_comment_token, word);

    let token = match token_result {
        Ok (token)       => token,
        Err(token_error) => {
            return Err(token_error);
        }
    };

    #[cfg(test)]
    println!("token = `{token:?}`\n");

    *start = end;
    Ok(Some(token))
}

fn _parse<'a, T: TokenTrait>(input: &'a str, settings: ParseSettings) -> ParseResult<'a, T> {
    let mut tokens                 = vec![];
    let mut start                  = 0;
    let mut last_start             = 0;
    let mut last_token             = T::default();
    let mut last_non_comment_token = T::default();

    loop {
        match next_token::<T>(input, &mut start, last_token, last_non_comment_token) {
            Ok(Some(token)) => {
                assert_ne!(start, last_start, "(dev error) infinite logic loop detected");

                last_start = start;

                let is_whitespace = token.is_whitespace();
                let is_newline    = token.is_newline();
                let is_comment    = token.is_comment();

                if
                    !token.is_special()
                    ||
                    ( is_whitespace && settings.include_whitespaces)
                    ||
                    ( is_comment    && settings.include_comments)
                    ||
                    ( is_newline    && settings.include_newlines)
                {
                    tokens.push(token);
                }

                if is_newline {
                    // TODO: refactor this when star comments are implemented?
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

pub fn parse<'a, T: TokenTrait>(input: &'a str) -> ParseResult<'a, T> {
    _parse::<T>(input, ParseSettings::default())
}

pub fn parse_with_settings<'a, T: TokenTrait>(input: &'a str, settings: ParseSettings) -> ParseResult<'a, T> {
    _parse::<T>(input, settings)
}

