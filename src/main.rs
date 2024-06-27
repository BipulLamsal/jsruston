fn main() {}
#[cfg(test)]
mod tests {
    use jsruston::lexer::Lexer;
    use jsruston::token::Token;

    #[test]
    pub fn valid_empty_delimitter_check() {
        let test_json = r#"{}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(lexer.lex(), vec!(Token::BeginObject, Token::EndObject))
    }

    #[test]
    pub fn invalid_key_check() {
        let test_json = r#"{"name"}"#;

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
                Token::NameSeperator,
                Token::ValueString("Bipul".to_string()),
                Token::EndObject
            )
        )
    }

    #[test]
    fn valid_one_key_one_value_number() {
        let test_json = r#"{"number" : 100}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec!(
                Token::BeginObject,
                Token::ValueString("number".to_string()),
                Token::NameSeperator,
                Token::ValueNumber(100 as f64),
                Token::EndObject
            )
        )
    }

    #[test]
    fn valid_one_key_one_value_boolean_false() {
        let test_json = r#"{"boolean" : false}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec!(
                Token::BeginObject,
                Token::ValueString("boolean".to_string()),
                Token::NameSeperator,
                Token::ValueBoolean(false),
                Token::EndObject
            )
        )
    }

    #[test]
    fn valid_one_key_one_value_boolean_true() {
        let test_json = r#"{"boolean" : true}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec!(
                Token::BeginObject,
                Token::ValueString("boolean".to_string()),
                Token::NameSeperator,
                Token::ValueBoolean(true),
                Token::EndObject
            )
        )
    }

    #[test]
    fn valid_one_key_one_value_null() {
        let test_json = r#"{"value" : null}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec!(
                Token::BeginObject,
                Token::ValueString("value".to_string()),
                Token::NameSeperator,
                Token::ValueNil,
                Token::EndObject
            )
        )
    }

    #[test]
    fn valid_one_key_array_value() {
        let test_json = r#"{"foo":[1,2,{"bar":2}]}"#;
        let mut lexer = Lexer::new(test_json);
        assert_eq!(
            lexer.lex(),
            vec!(
                Token::BeginObject,
                Token::ValueString("foo".to_string()),
                Token::NameSeperator,
                Token::BeginArray,
                Token::ValueNumber(1 as f64),
                Token::ValueSeperator,
                Token::ValueNumber(2 as f64),
                Token::ValueSeperator,
                Token::BeginObject,
                Token::ValueString("bar".to_string()),
                Token::NameSeperator,
                Token::ValueNumber(2 as f64),
                Token::EndObject,
                Token::EndArray,
                Token::EndObject,
            )
        )
    }
}
