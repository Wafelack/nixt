use crate::utils::{
  node::{NodeType::*, *},
  token::{TokenType::*, *},
};

#[derive(Debug)]
pub struct Parser {
  tokens: Vec<Token>,
  ast: Node,
  current: usize,
  had_error: bool,
  errors: Vec<String>,
}
impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      ast: Node::new(Block),
      current: 0,
      had_error: false,
      errors: vec![],
    }
  }
  fn advance(&mut self) -> Token {
    self.current += 1;
    self.tokens[self.current - 1].clone()
  }
  fn is_at_end(&self) -> bool {
    self.current >= self.tokens.len() || self.tokens[self.current].typ == Eof
  }
  fn parse_block(&mut self, blk_type: TokenType) -> Node {
    let mut toret = Node::new(Block);
    let to_match = match blk_type {
      LeftParen => RightParen,
      _ => RightBrace,
    };

    loop {
      if self.is_at_end() || self.peek().unwrap().typ == to_match {
        if self.peek().is_some() && self.peek().unwrap().typ == to_match {
          self.advance(); // Consume closing char
        }
        break;
      }
      let current = self.advance();
      let to_add = match current.typ {
        LeftParen | LeftBrace => self.parse_block(current.typ),
        Let | Const | Set => self.parse_assignement(&current.typ),
        Plus => self.parse_adding(),
        Identifier(s) => Node::new(NodeIdentifier(s)),
        _ => {
          self.had_error = true;
          self
            .errors
            .push(format!("{} | Invalid token: {:?}", line!(), current));
          Node::new(None)
        }
      };
      toret.add_children(&to_add);
    }
    toret
  }
  fn peek(&self) -> Option<Token> {
    if self.is_at_end() {
      return std::option::Option::None;
    }
    Some(self.tokens[self.current].clone())
  }
  fn parse_adding(&mut self) -> Node {
    let first_tok = self.advance();

    let first = match first_tok.typ {
      LeftParen | LeftBrace => self.parse_block(first_tok.typ),
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      _ => {
        self.had_error = true;
        self
          .errors
          .push(format!("{} | Invalid token: {:?}", line!(), first_tok));
        Node::new(None)
      }
    };

    let second_tok = self.advance();

    let second = match second_tok.typ {
      LeftParen | LeftBrace => self.parse_block(second_tok.typ),
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      _ => {
        self.had_error = true;
        self
          .errors
          .push(format!("{} | Invalid token: {:?}", line!(), second_tok));
        Node::new(None)
      }
    };

    let mut master = Node::new(Operator(OperatorType::Plus));
    master.add_children(&first);
    master.add_children(&second);
    master
  }
  fn parse_assignement(&mut self, typ: &TokenType) -> Node {
    let name_tok = self.advance();

    let name = match name_tok.typ {
      Identifier(s) => Node::new(NodeIdentifier(s)),
      _ => {
        self.had_error = true;
        self
          .errors
          .push(format!("{} | Invalid token: {:?}", line!(), name_tok));
        return Node::new(None);
      }
    };

    let value_tok = self.advance();

    let value = match value_tok.typ {
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      Identifier(s) => Node::new(NodeIdentifier(s)),
      Plus => self.parse_adding(),
      LeftParen | RightParen => self.parse_block(value_tok.typ),
      _ => {
        self.had_error = true;
        self
          .errors
          .push(format!("{} | Invalid token: {:?}", line!(), value_tok));
        return Node::new(None);
      }
    };

    let mut master = match typ {
      Const => Node::new(Assignement(AssignType::Const)),
      Set => Node::new(Assignement(AssignType::Set)),
      _ => Node::new(Assignement(AssignType::Let)),
    };

    master.add_children(&name);
    master.add_children(&value);

    self.ast.add_children(&master);

    master
  }
  fn parse_token(&mut self) {
    let current = self.advance();

    match current.typ {
      LeftParen | LeftBrace => {
        let mut blck = Node::new(Block);
        blck.add_children(&self.parse_block(current.typ));
      }
      _ => {
        self.had_error = true;
        self
          .errors
          .push(format!("{} | Invalid token: {:?}", line!(), current));
      }
    }
  }
  pub fn parse(&mut self) -> Node {
    while !self.is_at_end() {
      self.parse_token();
    }
    self.ast.clone()
  }
  pub fn had_error(&self) -> bool {
    self.had_error
  }
  pub fn get_errors(&self) -> Vec<String> {
    self.errors.clone()
  }
}
