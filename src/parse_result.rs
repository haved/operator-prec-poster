use core::result;

use crate::lexer::Token;

pub struct ParseError {
    expected: &'static str
}

impl ParseError {
    pub fn expected(expected: &'static str) -> Self {
        Self { expected }
    }

    pub fn got(&self, got: Token) -> String {
        format!("Expected {0}, got {1}", self.expected, got)
    }
}

pub type ParseResult<T> = result::Result<T, ParseError>;
