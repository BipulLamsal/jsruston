#[derive(Debug, PartialEq)]
pub enum Token {
    BeginArray,     // [
    BeginObject,    // {
    EndArray,       // ]
    EndObject,      // }
    NameSeperator,  // :
    ValueSeperator, // ,
    ValueNumber(f64),
    ValueBoolean(bool),
    ValueString(String),
    ValueNil,
}
