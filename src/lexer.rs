use crate::{error::LexerError, token::Token};
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

    fn lex_string(&mut self) -> Result<String, LexerError> {
        let mut string_value = String::new();
        while let Some(value) = self.token_iter.peek() {
            match *value {
                '\\' => {
                    self.advance();
                    match self.advance() {
                        Some('n') => string_value.push('\n'),
                        Some('r') => string_value.push('\r'),
                        Some('t') => string_value.push('\t'),
                        Some('\\') => string_value.push('\\'),
                        Some('"') => {
                            string_value.push('\"');
                        }
                        ch => return Err(LexerError::InvalidEscapeSquence(ch.unwrap_or(' '))),
                    }
                }
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
        Ok(string_value)
    }

    fn lex_number(&mut self) -> Result<String, LexerError> {
        let mut number_value = String::new();
        while let Some(value) = self.token_iter.peek() {
            match *value {
                '0'..='9' | '-' | '.' | 'e' | 'E' => {
                    number_value.push(*value);
                    self.advance();
                }
                _ => return Ok(number_value),
            }
        }
        Ok(number_value)
    }

    fn lex_boolean(&mut self) -> Result<bool, LexerError> {
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
                ch => return Err(LexerError::UnexpectedCharacter(ch)),
            }
            if boolean_value.as_str().contains("false") {
                return Ok(false);
            } else if boolean_value.as_str().contains("true") {
                return Ok(true);
            } else {
                return Err(LexerError::InvalidValue(boolean_value));
            }
        }
        return Err(LexerError::InvalidValue(boolean_value));
    }

    fn lex_null(&mut self) -> Result<Token, LexerError> {
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
            return Ok(Token::ValueNil);
        } else {
            return Err(LexerError::InvalidValue(null_value));
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
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
                    let string_value = self.lex_string()?;
                    tokens.push(Token::ValueString(string_value));
                }
                '0'..='9' | '-' => {
                    let number_value = self.lex_number()?;
                    tokens.push(Token::ValueNumber(number_value.parse().unwrap()));
                }
                't' | 'f' => {
                    let boolean_value = self.lex_boolean()?;
                    tokens.push(Token::ValueBoolean(boolean_value));
                }
                'n' => {
                    let null_value = self.lex_null()?;
                    tokens.push(null_value);
                }
                ' ' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                }
                val => {
                    return Err(LexerError::UnexpectedCharacter(*val));
                }
            }
        }
        Ok(tokens)
    }
}
