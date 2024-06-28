use jsruston::lexer::Lexer;
use jsruston::parser::Parser;

fn main() {
    let test_json = r#"[{"name": "John"}]"#;
    let mut lexer = Lexer::new(test_json);
    let mut parser = Parser::new(lexer.lex());
    println!("{:?}", parser.parse());
}
