#[derive(Debug, Clone)]
pub enum NodeType {
  Assignement(AssignType),
  Condition,
  Operator(OperatorType),
  Check(CheckType),
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum CheckType {
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  Equal,
  NotEqual,
  And,
  Or,
}

#[derive(Debug, Clone)]
pub enum AssignType {
  Let,
  Const,
  Set,
}
#[derive(Debug, Clone)]
pub enum OperatorType {
  Plus,
  Minus,
  Times,
  Div,
}
