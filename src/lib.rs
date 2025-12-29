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
}

#[derive(Debug)]
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

