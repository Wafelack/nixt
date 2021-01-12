use crate::core::interpreter::interpreter::Interpreter;
use crate::utils::{element::*, node::*};

impl Interpreter {
  pub fn proc_fun_def(&self, val: &Node) -> Result<Value, String> {
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
  pub fn process_func_call(&mut self, function: &Node, args: &Vec<Value>) -> Result<(), String> {
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
}
