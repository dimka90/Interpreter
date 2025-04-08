pub mod types{

pub struct  Invincible;
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}
#[derive(Debug)]
pub  struct  Token{
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralValue,
    pub line: u16,
}
#[derive(Debug)]
pub  struct Scanner{
    pub source: String,
    pub start: u16,
    pub line: u16,
    pub current: u16,
    pub tokens: Vec<Token>,
}
#[derive(Debug)]
pub enum  LiteralValue {
    Double(f64),
    Number(u64),
    String(String),
    Boolean(bool),
    NIL,
    
}
}