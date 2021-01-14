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
  Scope,
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
    Modulo
}

pub fn stringify(node: &Node, indentations: usize) -> String {
  let mut toret = String::new();
  toret.push_str("{\n");
  for children in node.get_child() {
    toret.push_str(&format!("{}@type : ", gen_indents(indentations)));
    toret.push_str(&format!("{:?}\n", children.get_type()));
    toret.push_str(&format!("{}@children : ", gen_indents(indentations)));
    toret.push_str(&stringify(&children, indentations + 1));
  }
  toret.push_str(&format!("{}}}\n", gen_indents(indentations)));
  toret
}

fn gen_indents(amount: usize) -> String {
  let mut toret = String::new();
  for _ in 0..amount {
    toret.push_str("  ");
  }
  toret
}

impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", stringify(self, 0))?;
    Ok(())
  }
}
