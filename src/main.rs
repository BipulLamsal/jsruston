mod lexer;
mod token;
fn main() {}
#[cfg(test)]
mod tests {
    use self::lexer::Lexer;
    use crate::token::Token;

    use super::*;
    #[test]
    fn valid_empty_delimitter_check() {
        let test_json = "{}";
        let mut lexer = Lexer::new(test_json);
        assert_eq!(lexer.lex(), vec!(Token::BeginObject, Token::EndObject));
    }
}
