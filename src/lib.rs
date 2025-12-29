// paasa/src/lib.rs

#[cfg(feature = "rust")]
pub mod rust;


pub trait TokenTrait {
    fn is_whitespace(&self) -> bool;
    fn is_newline   (&self) -> bool;
    fn is_comment   (&self) -> bool;

    fn is_special(&self) -> bool {
        self.is_whitespace() || self.is_newline() || self.is_comment()
    }
}

// NOTE: maybe a mask later
pub struct ParseSettings {
    pub include_whitespaces: bool,
    pub include_newlines:    bool,
    pub include_comments:    bool
}

impl Default for ParseSettings {
    fn default() -> Self {
        Self {
            include_whitespaces: false,
            include_newlines:    false,
            include_comments:    false
        }
    }
}

