#[derive(Debug, Clone)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Str(String),
    Identifier(String),
    Number(f32),
    Equal,
    EqualEqual,
    Dot,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Tilde,
    TildeEqual,

    And,
    Func,
    Else,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    False,
    While,
    Var,
    Const,
    Set,

    Eof,
}
#[derive(Debug, Clone)]
pub struct Token {
    typ: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize) -> Self {
        Self { typ, lexeme, line }
    }
}
