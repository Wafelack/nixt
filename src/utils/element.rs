pub use crate::utils::node::Node;
#[derive(Debug, Clone)]
pub enum Value {
  String(String),
  Number(f32),
  List(Vec<Value>),
  Bool(bool),
  Func(Func),
  Nil,
}
#[derive(Debug, Clone)]
pub struct Func {
  pub args: Vec<String>,
  pub body: Node,
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::String(s) => write!(f, "{}", s)?,
      Self::Number(n) => write!(f, "{}", n)?,
      Self::List(l) => {
        write!(f, "[")?;
        for i in 0..l.len() {
          if i + 1 < l.len() {
            write!(f, "{}, ", l[i]);
          } else {
            write!(f, "{}", l[i]);
          }
        }
        write!(f, "]")?;
      }
      Self::Bool(b) => write!(f, "{}", b)?,
      Self::Func(fnc) => write!(f, "{}", fnc)?,
      Self::Nil => write!(f, "nil")?,
    }
    Ok(())
  }
}

impl std::fmt::Display for Func {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "@args: {:?}\n@body: {}", self.args, self.body)?;
    Ok(())
  }
}
