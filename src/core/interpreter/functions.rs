use crate::core::interpreter::interpreter::Interpreter;
use crate::stdlib;
use crate::utils::{element::*, node::*};

impl Interpreter {
  pub fn proc_fun_def(&mut self, val: &Node) -> Result<Value, String> {
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
  pub fn process_func_call(&mut self, function: &Node, args: &Vec<Value>) -> Result<Value, String> {
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
    let toret = self.process_node(&body)?;
    self.remove_scope();
    Ok(toret) // Temporary, I will implement return later
  }

  pub fn process_func(&mut self, func: &Node) -> Result<Value, String> {
    let children = &func.get_child();
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
            return Err("Attempted to use an undefined variable: `{}`, s".to_owned());
          }
        }
        NodeType::FunctionCall(_) => self.process_func(child)?,
        x => return Err(format!("Unexpected value: {:?}", x)),
      };
      as_value.push(topsh);
    }
    let fname = if let NodeType::FunctionCall(name) = func.get_type() {
      // Should always be true
      name
    } else {
      return Err("Invalid function call".to_owned());
    };

    if &fname == &"print" {
      return stdlib::io::print(&as_value);
    } else if &fname == &"puts" {
      return stdlib::io::puts(&as_value);
    } else {
      return self.process_func_call(&func, &as_value);
    }
  }
}
