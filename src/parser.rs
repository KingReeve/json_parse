use crate::lexer::Token;
use crate::value::JsonValue;
use std::collections::HashMap;

pub fn parse(tokens: Vec<Token>) -> Result<JsonValue, String> {
    if tokens.len() == 2
        && tokens[0] == Token::LeftBrace
        && tokens[1] == Token::RightBrace
    {
        Ok(JsonValue::Object(HashMap::new()))
    } else {
        Err("Expected '{}'".to_string())
    }
}