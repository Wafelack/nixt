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
    self.current >= self.tokens.len()
  }
  fn parse_block(&mut self, blk_type: TokenType) -> Node {
    let mut toret = Node::new(Block);

    let mut current = self.advance();
    while !self.is_at_end() && current.typ != blk_type {
      let to_add = match current.typ {
        LeftParen | LeftBrace => self.parse_block(current.typ),
        Let | Const | Set => self.parse_assignement(&current.typ),
        _ => {
          self.had_error = true;
          self.errors.push(format!("Invalid token: {:?}", current));
          Node::new(None)
        }
      };
      toret.add_children(&to_add);
      current = self.advance();
    }
    toret
  }
  fn parse_assignement(&mut self, typ: &TokenType) -> Node {
    let name_tok = self.advance();

    let name = match name_tok.typ {
      Identifier(s) => Node::new(NodeIdentifier(s)),
      _ => {
        self.had_error = true;
        self.errors.push(format!("Invalid token: {:?}", name_tok));
        return Node::new(None);
      }
    };

    let value_tok = self.advance();

    let value = match value_tok.typ {
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      Identifier(s) => Node::new(NodeIdentifier(s)),
      RightParen | RightBrace => self.parse_block(value_tok.typ),
      _ => {
        self.had_error = true;
        self.errors.push(format!("Invalid token: {:?}", value_tok));
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
        self.errors.push(format!("Invalid token: {:?}", current));
      }
    }
  }
  pub fn parse(&mut self) -> Node {
    while !self.is_at_end() {
      self.parse_token();
    }
    self.ast.clone()
  }
}
