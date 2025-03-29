
use crate::types::types::types::{Token, TokenType};
impl<T> Token<T>{
pub fn new(token_type: TokenType, lexeme: String, literal: Option<T>, line: u16) -> Self{
    Self{
        token_type,
        lexeme,
        literal,
        line,
    }
}
}