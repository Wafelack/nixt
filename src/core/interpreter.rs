use crate::utils::{elements::*, node::*};

pub struct Interpreter {
  stack: Vec<StackElement>,
  ast: Node,
}

pub fn interpret_operation(block: &Node) -> Value {
  let content = match block.get_type() {
    NodeType::Block => block.get_child(),
    _ => return Value::Nil,
  };
  if content.len() < 1 {
    return Value::Nil;
  }
  let operator_node = &content[0];
  let operator = match operator_node.get_type() {
    NodeType::Operator(op) => op,
    NodeType::Block => return interpret_operation(&operator_node),
    _ => return Value::Nil,
  };

  if content[0].get_child().len() < 2 {
    return Value::Nil;
  }
  let child = content[0].get_child();

  let lhs = match &child[0].get_type() {
    NodeType::NodeNumber(f) => Value::Number(*f),
    NodeType::NodeStr(s) => Value::String(s.to_owned()),
    NodeType::NodeIdentifier(s) => todo!(),
    NodeType::Block => interpret_operation(&child[0]),
    _ => Value::Nil,
  };

  let rhs = match &child[1].get_type() {
    NodeType::NodeNumber(f) => Value::Number(*f),
    NodeType::NodeStr(s) => Value::String(s.to_owned()),
    NodeType::NodeIdentifier(s) => todo!(),
    NodeType::Block => interpret_operation(&child[1]),
    _ => Value::Nil,
  };

  match operator {
    OperatorType::Plus => add(lhs, rhs),
    /*OperatorType::Minus => sub(lhs, rhs),
    OperatorType::Times => times(lhs, rhs),
    OperatorType::Div => div(lhs, rhs),*/
    _ => Value::Nil,
  }
}

fn add(lhs: Value, rhs: Value) -> Value {
  match lhs {
    Value::String(lh) => match rhs {
      Value::String(rh) => return Value::String(format!("{}{}", lh, rh)),
      Value::Number(rh) => return Value::String(format!("{}{}", lh, rh)),
      _ => return Value::Nil,
    },
    Value::Number(lh) => match rhs {
      Value::Number(rh) => return Value::Number(lh + rh),
      Value::String(rh) => return Value::String(format!("{}{}", lh, rh)),
      _ => return Value::Nil,
    },
    _ => return Value::Nil,
  }
}
