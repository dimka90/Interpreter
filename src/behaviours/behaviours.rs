use crate::types::types::types::{LiteralValue, Scanner, Token, TokenType};
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line: u16) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            start: 1,
            line: 1,
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            let character = self.advance();
            match character {
                '(' => self.add_token(TokenType::LEFT_PAREN),
                ')' => self.add_token(TokenType::RIGHT_PAREN),
                '{' => self.add_token(TokenType::LEFT_BRACE),
                '}' => self.add_token(TokenType::RIGHT_BRACE),
                ',' => self.add_token(TokenType::COMMA),
                '.' => self.add_token(TokenType::DOT),
                '-' => self.add_token(TokenType::MINUS),
                '+' => self.add_token(TokenType::PLUS),
                ':' => self.add_token(TokenType::SEMICOLON),
                '/' => self.add_token(TokenType::SLASH),
                '*' => self.add_token(TokenType::STAR),
                'l' => println!("i am letter l"),
                'e' => println!("I am letter e"),
                't' => println!("I am letter t"),
                _ => (),
            }
        }
        vec![]
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u16;
    }
    pub fn advance(&mut self) -> char {
        self.current += 1;
        let character = self.source.chars().nth(self.current as usize - 1).unwrap();

        return character;
    }
    pub fn add_token(&mut self, token: TokenType) {
        let substring= self
            .source
            .chars()
            .skip(self.start.into())
            .take((self.current as u16 - self.start as u16).into())
            .collect();
        let token: Token = Token {
            token_type: token,
            lexeme: substring,
            literal: LiteralValue::NIL,
            line: self.line,
        };

        self.tokens.push(token);
    }
}
