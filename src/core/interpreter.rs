use crate::stdlib;
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
      NodeType::Operator(op) => self.proc_operator(op, &val.get_child()[0]),
      NodeType::Block => Ok(self.process_inner_block(val)?),
      _ => Ok(Value::Nil),
    }
  }
  fn proc_operator(&self, op: OperatorType, val: &Node) -> Result<Value, String> {
    let lhs = match val.get_child()[0].get_type() {
      NodeType::Block => self.process_inner_block(&val.get_child()[0])?,
      NodeType::NodeNumber(n) => Value::Number(n),
      NodeType::NodeStr(s) => Value::String(s),
      NodeType::NodeBool(b) => Value::Bool(b),
      NodeType::None => Value::Nil,
      NodeType::NodeIdentifier(s) => {
        if self.get_value(&s).is_some() {
          self.get_value(&s).unwrap()
        } else {
          return Err("Attempted to access an undefined variable".to_owned());
        }
      }
      _ => return Err("Invalid element".to_owned()),
    };

    let rhs = match val.get_child()[1].get_type() {
      NodeType::Block => self.process_inner_block(&val.get_child()[1])?,
      NodeType::NodeNumber(n) => Value::Number(n),
      NodeType::NodeStr(s) => Value::String(s),
      NodeType::NodeBool(b) => Value::Bool(b),
      NodeType::None => Value::Nil,
      NodeType::NodeIdentifier(s) => {
        if self.get_value(&s).is_some() {
          self.get_value(&s).unwrap()
        } else {
          return Err("Attempted to access an undefined variable".to_owned());
        }
      }
      _ => return Err("Invalid element".to_owned()),
    };

    let toret = match op {
      OperatorType::Div => self.div(lhs, rhs)?,
      OperatorType::Times => self.mul(lhs, rhs)?,
      OperatorType::Plus => self.add(lhs, rhs)?,
      OperatorType::Minus => self.sub(lhs, rhs)?,
      OperatorType::Modulo => self.modulo(lhs, rhs)?,
      OperatorType::Equal => self.eq(lhs, rhs)?,
      OperatorType::NotEqual => self.neq(lhs, rhs)?,
      OperatorType::LessEqual => self.leq(lhs, rhs)?,
      OperatorType::Less => self.le(lhs, rhs)?,
      OperatorType::Greater => self.ge(lhs, rhs)?,
      OperatorType::GreaterEqual => self.ge(lhs, rhs)?,
      OperatorType::And => self.and(lhs, rhs)?,
      OperatorType::Or => self.or(lhs, rhs)?,
      _ => return Err("Invalid operator".to_owned()),
    };
    Ok(toret)
  }

  fn process_loop(&mut self, master: &Node) -> Result<(), String> {
    let raw_condition = &master.get_child()[0].get_child()[0];

    while self.eval_condition(raw_condition)? {
      self.process_node(&master.get_child()[1])?;
    }

    Ok(())
  }

  fn process_if(&mut self, master: &Node) -> Result<(), String> {
    let raw_condition = &master.get_child()[0].get_child()[0];

    if self.eval_condition(raw_condition)? {
      self.process_node(&master.get_child()[1])?;
    } else {
      if &master.get_child()[2].get_type() == &NodeType::None {
        return Ok(());
      } else {
        self.process_node(&master.get_child()[2])?;
      }
    }

    Ok(())
  }

  fn eval_condition(&self, cdn: &Node) -> Result<bool, String> {
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

  fn and(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(lh && rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  fn or(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(lh || rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  fn eq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(rh == lh)),
        _ => Ok(Value::Bool(false)),
      },
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(rh == lh)),
        _ => Ok(Value::Bool(false)),
      },
      Value::String(lh) => match rhs {
        Value::String(rh) => Ok(Value::Bool(rh == lh)),
        _ => Ok(Value::Bool(false)),
      },
      Value::Nil => match rhs {
        Value::Nil => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
      },
      _ => Ok(Value::Bool(false)),
    }
  }

  fn neq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(rh != lh)),
        _ => Ok(Value::Bool(true)),
      },
      Value::Bool(lh) => match rhs {
        Value::Bool(rh) => Ok(Value::Bool(rh != lh)),
        _ => Ok(Value::Bool(true)),
      },
      Value::String(lh) => match rhs {
        Value::String(rh) => Ok(Value::Bool(rh != lh)),
        _ => Ok(Value::Bool(true)),
      },
      Value::Nil => match rhs {
        Value::Nil => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
      },
      _ => Ok(Value::Bool(true)),
    }
  }

  fn leq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh <= rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  fn le(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh < rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  fn geq(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh >= rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  fn ge(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Bool(lh > rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }

  fn div(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh / rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  fn mul(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh * rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  fn sub(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh - rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  fn add(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh + rh)),
        _ => Ok(Value::Nil),
      },
      _ => Ok(Value::Nil),
    }
  }
  fn modulo(&self, lhs: Value, rhs: Value) -> Result<Value, String> {
    match lhs {
      Value::Number(lh) => match rhs {
        Value::Number(rh) => Ok(Value::Number(lh % rh)),
        _ => Ok(Value::Nil),
      },
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
        } else if let NodeType::FunctionCall(func) = t {
          let mut as_value = vec![];
          for child in children {
            let topsh = match child.get_type() {
              NodeType::Block => self.process_inner_block(&child)?,
              NodeType::NodeBool(b) => Value::Bool(b),
              NodeType::NodeNumber(n) => Value::Number(n),
              NodeType::NodeStr(s) => Value::String(s),
              NodeType::None => Value::Nil,
              NodeType::NodeIdentifier(s) => {
                if self.get_value(&s).is_some() {
                  self.get_value(&s).unwrap()
                } else {
                  return Err("Attempted to use an undefined function: `{}`, s".to_owned());
                }
              }
              _ => return Err("Unexpected value".to_owned()),
            };
            as_value.push(topsh);
          }
          if &func == "print" {
            stdlib::io::print(&as_value);
          } else if &func == "puts" {
            stdlib::io::puts(&as_value);
          } else {
            self.process_func_call(&instruction, &as_value)?;
          }
        } else if let NodeType::Loop = t {
          self.process_loop(&instruction)?;
        } else if let NodeType::Condition = t {
          self.process_if(&instruction)?;
        }
      }
    }
    Ok(())
  }
  fn process_func_call(&mut self, function: &Node, args: &Vec<Value>) -> Result<(), String> {
    let (func_args, body) = if let NodeType::FunctionCall(s) = function.get_type() {
      if self.get_value(&s).is_some() {
        let raw_func = self.get_value(&s).unwrap();

        if let Value::Func(fnc) = raw_func {
          let args = fnc.args;
          let body = fnc.body;

          (args, body)
        } else {
          return Err("Attempted to call a regular variable as a function".to_owned());
        }
      } else {
        return Err("Attempted to call an undefined function".to_owned());
      }
    } else {
      panic!("This should not be called, if you see this, please open an issue.");
    };
    if args.len() != func_args.len() {
      return Err(format!(
        "Invalid number of arguments: expected {} got {}",
        func_args.len(),
        args.len()
      ));
    }
    self.add_scope();
    for i in 0..args.len() {
      self
        .scopes
        .last_mut()
        .unwrap()
        .insert(func_args[i].clone(), (args[i].clone(), false));
    }
    self.process_node(&body)?;
    self.remove_scope();
    Ok(())
  }
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
  pub fn process_ast(&mut self, ast: &Node) -> Result<(), String> {
    self.process_node(ast)
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
