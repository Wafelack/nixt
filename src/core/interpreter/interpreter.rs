use crate::utils::element::*;
use crate::utils::node::*;
use std::collections::BTreeMap;

pub fn is_defined(scope: &BTreeMap<String, (Value, bool)>, name: &String) -> bool {
  if scope.contains_key(name) {
    return true;
  }
  false
}

pub struct Interpreter {
  pub scopes: Vec<BTreeMap<String, (Value, bool)>>,
}

impl Interpreter {
  pub fn new(ast: Option<&Node>) -> Result<Interpreter, String> {
    let toret = if ast.is_some() {
      let mut interpreter = Interpreter { scopes: vec![] };
      interpreter.process_node(&ast.unwrap())?;

      interpreter
    } else {
      let interpreter = Interpreter {
        scopes: vec![BTreeMap::new()],
      };

      interpreter
    };
    Ok(toret)
  }
  pub fn add_scope(&mut self) {
    self.scopes.push(BTreeMap::new())
  }
  pub fn remove_scope(&mut self) {
    self.scopes.pop();
  }
  pub fn var_edit(&mut self, name: &Node, new_val: &Node) -> Result<(), String> {
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
  pub fn var_def(&mut self, is_const: bool, name: &Node, value: &Node) -> Result<(), String> {
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
  pub fn proc_value(&mut self, val: &Node) -> Result<Value, String> {
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
  pub fn get_value(&mut self, value: &String) -> Option<Value> {
    for i in (0..self.scopes.len()).rev() {
      let scope = &self.scopes[i];
      if scope.contains_key(value) {
        return Some((scope[value].0).clone());
      }
    }
    None
  }

  pub fn eval_condition(&mut self, cdn: &Node) -> Result<bool, String> {
    let t = if let NodeType::Operator(op) = cdn.get_type() {
      op
    } else {
      return Err(format!("Found invalid operator: {}", cdn));
    };
    let processed = self.proc_operator(t, cdn)?;
    let you_have_been_banboolzled = if let Value::Bool(b) = processed {
      b
    } else {
      return Err("Attempted to use a non-boolean value as a condition".to_owned());
    };
    Ok(you_have_been_banboolzled)
  }

  pub fn process_ast(&mut self, ast: &Node) -> Result<(), String> {
    self.process_node(ast)?;
    Ok(())
  }
}

pub fn display_scope(scope: &BTreeMap<String, (Value, bool)>) {
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
