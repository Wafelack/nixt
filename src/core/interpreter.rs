use crate::utils::{elements::*, node::*};
use std::mem;

pub struct Interpreter {
  pub stack: Vec<StackElement>,
  pub ast: Node,
}

impl Interpreter {
  pub fn new(ast: &Node) -> Self {
    Self {
      stack: vec![],
      ast: (*ast).clone(),
    }
  }
  pub fn interpret_blocks(&mut self) {
    self.interpret_block(self.ast.clone());
  }
  fn interpret_assignement(&mut self, current: &Node, typ: AssignType) {
    let children = current.get_child();
    if children.len() < 2 {
      return;
    }
    let first_typ = &children[0].get_type();
    let name = match first_typ {
      NodeType::NodeIdentifier(s) => s,
      _ => return,
    };

    let value = match &children[1].get_type() {
      NodeType::NodeIdentifier(s) => todo!(),
      NodeType::NodeStr(s) => Value::String(s.to_owned()),
      NodeType::NodeNumber(n) => Value::Number(*n),
      NodeType::NodeBool(b) => Value::Bool(*b),
      NodeType::None => Value::Nil,
      NodeType::Block => self.interpret_block(children[1].clone()),
      _ => todo!(),
    };

    self.stack.push(StackElement {
      name: name.to_owned(),
      typ: match typ {
        AssignType::Const => VariableType::Constant,
        _ => VariableType::Mutable,
      },
      value: value,
    });
  }
  pub fn interpret_block(&mut self, block: Node) -> Value {
    let mut i = 0usize;
    let children = block.get_child();
    while i < children.len() {
      let current = &children[i];
      match current.get_type() {
        NodeType::Block => {
          self.interpret_block((*current).clone());
        }
        NodeType::Operator(op) => return self.interpret_operation(current),
        NodeType::Assignement(t) => {
          self.interpret_assignement(current, t);
        }
        _ => {}
      }
      i += 1;
    }
    Value::Nil
  }

  pub fn interpret_operation(&mut self, block: &Node) -> Value {
    let (operator, child) = match block.get_type() {
      NodeType::Operator(op) => (op, block.get_child()),
      _ => return Value::Nil,
    };
    let lhs = match &child[0].get_type() {
      NodeType::NodeNumber(f) => Value::Number(*f),
      NodeType::NodeStr(s) => Value::String(s.to_owned()),
      NodeType::NodeBool(b) => Value::Bool(*b),
      NodeType::NodeIdentifier(s) => todo!(),
      NodeType::Block => self.interpret_operation(&child[0]),
      _ => Value::Nil,
    };
    let rhs = match &child[1].get_type() {
      NodeType::NodeNumber(f) => Value::Number(*f),
      NodeType::NodeStr(s) => Value::String(s.to_owned()),
      NodeType::NodeBool(b) => Value::Bool(*b),
      NodeType::NodeIdentifier(s) => todo!(),
      NodeType::Block => self.interpret_operation(&child[1]),
      _ => Value::Nil,
    };
    match operator {
      OperatorType::Plus => add(lhs, rhs),
      OperatorType::Minus => sub(lhs, rhs),
      OperatorType::Times => times(lhs, rhs),
      OperatorType::Div => div(lhs, rhs),
      OperatorType::And => and(lhs, rhs),
      OperatorType::Or => or(lhs, rhs),
      OperatorType::Equal => eq(lhs, rhs),
      OperatorType::NotEqual => neq(lhs, rhs),
      OperatorType::Less => less(lhs, rhs),
      OperatorType::Greater => more(lhs, rhs),
      OperatorType::LessEqual => leeq(lhs, rhs),
      OperatorType::GreaterEqual => moeq(lhs, rhs),
      _ => Value::Nil,
    }
  }
}

fn and(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Bool(lh) => match rhs {
      Value::Bool(rh) => Value::Bool(lh && rh),
      _ => Value::Bool(false),
    },
    _ => Value::Bool(false),
  }
}

fn less(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Bool(lh < rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn more(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Bool(lh > rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn moeq(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Bool(lh >= rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn leeq(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Bool(lh <= rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn neq(lhs: Value, rhs: Value) -> Value {
  if mem::discriminant(&lhs) != mem::discriminant(&rhs) {
    return Value::Bool(true);
  }
  match lhs {
    Value::String(lh) => match rhs {
      Value::String(rh) => Value::Bool(lh != rh),
      _ => Value::Bool(true),
    },
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Bool(lh != rh),
      _ => Value::Bool(true),
    },
    Value::Bool(lh) => match rhs {
      Value::Bool(rh) => Value::Bool(lh != rh),
      _ => Value::Bool(true),
    },
    _ => todo!(),
  }
}
fn eq(lhs: Value, rhs: Value) -> Value {
  if mem::discriminant(&lhs) != mem::discriminant(&rhs) {
    return Value::Bool(false);
  }
  match lhs {
    Value::String(lh) => match rhs {
      Value::String(rh) => Value::Bool(lh == rh),
      _ => Value::Bool(false),
    },
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Bool(lh == rh),
      _ => Value::Bool(false),
    },
    Value::Bool(lh) => match rhs {
      Value::Bool(rh) => Value::Bool(lh == rh),
      _ => Value::Bool(false),
    },
    _ => todo!(),
  }
}

fn or(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Bool(lh) => match rhs {
      Value::Bool(rh) => Value::Bool(lh || rh),
      _ => Value::Bool(false),
    },
    _ => Value::Bool(false),
  }
}

fn add(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::String(lh) => match rhs {
      Value::String(rh) => Value::String(format!("{}{}", lh, rh)),
      _ => Value::Nil,
    },
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Number(lh + rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn sub(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Number(lh - rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn times(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Number(lh * rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}

fn div(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::Number(lh) => match rhs {
      Value::Number(rh) => Value::Number(lh / rh),
      _ => Value::Nil,
    },
    _ => Value::Nil,
  }
}
