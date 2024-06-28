use crate::lexer::Lexer;
use crate::parser::{JsonValue, Parser};
#[test]
pub fn parse_valid_empty_delimitter_check() {
    let test_json = r#"{}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    assert_eq!(parser.parse(), JsonValue::Object(Vec::new()))
}

#[test]
pub fn parse_valid_one_key_value_check() {
    let test_json = r#"{"name": "John"}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Object(vec![(
        "name".to_string(),
        JsonValue::String("John".to_string()),
    )]);
    assert_eq!(parser.parse(), expected_value);
}

#[test]
pub fn parse_valid_one_key_value_boolean_check() {
    let test_json = r#"{"name": true}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Object(vec![("name".to_string(), JsonValue::Boolean(true))]);
    assert_eq!(parser.parse(), expected_value);
}

#[test]
pub fn parse_valid_one_key_value_number_check() {
    let test_json = r#"{"name": 10}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Object(vec![("name".to_string(), JsonValue::Number(10.0))]);
    assert_eq!(parser.parse(), expected_value);
}

#[test]
pub fn parse_valid_one_key_value_null_check() {
    let test_json = r#"{"name": null}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Object(vec![("name".to_string(), JsonValue::Null)]);
    assert_eq!(parser.parse(), expected_value);
}
#[test]
pub fn parse_valid_empty_array_check() {
    let test_json = r#"[]"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    assert_eq!(parser.parse(), JsonValue::Array(Vec::new()));
}

#[test]
pub fn parse_valid_array_with_values_check() {
    let test_json = r#"[1, "two", true, null]"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::String("two".to_string()),
        JsonValue::Boolean(true),
        JsonValue::Null,
    ]);
    assert_eq!(parser.parse(), expected_value);
}

#[test]
pub fn parse_valid_nested_object_check() {
    let test_json = r#"{"name": {"first": "John", "last": "Doe"}}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Object(vec![(
        "name".to_string(),
        JsonValue::Object(vec![
            ("first".to_string(), JsonValue::String("John".to_string())),
            ("last".to_string(), JsonValue::String("Doe".to_string())),
        ]),
    )]);
    assert_eq!(parser.parse(), expected_value);
}

// failed
#[test]
pub fn parse_valid_array_of_objects_check() {
    let test_json = r#"[{"name": "John"}, {"name": "Jane"}]"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Array(vec![
        JsonValue::Object(vec![(
            "name".to_string(),
            JsonValue::String("John".to_string()),
        )]),
        JsonValue::Object(vec![(
            "name".to_string(),
            JsonValue::String("Jane".to_string()),
        )]),
    ]);
    assert_eq!(parser.parse(), expected_value);
}

#[test]
pub fn parse_valid_mixed_array_check() {
    let test_json = r#"[{"key": "value"}, [1, 2, 3], true]"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Array(vec![
        JsonValue::Object(vec![(
            "key".to_string(),
            JsonValue::String("value".to_string()),
        )]),
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]),
        JsonValue::Boolean(true),
    ]);
    assert_eq!(parser.parse(), expected_value);
}

#[test]
pub fn parse_valid_object_with_multiple_data_types_check() {
    let test_json = r#"{"string": "value", "number": 42, "boolean": false, "null_value": null}"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    let expected_value = JsonValue::Object(vec![
        ("string".to_string(), JsonValue::String("value".to_string())),
        ("number".to_string(), JsonValue::Number(42.0)),
        ("boolean".to_string(), JsonValue::Boolean(false)),
        ("null_value".to_string(), JsonValue::Null),
    ]);
    assert_eq!(parser.parse(), expected_value);
}
