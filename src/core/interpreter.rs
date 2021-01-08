use crate::utils::element::*;
use crate::utils::node::*;
use std::collections::BTreeMap;

fn is_defined(scope: &BTreeMap<String, (Value, bool)>, name: &String) -> bool {
  if scope.contains_key(name) {
    return true;
  }
  false
}

pub struct Interpreter {
  ast: Node,
  scopes: Vec<BTreeMap<String, (Value, bool)>>,
}

impl Interpreter {
  fn add_scope(&mut self) {
    self.scopes.push(BTreeMap::new())
  }
  fn remove_scope(&mut self) {
    self.scopes.pop();
  }
  fn var_edit(&mut self, name: &Node, new_val: &Node) -> Result<(), String> {
    if self.scopes.len() == 0 {
      return Err("No scopes available. Consider adding a scope to your program".to_owned());
    }
    let scope = &self.scopes[self.scopes.len() - 1];
    let name = if let NodeType::NodeIdentifier(s) = name.get_type() {
      s
    } else {
      return Err("Found an invalid identifier in variable edition".to_owned());
      // Should never be called because parser checks
    };

    if !is_defined(&scope, &name) {
      return Err("Attempted to redefine an undefined variable".to_owned());
    }

    if scope[&name].1 {
      return Err("Attempted to redefine a constant".to_owned());
    }

    let new_val_valued = self.proc_value(new_val)?;

    if let Some(x) = self.scopes.last_mut().unwrap().get_mut(&name) {
      *x = (new_val_valued, false)
    }

    Ok(())
  }
  fn var_def(&mut self, is_const: bool, name: &Node, value: &Node) -> Result<(), String> {
    if self.scopes.len() == 0 {
      return Err("No scopes available. Consider adding a scope to your program".to_owned());
    }
    let scope = &self.scopes[self.scopes.len() - 1];
    let name = if let NodeType::NodeIdentifier(s) = name.get_type() {
      s
    } else {
      return Err("Found an invalid identifier in variable declaration".to_owned());
      // Should never be called because parser checks
    };

    if is_defined(&scope, &name) {
      return Err(format!(
        "Attempted to redefine variable `{}` that is already present in the current scope",
        &name
      ));
    }
    let value = self.proc_value(value)?;
    self
      .scopes
      .last_mut()
      .unwrap()
      .insert(name, (value, is_const));
    Ok(())
  }
  fn proc_fun_def(&self, val: &Node) -> Result<Value, String> {
    let mut argstr = Vec::<String>::new();

    let args = val.get_child()[0].get_child();

    for arg in &args {
      if let NodeType::NodeIdentifier(s) = arg.get_type() {
        argstr.push(s);
      } else {
        return Err("Invalid argument in function declaration".to_owned());
        // Should never be called because parser checks before
      }
    }

    Ok(Value::Func(Func {
      args: argstr,
      body: val.get_child()[1].clone(),
    }))
  }
  fn proc_value(&self, val: &Node) -> Result<Value, String> {
    match val.get_type() {
      NodeType::NodeNumber(n) => return Ok(Value::Number(n)),
      NodeType::NodeStr(s) => return Ok(Value::String(s)),
      NodeType::NodeBool(b) => return Ok(Value::Bool(b)),
      NodeType::Block => return Ok(self.process_inner_block(&val)?),
      NodeType::NodeIdentifier(s) => {
        return if self.get_value(&s).is_some() {
          Ok(self.get_value(&s).unwrap())
        } else {
          Err("Attempted to access an undefined variable".to_owned())
        }
      }
      _ => return Ok(Value::Nil),
    }
  }
  fn get_value(&self, value: &String) -> Option<Value> {
    for i in (0..self.scopes.len()).rev() {
      let scope = &self.scopes[i];
      if scope.contains_key(value) {
        return Some((scope[value].0).clone());
      }
    }
    None
  }
  fn process_inner_block(&self, val: &Node) -> Result<Value, String> {
    if val.get_child().len() < 1 {
      return Ok(Value::Nil);
    }

    match val.get_child()[0].get_type() {
      NodeType::Func => Ok(self.proc_fun_def(&val.get_child()[0])?),
      NodeType::Operator(op) => self.proc_operator(op, val),
      NodeType::Block => Ok(self.process_inner_block(val)?),
      _ => Ok(Value::Nil),
    }
  }
  fn proc_operator(&self, op: OperatorType, val: &Node) -> Result<Value, String> {
    match op {
      OperatorType::Div => Ok(self.div(&val.get_child()[0], &val.get_child()[1])?),
      OperatorType::Times => Ok(self.mul(&val.get_child()[0], &val.get_child()[1])?),
      OperatorType::Plus => Ok(self.add(&val.get_child()[0], &val.get_child()[1])?),
      OperatorType::Minus => Ok(self.sub(&val.get_child()[0], &val.get_child()[1])?),
      OperatorType::Modulo => Ok(self.mod(&val.get_child()[0], &val.get_child()[1])?),
    }
  }

  fn div(&self, lhs: &Node, rhs: &Node) -> Result<Value, String> {
    match lhs.get_type() {
      NodeType::NodeNumber(lh) => match rhs.get_type() {
        NodeType::NodeNumber(rh) => Ok(Value::Number(lh/rh)),
        _ => Ok(Value::Nil),
      }
      _ => Ok(Value::Nil),
    }
  }
  fn mul(&self, lhs: &Node, rhs: &Node) -> Result<Value, String> {
    match lhs.get_type() {
      NodeType::NodeNumber(lh) => match rhs.get_type() {
        NodeType::NodeNumber(rh) => Ok(Value::Number(lh*rh)),
        _ => Ok(Value::Nil),
      }
      _ => Ok(Value::Nil),
    }
  }
  fn sub(&self, lhs: &Node, rhs: &Node) -> Result<Value, String> {
    match lhs.get_type() {
      NodeType::NodeNumber(lh) => match rhs.get_type() {
        NodeType::NodeNumber(rh) => Ok(Value::Number(lh-rh)),
        _ => Ok(Value::Nil),
      }
      _ => Ok(Value::Nil),
    }
  }
  fn add(&self, lhs: &Node, rhs: &Node) -> Result<Value, String> {
    match lhs.get_type() {
      NodeType::NodeNumber(lh) => match rhs.get_type() {
        NodeType::NodeNumber(rh) => Ok(Value::Number(lh+rh)),
        _ => Ok(Value::Nil),
      }
      _ => Ok(Value::Nil),
    }
  }
  fn mod(&self, lhs: &Node, rhs: &Node) -> Result<Value, String> {
    match lhs.get_type() {
      NodeType::NodeNumber(lh) => match rhs.get_type() {
        NodeType::NodeNumber(rh) => Ok(Value::Number(lh%rh)),
        _ => Ok(Value::Nil),
      }
      _ => Ok(Value::Nil),
    }
  }
  fn process_node(&mut self, node: &Node) -> Result<(), String> {
    for instruction in node.get_child() {
      if instruction.get_type() == NodeType::Scope {
        self.add_scope();
        self.process_node(&instruction)?;
        print!("{} -> ", self.scopes.len() - 1,); // DEBUG
        display_scope(&self.scopes[self.scopes.len() - 1]); // DEBUG
        self.remove_scope();
      } else if instruction.get_type() == NodeType::Block {
        if self.scopes.len() == 0 {
          return Err("No scopes available. Consider adding a scope to your program".to_owned());
        }
        self.process_node(&instruction)?;
      } else {
        if self.scopes.len() == 0 {
          return Err("No scopes available. Consider adding a scope to your program".to_owned());
        }
        let t = instruction.get_type();
        let children = instruction.get_child();

        if let NodeType::Assignement(a) = t {
          if a == AssignType::Set {
            self.var_edit(&children[0], &children[1])?;
          } else {
            self.var_def(
              if a == AssignType::Const { true } else { false },
              &children[0],
              &children[1],
            )?;
          }
        }
      }
    }
    Ok(())
  }
  pub fn run(ast: Node) -> Result<(), String> {
    let mut interpreter = Self {
      ast: ast.clone(),
      scopes: vec![],
    };
    interpreter.process_node(&ast)
  }
}

fn display_scope(scope: &BTreeMap<String, (Value, bool)>) {
  println!("{{");
  for (key, (value, is_const)) in scope {
    println!(
      "{}: {} ({})",
      key,
      value,
      if *is_const { "const" } else { "mutable" }
    );
  }
  println!("}}");
}
