use crate::lexer::Lexer;
use crate::token::Token;

#[test]
pub fn lex_valid_empty_delimitter_check() {
    let test_json = r#"{}"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(lexer.lex(), vec!(Token::BeginObject, Token::EndObject));
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

#[test]
pub fn lex_valid_nested_object() {
    let test_json =
        r#"{"name": "John", "age": 30, "address": {"city": "New York", "state": "NY"}}"#;
    let mut lexer = Lexer::new(test_json);
    let tokens = lexer.lex();

    let expected_tokens = vec![
        Token::BeginObject,
        Token::ValueString("name".to_string()),
        Token::NameSeperator,
        Token::ValueString("John".to_string()),
        Token::ValueSeperator,
        Token::ValueString("age".to_string()),
        Token::NameSeperator,
        Token::ValueNumber(30.0),
        Token::ValueSeperator,
        Token::ValueString("address".to_string()),
        Token::NameSeperator,
        Token::BeginObject,
        Token::ValueString("city".to_string()),
        Token::NameSeperator,
        Token::ValueString("New York".to_string()),
        Token::ValueSeperator,
        Token::ValueString("state".to_string()),
        Token::NameSeperator,
        Token::ValueString("NY".to_string()),
        Token::EndObject,
        Token::EndObject,
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
pub fn lex_valid_empty_array() {
    let test_json = r#"[]"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(lexer.lex(), vec!(Token::BeginArray, Token::EndArray));
}

#[test]
pub fn lex_valid_array_with_values() {
    let test_json = r#"[1, "two", true, null]"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginArray,
            Token::ValueNumber(1 as f64),
            Token::ValueSeperator,
            Token::ValueString("two".to_string()),
            Token::ValueSeperator,
            Token::ValueBoolean(true),
            Token::ValueSeperator,
            Token::ValueNil,
            Token::EndArray,
        )
    )
}

#[test]
pub fn lex_invalid_missing_comma() {
    let test_json = r#"{"key1": "value1" "key2": "value2"}"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginObject,
            Token::ValueString("key1".to_string()),
            Token::NameSeperator,
            Token::ValueString("value1".to_string()),
            Token::ValueString("key2".to_string()),
            Token::NameSeperator,
            Token::ValueString("value2".to_string()),
            Token::EndObject,
        )
    )
}

#[test]
pub fn lex_valid_mixed_array() {
    let test_json = r#"[{"key": "value"}, [1, 2, 3], true]"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginArray,
            Token::BeginObject,
            Token::ValueString("key".to_string()),
            Token::NameSeperator,
            Token::ValueString("value".to_string()),
            Token::EndObject,
            Token::ValueSeperator,
            Token::BeginArray,
            Token::ValueNumber(1 as f64),
            Token::ValueSeperator,
            Token::ValueNumber(2 as f64),
            Token::ValueSeperator,
            Token::ValueNumber(3 as f64),
            Token::EndArray,
            Token::ValueSeperator,
            Token::ValueBoolean(true),
            Token::EndArray,
        )
    )
}

#[test]
pub fn lex_invalid_unterminated_string() {
    let test_json = r#"{"key": "value"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginObject,
            Token::ValueString("key".to_string()),
            Token::NameSeperator,
            Token::ValueString("value".to_string())
        )
    )
}

#[test]
pub fn lex_valid_escaped_characters() {
    let test_json = r#"{"escaped": "Line1\nLine2"}"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginObject,
            Token::ValueString("escaped".to_string()),
            Token::NameSeperator,
            Token::ValueString("Line1\nLine2".to_string()),
            Token::EndObject,
        )
    )
}

#[test]
pub fn lex_invalid_extra_comma() {
    let test_json = r#"{"key": "value",}"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginObject,
            Token::ValueString("key".to_string()),
            Token::NameSeperator,
            Token::ValueString("value".to_string()),
            Token::ValueSeperator,
            Token::EndObject,
        )
    )
}

#[test]
pub fn lex_valid_object_with_multiple_data_types() {
    let test_json = r#"{"string": "value", "number": 42, "boolean": false, "null_value": null}"#;
    let mut lexer = Lexer::new(test_json);
    assert_eq!(
        lexer.lex(),
        vec!(
            Token::BeginObject,
            Token::ValueString("string".to_string()),
            Token::NameSeperator,
            Token::ValueString("value".to_string()),
            Token::ValueSeperator,
            Token::ValueString("number".to_string()),
            Token::NameSeperator,
            Token::ValueNumber(42 as f64),
            Token::ValueSeperator,
            Token::ValueString("boolean".to_string()),
            Token::NameSeperator,
            Token::ValueBoolean(false),
            Token::ValueSeperator,
            Token::ValueString("null_value".to_string()),
            Token::NameSeperator,
            Token::ValueNil,
            Token::EndObject,
        )
    )
}
