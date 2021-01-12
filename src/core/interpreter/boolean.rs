use crate::core::interpreter::interpreter::Interpreter;
use crate::utils::{element::*, node::*};

impl Interpreter {
  pub fn and(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(lh && rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  pub fn or(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(lh || rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  pub fn eq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(rh == lh)),
        _ => Ok(Value::Bool(false)),
      },
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(rh == lh)),
        _ => Ok(Value::Bool(false)),
      },
      Value::String(lh) => match rhs {
        Value::String(rh) => Ok(Value::Bool(rh == lh)),
        _ => Ok(Value::Bool(false)),
      },
      Value::Nil => match rhs {
        Value::Nil => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
      },
      _ => Ok(Value::Bool(false)),
    }
  }

  pub fn neq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(rh != lh)),
        _ => Ok(Value::Bool(true)),
      },
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(rh != lh)),
        _ => Ok(Value::Bool(true)),
      },
      Value::String(lh) => match rhs {
        Value::String(rh) => Ok(Value::Bool(rh != lh)),
        _ => Ok(Value::Bool(true)),
      },
      Value::Nil => match rhs {
        Value::Nil => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
      },
      _ => Ok(Value::Bool(true)),
    }
  }

  pub fn leq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh <= rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  pub fn le(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh < rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  pub fn geq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh >= rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  pub fn ge(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh > rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
}
