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
  line: usize,
}
impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      ast: Node::new(Block),
      current: 0,
      had_error: false,
      errors: vec![],
      line: 1,
    }
  }
  fn advance(&mut self) -> Token {
    self.current += 1;
    let toret = self.tokens[self.current - 1].clone();
    self.line = toret.line;
    toret
  }
  fn is_at_end(&self) -> bool {
    self.current >= self.tokens.len() || self.tokens[self.current].typ == Eof
  }
  fn parse_block(&mut self, ast: bool) -> Node {
    let mut toret = Node::new(Block);

    loop {
      if self.is_at_end() || self.peek().unwrap().typ == RightParen {
        if self.peek().is_some() && self.peek().unwrap().typ == RightParen {
          self.advance(); // Consume closing char
        }
        break;
      }
      let current = self.advance();
      let to_add = match current.typ {
        TokenType::Return => self.parse_return(),
        If => self.parse_condition(),
        While => self.parse_loop(),
        LeftParen => self.parse_block(ast),
        Let | Const | Set => self.parse_assignement(&current.typ),
        Plus | Minus | Star | Slash => self.parse_op(&current.typ),
        Less | LessEqual | And | Or | Tilde | Equal | Greater | GreaterEqual => {
          self.parse_verif(&current.typ)
        }
        TokenType::Func => self.parse_func(),
        Identifier(s) => self.function_call(s),
        _ => {
          self.had_error = true;
          self.errors.push(format!(
            "{} | Invalid token: {:?} occured on code line {}",
            self.line,
            current,
            line!()
          ));
          Node::new(None)
        }
      };
      toret.add_children(&to_add);
      if ast {
        self.ast.add_children(&toret);
      }
    }
    toret
  }

  fn function_call(&mut self, s: String) -> Node {
    let mut master = Node::new(NodeIdentifier(s));
    let mut args: Vec<Node> = vec![];

    loop {
      if self.is_at_end() || self.peek().unwrap().typ == RightParen {
        if self.peek().is_some() && self.peek().unwrap().typ == RightParen {
          self.advance(); // Consume closing char
        }
        break;
      }
      let current = self.advance();

      let to_add = match &current.typ {
        Identifier(s) => self.function_call(s.to_owned()),
        Str(s) => Node::new(NodeStr(s.to_owned())),
        Number(f) => Node::new(NodeNumber(*f)),
        LeftParen => self.parse_block(false),
        True => Node::new(NodeBool(true)),
        False => Node::new(NodeBool(false)),
        _ => {
          self.had_error = true;
          self.errors.push(format!(
            "{} | Invalid token: {:?}-{} occured on code line {}",
            self.line,
            current,
            self.current,
            line!()
          ));
          Node::new(None)
        }
      };
      args.push(to_add);
    }
    for arg in args {
      master.add_children(&arg);
    }
    master
  }
  fn parse_return(&mut self) -> Node {
    let to_ret = self.advance();

    let mut master = Node::new(NodeType::Return);

    let value = match to_ret.typ {
      Identifier(s) => Node::new(NodeIdentifier(s)),
      Str(s) => Node::new(NodeStr(s)),
      Number(f) => Node::new(NodeNumber(f)),
      LeftParen => self.parse_block(false),
      TokenType::Func => self.parse_func(),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          to_ret,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };
    master.add_children(&value);
    master
  }
  fn parse_verif(&mut self, typ: &TokenType) -> Node {
    let mut master = match typ {
      Less => Node::new(Check(CheckType::Less)),
      LessEqual => Node::new(Check(CheckType::LessEqual)),
      Greater => Node::new(Check(CheckType::Greater)),
      GreaterEqual => Node::new(Check(CheckType::GreaterEqual)),
      Equal => Node::new(Check(CheckType::Equal)),
      Tilde => Node::new(Check(CheckType::NotEqual)),
      And => Node::new(Check(CheckType::And)),
      _ => Node::new(Check(CheckType::Or)),
    };

    let lhs_tok = self.advance();

    let lhs = match &lhs_tok.typ {
      LeftParen => self.parse_block(false),
      Number(f) => Node::new(NodeNumber(*f)),
      Str(s) => Node::new(NodeStr((*s).clone())),
      True | False => {
        if &lhs_tok.typ == &True {
          Node::new(NodeBool(true))
        } else {
          Node::new(NodeBool(false))
        }
      }
      Identifier(s) => Node::new(NodeIdentifier(s.to_owned())),
      TokenType::Func => self.parse_func(),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          lhs_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    let rhs_tok = self.advance();

    let rhs = match &rhs_tok.typ {
      LeftParen => self.parse_block(false),
      Number(f) => Node::new(NodeNumber(*f)),
      Str(s) => Node::new(NodeStr((*s).clone())),
      True | False => {
        if &rhs_tok.typ == &True {
          Node::new(NodeBool(true))
        } else {
          Node::new(NodeBool(false))
        }
      }
      Identifier(s) => Node::new(NodeIdentifier(s.to_owned())),
      TokenType::Func => self.parse_func(),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          rhs_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    master.add_children(&lhs);
    master.add_children(&rhs);
    master
  }
  fn parse_loop(&mut self) -> Node {
    let mut master = Node::new(Loop);
    let first_tok = self.advance();

    let check = match &first_tok.typ {
      LeftParen => self.parse_block(false),
      Identifier(s) => Node::new(NodeIdentifier(s.to_string())),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          first_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    let body_tok = self.advance();
    let body = match &body_tok.typ {
      LeftParen => self.parse_block(false),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid character {:?}-{} occured on code line {}",
          self.line,
          body_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    master.add_children(&check);
    master.add_children(&body);
    master
  }
  fn parse_func(&mut self) -> Node {
    let mut master = Node::new(NodeType::Func);

    let first_tok = self.advance();
    let args = match &first_tok.typ {
      LeftParen => self.parse_args(),
      Eof => return Node::new(None),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          first_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };
    let sec_tok = self.advance();
    let body = match &sec_tok.typ {
      LeftParen => self.parse_block(false),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          sec_tok,
          self.current,
          line!()
        ));
        Node::new(Block)
      }
    };

    master.add_children(&args);
    master.add_children(&body);
    master
  }
  fn parse_args(&mut self) -> Node {
    let mut master = Node::new(Block);
    let mut args = vec![];
    loop {
      if self.is_at_end() || self.peek().unwrap().typ == RightParen {
        if self.peek().is_some() && self.peek().unwrap().typ == RightParen {
          self.advance(); // Consume closing char
        }
        break;
      }
      let current = self.advance();

      match &current.typ {
        Identifier(s) => args.push(Node::new(NodeIdentifier(s.to_owned()))),
        _ => {}
      }
    }
    for arg in args {
      master.add_children(&arg);
    }
    master
  }
  fn parse_condition(&mut self) -> Node {
    let mut master = Node::new(Condition);

    let first_tok = self.advance();

    let check = match &first_tok.typ {
      LeftParen => self.parse_block(false),
      Identifier(s) => Node::new(NodeIdentifier(s.to_string())),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          first_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    let todo_if_tok = self.advance();
    let todo_if = match &todo_if_tok.typ {
      LeftParen => self.parse_block(false),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid character {:?}",
          self.line, todo_if_tok
        ));
        Node::new(None)
      }
    };
    let todo_else_tok = self.advance();

    let todo_else = match &todo_else_tok.typ {
      LeftParen => self.parse_block(false),
      _ => Node::new(None), // Valid because Else block is not required
    };

    master.add_children(&check);
    master.add_children(&todo_if);
    master.add_children(&todo_else);
    master
  }
  fn peek(&self) -> Option<Token> {
    if self.is_at_end() {
      return std::option::Option::None;
    }
    Some(self.tokens[self.current].clone())
  }
  fn parse_op(&mut self, typ: &TokenType) -> Node {
    let first_tok = self.advance();

    let first = match first_tok.typ {
      LeftParen => self.parse_block(false),
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      Identifier(s) => Node::new(NodeIdentifier(s)),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          first_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    let second_tok = self.advance();

    let second = match second_tok.typ {
      LeftParen => self.parse_block(false),
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      Identifier(s) => Node::new(NodeIdentifier(s)),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          second_tok,
          self.current,
          line!()
        ));
        Node::new(None)
      }
    };

    let mut master = match typ {
      Plus => Node::new(Operator(OperatorType::Plus)),
      Minus => Node::new(Operator(OperatorType::Minus)),
      Star => Node::new(Operator(OperatorType::Times)),
      _ => Node::new(Operator(OperatorType::Div)),
    };
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
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          name_tok,
          self.current,
          line!()
        ));
        return Node::new(None);
      }
    };

    let value_tok = self.advance();

    let value = match value_tok.typ {
      Number(f) => Node::new(NodeNumber(f)),
      Str(s) => Node::new(NodeStr(s)),
      Identifier(s) => Node::new(NodeIdentifier(s)),
      True => Node::new(NodeBool(true)),
      False => Node::new(NodeBool(false)),
      Plus | Minus | Star | Slash => self.parse_op(&value_tok.typ),
      LeftParen => self.parse_block(false),
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          value_tok,
          self.current,
          line!()
        ));
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

    master
  }
  fn parse_token(&mut self) {
    let current = self.advance();
    self.line = current.line;

    match current.typ {
      LeftParen => {
        let mut blck = Node::new(Block);
        blck.add_children(&self.parse_block(true));
      }
      _ => {
        self.had_error = true;
        self.errors.push(format!(
          "{} | Invalid token: {:?}-{} occured on code line {}",
          self.line,
          current,
          self.current,
          line!()
        ));
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
