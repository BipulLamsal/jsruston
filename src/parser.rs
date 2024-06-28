use crate::token::Token;
use core::panic;
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let iterator = tokens.into_iter().peekable();
        Parser { tokens: iterator }
    }
    fn current(&mut self) -> Option<&Token> {
        return self.tokens.peek();
    }
    fn consume(&mut self) -> Option<Token> {
        return self.tokens.next();
    }
    pub fn expect_token(&mut self, token: Token) {
        if self.tokens.peek() == Some(&token) {
            self.consume();
        } else {
            panic!(
                "Parser Error : Expected token {:?}, found {:?}",
                token,
                self.tokens.peek()
            );
        }
    }
    fn parse_value(&mut self) -> JsonValue {
        match self.current() {
            Some(Token::BeginObject) => self.parse_object(),
            Some(Token::BeginArray) => self.parse_array(),
            Some(Token::ValueString(val)) => JsonValue::String(val.to_string()),
            Some(Token::ValueNumber(val)) => JsonValue::Number(*val),
            Some(Token::ValueBoolean(val)) => JsonValue::Boolean(*val),
            Some(Token::ValueNil) => JsonValue::Null,

            _ => {
                panic!("Parser Error: Unexpected token")
            }
        }
    }

    fn parse_object(&mut self) -> JsonValue {
        let mut object = Vec::new();
        self.consume();
        while self.current() != Some(&Token::EndObject) {
            if let Some(Token::ValueString(key)) = self.consume() {
                // we need to consume if true else need to panic
                self.expect_token(Token::NameSeperator);
                let value = self.parse_value();
                object.push((key, value));
                self.consume();
                if self.current() == Some(&Token::ValueSeperator) {
                    self.consume();
                }
            }
        }
        JsonValue::Object(object)
    }

    fn parse_array(&mut self) -> JsonValue {
        let mut array = Vec::new();
        self.consume();
        while self.current() != Some(&Token::EndArray) {
            let value = self.parse_value();
            array.push(value);
            self.consume();
            if self.current() == Some(&Token::ValueSeperator) {
                self.consume();
            }
        }
        JsonValue::Array(array)
    }

    pub fn parse(&mut self) -> JsonValue {
        match self.current() {
            Some(Token::BeginObject) => self.parse_object(),
            Some(Token::BeginArray) => self.parse_array(),
            _ => panic!("Parser Error: JSON must start as a object or an array"),
        }
    }
}
