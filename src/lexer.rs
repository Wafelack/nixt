use crate::{tokens::TokenType::*, tokens::*};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
    errors: Vec<String>,
    keywords: BTreeMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut keywords = BTreeMap::new();
        keywords.insert("func".to_owned(), Func);
        keywords.insert("else".to_owned(), Else);
        keywords.insert("if".to_owned(), If);
        keywords.insert("nil".to_owned(), Nil);
        keywords.insert("or".to_owned(), Or);
        keywords.insert("print".to_owned(), Print);
        keywords.insert("ret".to_owned(), Return);
        keywords.insert("true".to_owned(), True);
        keywords.insert("false".to_owned(), False);
        keywords.insert("while".to_owned(), While);
        keywords.insert("var".to_owned(), Var);
        keywords.insert("const".to_owned(), Const);
        keywords.insert("set".to_owned(), Set);
        keywords.insert("and".to_owned(), And);
        Self {
            source: source.to_owned(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
            errors: vec![],
            keywords: keywords,
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
                if self.match_('=') {
                    self.multi_line_comment();
                } else {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
            }
            ' ' | '\r' | '\t' => {}
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '\n' => self.line += 1,
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    self.had_error = true;
                    self.errors
                        .push(format!("{} | Unexpected character", self.line));
                }
            }
        }
    }
    fn multi_line_comment(&mut self) {
        while self.peek() != '=' && self.peek_next() != '%' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.advance();
        self.advance(); // Skip =%
    }
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let copied = self.clone();
        let text = &copied.source[self.start..self.current];

        if copied.is_keyword(&text).is_none() {
            self.add_token(Identifier(text.to_owned()));
        } else {
            self.add_token(copied.is_keyword(&text).unwrap()); // safe because checked above
        }
    }
    fn is_keyword(&self, word: &str) -> Option<TokenType> {
        if !self.keywords.contains_key(word) {
            return None;
        } else {
            return Some(self.keywords[word].clone());
        }
    }
    fn number(&mut self) {
        while self.peek().is_digit(10) || self.peek() == '.' {
            self.advance();
        }
        let num = self.source[self.start..self.current]
            .parse::<f32>()
            .unwrap_or(-1.);
        self.add_token(Number(num));
    }
    fn string(&mut self, delimiter: char) {
        while self.peek() != delimiter && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.errors
                .push(format!("{} | Unterminated string", self.line));
            self.had_error = true;
            return;
        }
        self.advance(); // Consume closing character
        let value = (&self.source[self.start + 1..self.current - 1]).to_owned();
        self.add_token(Str(value));
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().collect::<Vec<char>>()[self.current]
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().collect::<Vec<char>>()[self.current + 1]
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
