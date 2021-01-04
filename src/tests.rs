#[cfg(test)]
mod test {
  use crate::{
    core::lexer::*,
    core::parser::*,
    utils::node::*,
    utils::token::{TokenType::*, *},
  };

  #[test]
  fn parsing() -> Result<(), ()> {
    let to_parse = "(let foo 5)";
    let mut lexer = Lexer::new(to_parse);
    let tokens = lexer.scan_tokens();
    if lexer.had_error() {
      return Err(());
    }
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    if parser.had_error() {
      return Err(());
    }
    Ok(())
  }
  #[test]
  fn tokenizing() {
    let to_tokenize = "(let foo (+ 5 ( + 9 4)))(if (> foo 14) (print \"Greater that 14\")(print \"Less that 14\")))";
    let mut lexer = Lexer::new(to_tokenize);
    let tokens = lexer.scan_tokens();

    assert_eq!(
      tokens,
      vec![
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: Let,
          lexeme: "let".to_owned(),
          line: 1
        },
        Token {
          typ: Identifier("foo".to_owned()),
          lexeme: "foo".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: Plus,
          lexeme: "+".to_owned(),
          line: 1
        },
        Token {
          typ: Number(5.0),
          lexeme: "5".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: Plus,
          lexeme: "+".to_owned(),
          line: 1
        },
        Token {
          typ: Number(9.0),
          lexeme: "9".to_owned(),
          line: 1
        },
        Token {
          typ: Number(4.0),
          lexeme: "4".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: If,
          lexeme: "if".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: Greater,
          lexeme: ">".to_owned(),
          line: 1
        },
        Token {
          typ: Identifier("foo".to_owned()),
          lexeme: "foo".to_owned(),
          line: 1
        },
        Token {
          typ: Number(14.0),
          lexeme: "14".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: Print,
          lexeme: "print".to_owned(),
          line: 1
        },
        Token {
          typ: Str("Greater that 14".to_owned()),
          lexeme: "\"Greater that 14\"".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: LeftParen,
          lexeme: "(".to_owned(),
          line: 1
        },
        Token {
          typ: Print,
          lexeme: "print".to_owned(),
          line: 1
        },
        Token {
          typ: Str("Less that 14".to_owned()),
          lexeme: "\"Less that 14\"".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: RightParen,
          lexeme: ")".to_owned(),
          line: 1
        },
        Token {
          typ: Eof,
          lexeme: "".to_owned(),
          line: 1
        }
      ]
    )
  }
}
