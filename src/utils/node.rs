#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
  Assignement(AssignType),
  Condition,
  Operator(OperatorType),
  Func,
  Loop,
  Return,
  Block,
  None,
  NodeBool(bool),
  NodeNumber(f32),
  NodeStr(String),
  NodeIdentifier(String),
  FunctionCall(String),
}
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
  node_type: NodeType,
  child: Vec<Node>,
}

impl Node {
  pub fn new(t: NodeType) -> Self {
    Self {
      node_type: t,
      child: vec![],
    }
  }
  pub fn add_children(&mut self, c: &Node) {
    self.child.push((*c).clone());
  }
  pub fn get_child(&self) -> Vec<Node> {
    self.child.clone()
  }
  pub fn get_type(&self) -> NodeType {
    self.node_type.clone()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignType {
  Let,
  Const,
  Set,
}
#[derive(Debug, Clone, PartialEq)]
pub enum OperatorType {
  Plus,
  Minus,
  Times,
  Div,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equal,
  NotEqual,
  And,
  Or,
}
