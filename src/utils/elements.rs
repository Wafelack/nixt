use crate::utils::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct StackElement {
  pub typ: VariableType,
  pub name: String,
  pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
  Constant,
  Mutable,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Number(f32),
  Nil,
  Bool(bool),
  String(String),
  List(Vec<Value>),
  Func(Func), // Only for functions (and maybe loops)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
  args: Vec<String>,
  body: Node,
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Value::Number(n) => {
        write!(f, "{}", n)?;
      }
      Value::String(s) => {
        write!(f, "{}", s)?;
      }
      Value::List(v) => {
        write!(f, "{:?}", v)?;
      }
      Value::Bool(b) => {
        write!(f, "{}", b)?;
      }
      _ => {
        write!(f, "nil")?;
      }
    }
    Ok(())
  }
}
