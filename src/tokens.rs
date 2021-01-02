#[derive(Debug)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    String(String),
    Identifier(String),
    Number(f32),
    Equal,
    EqualEqual,
    Colon,
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
    Bang,
    BangEqual,

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
    Var
}
#[derive(Debug)]
pub struct Token {
    typ: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: String, line: usize) -> Self{
        Self {
            typ,
            lexeme,
            line
        }
    }
}
