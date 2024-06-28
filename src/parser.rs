use crate::{
    error::{ParsedJsonError, ParserError},
    lexer::Lexer,
    token::Token,
};
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

impl JsonValue {
    pub fn get(&self, key: &str) -> Result<&JsonValue, ParsedJsonError> {
        if let JsonValue::Object(ref obj) = *self {
            for (k, v) in obj {
                if k == key {
                    return Ok(v);
                }
            }
            Err(ParsedJsonError::KeyNotFound(key.to_string()))
        } else {
            Err(ParsedJsonError::UnexpectedType)
        }
    }

    pub fn get_string(&self) -> Result<&str, ParsedJsonError> {
        if let JsonValue::String(ref s) = *self {
            return Ok(s);
        }
        Err(ParsedJsonError::UnexpectedType)
    }

    pub fn get_number(&self) -> Result<f64, ParsedJsonError> {
        if let JsonValue::Number(n) = *self {
            return Ok(n);
        }
        Err(ParsedJsonError::UnexpectedType)
    }

    pub fn get_boolean(&self) -> Result<bool, ParsedJsonError> {
        if let JsonValue::Boolean(b) = *self {
            return Ok(b);
        }
        Err(ParsedJsonError::UnexpectedType)
    }

    pub fn is_null(&self) -> bool {
        matches!(*self, JsonValue::Null)
    }

    pub fn get_from_object(&self, key: &str) -> Result<&JsonValue, ParsedJsonError> {
        if let JsonValue::Object(ref obj) = *self {
            for (k, v) in obj {
                if k == key {
                    return Ok(v);
                }
            }
            Err(ParsedJsonError::KeyNotFound(key.to_string()))
        } else {
            Err(ParsedJsonError::UnexpectedType)
        }
    }

    pub fn len(&self) -> Result<usize, ParsedJsonError> {
        match *self {
            JsonValue::Object(ref obj) => Ok(obj.len()),
            JsonValue::Array(ref arr) => Ok(arr.len()),
            _ => Err(ParsedJsonError::InvalidMethodCall(format!(
                "len can be only perfomed with array and object type"
            ))),
        }
    }

    pub fn index(&self, index: usize) -> Result<&JsonValue, ParsedJsonError> {
        if let JsonValue::Array(ref array) = *self {
            return array
                .get(index)
                .ok_or(ParsedJsonError::IndexOutOfBounds(index));
        }
        Err(ParsedJsonError::UnexpectedType)
    }
}

impl std::ops::Index<&str> for JsonValue {
    type Output = JsonValue;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).expect("Key not found in JSON object")
    }
}

impl std::ops::Index<usize> for JsonValue {
    type Output = JsonValue;

    fn index(&self, index: usize) -> &Self::Output {
        self.index(index)
            .expect("Index out of bounds in JSON array")
    }
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
    pub fn expect_token(&mut self, token: Token) -> Result<(), ParserError> {
        if self.tokens.peek() == Some(&token) {
            self.consume();
            return Ok(());
        } else {
            return Err(ParserError::UnexpectedToken(token));
        }
    }
    fn parse_value(&mut self) -> Result<JsonValue, ParserError> {
        match self.current() {
            Some(Token::BeginObject) => Ok(self.parse_object()?),
            Some(Token::BeginArray) => Ok(self.parse_array()?),
            Some(Token::ValueString(val)) => Ok(JsonValue::String(val.to_string())),
            Some(Token::ValueNumber(val)) => Ok(JsonValue::Number(*val)),
            Some(Token::ValueBoolean(val)) => Ok(JsonValue::Boolean(*val)),
            Some(Token::ValueNil) => Ok(JsonValue::Null),
            ch => {
                return Err(ParserError::UnexpectedToken(ch.unwrap().clone()));
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParserError> {
        let mut object = Vec::new();
        self.consume();
        while self.current() != Some(&Token::EndObject) {
            if let Some(Token::ValueString(key)) = self.consume() {
                // we need to consume if true else need to panic
                self.expect_token(Token::NameSeperator)?;
                let value = self.parse_value()?;
                object.push((key, value));
                self.consume();
                if self.current() == Some(&Token::ValueSeperator) {
                    self.consume();
                }
            }
        }
        Ok(JsonValue::Object(object))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParserError> {
        let mut array = Vec::new();
        self.consume();
        while self.current() != Some(&Token::EndArray) {
            let value = self.parse_value()?;
            array.push(value);
            self.consume();
            if self.current() == Some(&Token::ValueSeperator) {
                self.consume();
            }
        }
        Ok(JsonValue::Array(array))
    }

    pub fn parse(&mut self) -> Result<JsonValue, ParserError> {
        match self.current() {
            Some(Token::BeginObject) => self.parse_object(),
            Some(Token::BeginArray) => self.parse_array(),
            ch => Err(ParserError::UnexpectedStart(ch.unwrap().clone())),
        }
    }
    pub fn parse_json(json: &str) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(json);
        let lex = lexer.lex()?;
        let parserd_json = Parser::new(lex).parse()?;
        Ok(parserd_json)
    }
}
