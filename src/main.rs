use lexer::Lexer;
use token::Token;

mod lexer;
mod token;
fn main() {
    let token_string_value = Token::ValueString("Bipul Lamsal".to_string());
    println!("{:?}", token_string_value);
}
#[cfg(test)]
mod tests {
    use self::lexer::Lexer;
    use crate::token::Token;

    use super::*;
    #[test]
    pub fn valid_empty_delimitter_check() {
        let test_json = "{}";
        let mut lexer = Lexer::new(test_json);
        assert_eq!(lexer.lex(), vec!(Token::BeginObject, Token::EndObject))
    }

    #[test]
    pub fn invalid_key_check() {
        let test_json = "{\"name\"}";
        println!("{}", test_json);
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec![
                Token::BeginObject,
                Token::ValueString("name".to_string()),
                Token::EndObject
            ]
        )
    }

    #[test]
    fn valid_one_key_one_value_string() {
        let test_json = r#"{"name" : "Bipul"}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec!(
                Token::BeginObject,
                Token::ValueString("name".to_string()),
                Token::ValueSeperator,
                Token::ValueString("Bipul".to_string()),
                Token::EndObject
            )
        )
    }
}
