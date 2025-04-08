mod types;
mod behaviours;
use crate::types::types::types::{TokenType, Token, Scanner, LiteralValue};
fn main() {
let mut scan:Scanner = Scanner::new("{/*-");

scan.scan_tokens();

println!("{:?}", scan);
}