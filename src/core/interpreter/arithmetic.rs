use crate::core::interpreter::interpreter::Interpreter;
use crate::utils::element::*;

impl Interpreter {
  pub fn div(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh / rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  pub fn mul(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh * rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  pub fn sub(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh - rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  pub fn add(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh + rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  pub fn modulo(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh % rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
}
