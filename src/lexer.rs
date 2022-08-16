use core::str::Chars;
use core::iter::Peekable;
use std::fmt;

use crate::parse_result::{ParseResult, ParseError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(i64),
    LeftParen,
    RightParen,
    Operator(char),
    EOL
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number(value) => write!(f, "Number({0})", value),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Operator(c) => write!(f, "Operator('{0}')", c),
            EOL => write!(f, "EOL")
        }
    }
}

use Token::*;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    current: Token,
}

impl<'a> Lexer<'a> {

    /// Returns the token currently at the front of the stream
    pub fn current(&self) -> Token {
        self.current
    }

    /// Returns the current token and then advances beyond it
    pub fn take(&mut self) -> Token {
        let return_value = self.current;
        self.advance();
        return_value
    }

    /// Advances one token ahead in the stream
    pub fn advance(&mut self) {
        // Skip whitespace
        while let Some(' ' | '\t') = self.chars.peek() {
            self.chars.next();
        };
        self.current = match self.chars.next() {
            Some(digit) if digit.is_ascii_digit() => {
                let mut value: i64 = digit.to_digit(10).unwrap().into();
                loop {
                    match self.chars.peek().and_then(|c| c.to_digit(10)) {
                        Some(digit) => {
                            self.chars.next();
                            value = value * 10 + i64::from(digit);
                        },
                        _ => break
                    }
                }
                Number(value)
            },
            Some('(') => LeftParen,
            Some(')') => RightParen,
            Some(op) => Operator(op),
            None => EOL
        };
    }

    /// Consumes the current token, but also checks that its type was as expected
    pub fn consume(&mut self, token: Token, expected: &'static str) -> ParseResult<()> {
        if self.current != token {
            Err(ParseError::expected(expected))
        } else {
            self.advance();
            Ok(())
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a mut String) -> Self {
        let mut new = Self {
            chars: text.chars().peekable(),
            current: EOL
        };
        new.advance();
        new
    }
}
