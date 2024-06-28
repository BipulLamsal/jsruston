use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char),
    InvalidEscapeSquence(char),
    InvalidValue(String),
    UnexpectedEndOfLine,
}
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexerError::UnexpectedCharacter(ch) => write!(f, "Unexpected character: {}", ch),
            LexerError::InvalidEscapeSquence(ch) => write!(f, "Invalid escape sequence: {}", ch),
            LexerError::InvalidValue(ref val) => write!(f, "Invalid value: {}", val),
            LexerError::UnexpectedEndOfLine => write!(f, "Unexpected end of line"),
        }
    }
}
impl Error for LexerError {}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(crate::token::Token),
    UnexpectedStart(crate::token::Token),
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserError::UnexpectedToken(ref token) => write!(f, "Unexpected token: {:?}", token),
            ParserError::UnexpectedStart(ref token) => write!(f, "Unexpected start: {:?}", token),
        }
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum JsonError {
    LexerError(LexerError),
    ParserError(ParserError),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonError::LexerError(ref err) => write!(f, "Lexer Error: {:?}", err),
            JsonError::ParserError(ref err) => write!(f, "Parser Error: {:?}", err),
        }
    }
}

impl Error for JsonError {}
impl From<LexerError> for JsonError {
    fn from(err: LexerError) -> JsonError {
        JsonError::LexerError(err)
    }
}

impl From<ParserError> for JsonError {
    fn from(err: ParserError) -> JsonError {
        JsonError::ParserError(err)
    }
}
