pub use crate::utils::node::Node;

pub enum Value {
  String(String),
  Number(f32),
  List(Vec<Value>),
  Bool(bool),
  Func(Func),
}

pub struct Func {
  args: Vec<String>,
  body: Node,
}
