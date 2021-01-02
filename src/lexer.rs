use crate::{tokens::*};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            tokens: vec![],
        }
    }
}