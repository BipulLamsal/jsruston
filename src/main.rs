use std::error::Error;

use jsruston::lexer::Lexer;
use jsruston::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let john = r#"{
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;
    let mut lexer = Lexer::new(john);
    let lex = lexer.lex()?;
    let parserd_json = Parser::new(lex).parse()?;
    println!(
        "parsed json value: {:?}",
        parserd_json["phones"][0].get_string().unwrap()
    );
    Ok(())
}
