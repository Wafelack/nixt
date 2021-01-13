use crate::core::interpreter::interpreter::Interpreter;
use crate::utils::{element::*, node::*};

impl Interpreter {
  pub fn process_loop(&mut self, master: &Node) -> Result<(), String> {
    let raw_condition = &master.get_child()[0].get_child()[0];
    while self.eval_condition(raw_condition)? {
      self.process_node(&master.get_child()[1])?;
    }
    Ok(())
  }
  pub fn process_if(&mut self, master: &Node) -> Result<(), String> {
    let raw_condition = &master.get_child()[0].get_child()[0];
    if self.eval_condition(raw_condition)? {
      self.process_node(&master.get_child()[1])?;
    } else {
      if &master.get_child()[2].get_type() == &NodeType::None {
        return Ok(());
      } else {
        self.process_node(&master.get_child()[2])?;
      }
    }
    Ok(())
  }
  pub fn proc_operator(&mut self, op: OperatorType, val: &Node) -> Result<Value, String> {
    let lhs = match val.get_child()[0].get_type() {
      NodeType::Block => self.process_inner_block(&val.get_child()[0])?,
      NodeType::NodeNumber(n) => Value::Number(n),
      NodeType::NodeStr(s) => Value::String(s),
      NodeType::NodeBool(b) => Value::Bool(b),
      NodeType::None => Value::Nil,
      NodeType::NodeIdentifier(s) => {
        if self.get_value(&s).is_some() {
          self.get_value(&s).unwrap()
        } else {
          return Err("Attempted to access an undefined variable".to_owned());
        }
      }
      _ => return Err("Invalid element".to_owned()),
    };
    let rhs = match val.get_child()[1].get_type() {
      NodeType::Block => self.process_inner_block(&val.get_child()[1])?,
      NodeType::NodeNumber(n) => Value::Number(n),
      NodeType::NodeStr(s) => Value::String(s),
      NodeType::NodeBool(b) => Value::Bool(b),
      NodeType::None => Value::Nil,
      NodeType::NodeIdentifier(s) => {
        if self.get_value(&s).is_some() {
          self.get_value(&s).unwrap()
        } else {
          return Err("Attempted to access an undefined variable".to_owned());
        }
      }
      _ => return Err("Invalid element".to_owned()),
    };
    let toret = match op {
      OperatorType::Div => self.div(lhs, rhs)?,
      OperatorType::Times => self.mul(lhs, rhs)?,
      OperatorType::Plus => self.add(lhs, rhs)?,
      OperatorType::Minus => self.sub(lhs, rhs)?,
      OperatorType::Modulo => self.modulo(lhs, rhs)?,
      OperatorType::Equal => self.eq(lhs, rhs)?,
      OperatorType::NotEqual => self.neq(lhs, rhs)?,
      OperatorType::LessEqual => self.leq(lhs, rhs)?,
      OperatorType::Less => self.le(lhs, rhs)?,
      OperatorType::Greater => self.ge(lhs, rhs)?,
      OperatorType::GreaterEqual => self.geq(lhs, rhs)?,
      OperatorType::And => self.and(lhs, rhs)?,
      OperatorType::Or => self.or(lhs, rhs)?,
    };
    Ok(toret)
  }
}
