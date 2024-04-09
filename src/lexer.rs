use crate::token::Token;
use std::{iter, str::Chars};

#[derive(Clone)]
pub struct Lexer<'a> {
    token_iter: iter::Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        println!("{:?}", data);
        Lexer {
            token_iter: data.chars().peekable(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.token_iter.next()
    }

    fn lex_string(&mut self) -> String {
        let mut string_value = String::new();
        while let Some(value) = self.clone().token_iter.peek() {
            println!("{:?}", value);
            match *value {
                '\\' => match self.advance() {
                    Some('n') => string_value.push('\n'),
                    Some('r') => string_value.push('\r'),
                    Some('t') => string_value.push('\t'),
                    Some('\\') => string_value.push('\\'),
                    Some('"') => {
                        self.advance();
                        break;
                    }
                    _ => panic!("Invalid sequence character"),
                },
                val => string_value.push(val),
            }
            self.advance();
        }
        string_value
    }

    fn lex_number(&mut self) -> String {
        let mut number_value = String::new();
        while let Some(value) = self.token_iter.peek() {
            match *value {
                '0'..='9' | '-' | '.' | 'e' | 'E' => {
                    number_value.push(*value);
                    self.advance();
                }
                _ => return number_value,
            }
        }
        number_value
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.token_iter.peek() {
            match ch {
                '{' => {
                    tokens.push(Token::BeginObject);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::EndObject);
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::BeginArray);
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::EndArray);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::NameSeperator);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::ValueSeperator);
                    self.advance();
                }
                '"' => {
                    let string_value = self.lex_string();
                    tokens.push(Token::ValueString(string_value));
                }
                '0'..='9' | '-' => {
                    let number_value = self.lex_number();
                    tokens.push(Token::ValueNumber(number_value.parse().unwrap()));
                }
                _ => {}
            }
        }
        tokens
    }
}
