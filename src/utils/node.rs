#[derive(Debug, Clone)]
pub enum NodeType {
  Assignement(AssignType),
  Condition(ConditionType),
  Operator(Operator),
  Check(CheckType),
  Func,
  Loop,
  Return,
  Block,
  None,
  NodeNumber(f32),
  NodeStr(String),
  NodeIdentifier(String),
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
pub enum ConditionType {
  If,
  Else,
}

#[derive(Debug, Clone)]
pub enum Operator {
  Plus,
  Minus,
  Times,
  Div,
}
