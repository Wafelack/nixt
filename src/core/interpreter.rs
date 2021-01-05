use crate::utils::node::*;

pub fn interpret_operation(block: &Node) -> Option<f32> {
  let content = match block.get_type() {
    NodeType::Block => block.get_child(),
    _ => return None,
  };
  if content.len() < 1 {
    return None;
  }
  let operator_node = &content[0];
  let operator = match operator_node.get_type() {
    NodeType::Operator(op) => op,
    NodeType::Block => return interpret_operation(&operator_node),
    _ => return None,
  };

  if content[0].get_child().len() < 2 {
    return None;
  }
  let child = content[0].get_child();

  let lhs = match &child[0].get_type() {
    NodeType::NodeNumber(f) => *f,
    NodeType::Block => interpret_operation(&child[0]).unwrap_or(0.),
    _ => 0.,
  };

  let rhs = match &child[1].get_type() {
    NodeType::NodeNumber(f) => *f,
    NodeType::Block => interpret_operation(&child[1]).unwrap_or(0.),
    _ => 0.,
  };

  match operator {
    OperatorType::Plus => Some(lhs + rhs),
    OperatorType::Minus => Some(lhs - rhs),
    OperatorType::Times => Some(lhs * rhs),
    OperatorType::Div => Some(lhs / rhs),
  }
}
