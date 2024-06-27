use crate::token::Token;
use std::{iter, str::Chars};

#[derive(Clone)]
pub struct Lexer<'a> {
    token_iter: iter::Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Lexer {
            token_iter: data.chars().peekable(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.token_iter.next()
    }

    fn lex_string(&mut self) -> String {
        let mut string_value = String::new();
        while let Some(value) = self.token_iter.peek() {
            match *value {
                '\\' => match self.advance() {
                    Some('n') => string_value.push('\n'),
                    Some('r') => string_value.push('\r'),
                    Some('t') => string_value.push('\t'),
                    Some('\\') => string_value.push('\\'),
                    Some('"') => {
                        string_value.push('\"');
                    }
                    _ => panic!("Lexer Error: Invalid sequence character"),
                },
                '"' => {
                    self.advance();
                    break;
                }
                val => {
                    string_value.push(val);
                    self.advance();
                }
            }
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

    fn lex_boolean(&mut self) -> bool {
        let mut boolean_value = String::new();
        while let Some(value) = self.token_iter.peek() {
            match *value {
                't' => {
                    let mut counter = 1;
                    while counter <= 4 {
                        match self.advance() {
                            Some(value) => {
                                boolean_value.push(value);
                                counter += 1;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
                'f' => {
                    let mut counter = 1;
                    while counter <= 5 {
                        match self.advance() {
                            Some(value) => {
                                boolean_value.push(value);
                                counter += 1;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
                _ => panic!("Lexer Error: Unusual sequence of character"),
            }
            if boolean_value.as_str().contains("false") {
                return false;
            } else if boolean_value.as_str().contains("true") {
                return true;
            } else {
                panic!(
                    "Lexer Error: Unusual sequence of characters {}",
                    boolean_value
                );
            }
        }
        return false;
    }

    fn lex_null(&mut self) -> Token {
        let mut null_value = String::new();
        let mut counter = 1;
        while counter <= 4 {
            match self.advance() {
                Some(value) => {
                    null_value.push(value);
                    counter += 1;
                }
                _ => {
                    break;
                }
            }
        }
        if null_value.contains("null") {
            return Token::ValueNil;
        } else {
            panic!("Lexer Error: Unusual Squence of characters {}", null_value);
        }
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
                    self.advance();
                    let string_value = self.lex_string();
                    tokens.push(Token::ValueString(string_value));
                }
                '0'..='9' | '-' => {
                    let number_value = self.lex_number();
                    tokens.push(Token::ValueNumber(number_value.parse().unwrap()));
                }
                't' | 'f' => {
                    let boolean_value = self.lex_boolean();
                    tokens.push(Token::ValueBoolean(boolean_value));
                }
                'n' => {
                    let null_value = self.lex_null();
                    tokens.push(null_value);
                }
                ' ' => {
                    self.advance();
                }
                val => {
                    panic!("Lexer Error: Unusal sequence of character: {}", val);
                }
            }
        }
        tokens
    }
}
