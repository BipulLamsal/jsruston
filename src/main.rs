use jsruston::parser::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let john = r#"{
         "key": "value",
        "key-n": 101,
        "key-o": {},
        "key-l": []
    }"#;
    let parserd_json = Parser::parse_json(john)?;
    let key = &parserd_json["key-o"];
    println!("parsed json value: {:?}", key);
    Ok(())
}
