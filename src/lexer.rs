use crate::{tokens::TokenType::*, tokens::*};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
    errors: Vec<String>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
            errors: vec![],
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            '*' => self.add_token(Star),
            '/' => self.add_token(Slash),
            '~' => {
                if self.match_('=') {
                    self.add_token(TildeEqual);
                } else {
                    self.add_token(Tilde);
                }
            }
            '=' => {
                if self.match_('=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }
            '<' => {
                if self.match_('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }
            '>' => {
                if self.match_('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            '%' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => {
                self.had_error = true;
                self.errors.push(format!(
                    "{}:{} | Unexpected character",
                    self.line, self.current
                ));
            }
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().collect::<Vec<char>>()[self.current]
    }
    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().collect::<Vec<char>>()[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().collect::<Vec<char>>()[self.current - 1]
    }
    fn add_token(&mut self, typ: TokenType) {
        let text = (&self.source[self.start..self.current]).to_owned();
        self.tokens.push(Token::new(typ, text, self.line));
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_owned(), self.line));
        self.tokens.clone()
    }
}
