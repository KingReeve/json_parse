#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for c in input.chars() {
        match c {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            c if c.is_whitespace() => continue,
            _ => {}
        }
    }

    tokens
}