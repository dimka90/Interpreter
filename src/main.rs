mod types;
mod behaviours;
use crate::types::types::types::{TokenType, Token};
fn main() {
let token: Token<String> = Token::new(TokenType::AND, "AND".to_string(), Some(String::from("&&")), 1);

println!("{:?}", token);
}
