#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    LeftBrace,
    RightBrace,
    RightParen,
    Str(String),
    Identifier(String),
    Number(f32),
    Equal,
    Dot,
    Comma,
    Percent,
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Tilde,

    And,
    Func,
    If,
    Nil,
    Or,
    Return,
    True,
    False,
    Let,
    Const,
    While,
    Set,

    Eof,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize) -> Self {
        Self { typ, lexeme, line }
    }
}
