pub use crate::utils::node::Node;
#[derive(Debug)]
pub enum Value {
  String(String),
  Number(f32),
  List(Vec<Value>),
  Bool(bool),
  Func(Func),
  Nil,
}
#[derive(Debug)]
pub struct Func {
  pub args: Vec<String>,
  pub body: Node,
}
