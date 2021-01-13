use crate::core::interpreter::interpreter::Interpreter;
use crate::stdlib;
use crate::utils::{element::*, node::*};
use std::path::Path;

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
    Ok(toret.unwrap_or(Value::Nil))
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

    let processed = process_std(&fname, &as_value);

    if processed.0 {
      return processed.1;
    } else if &fname == &"import" {
      return self.process_import(&as_value);
    } else {
      return self.process_func_call(&func, &as_value);
    }
  }
  pub fn process_import(&mut self, to_import: &Vec<Value>) -> Result<Value, String> {
    let mut import_in_ast = |fname: &str| -> Result<(), String> {
      let code = match std::fs::read_to_string(fname) {
        Ok(c) => c,
        Err(_) => return Err("Failed to read module code".to_owned()),
      };

      let mut lexer = crate::core::lexer::Lexer::new(&code);
      let tokens = lexer.scan_tokens();
      if lexer.get_errors().is_some() {
        return Err("The imported file contains lexing errors. Aborting".to_owned());
      }
      let mut parser = crate::core::parser::Parser::new(tokens);
      let ast = parser.parse();
      if parser.get_errors().is_some() {
        return Err("The imported file contains parsing errors. Aborting".to_owned());
      }
      self.process_ast(&ast)?;

      Ok(())
    };
    for val in to_import {
      if let Value::String(s) = val {
        if Path::new(s).exists() {
          import_in_ast(&s)?;
        } else {
          if s.starts_with("std/") {
            let folder = match std::env::var("NIXT_STD") {
              Ok(res) => res,
              _ => return Err("Could not find NIXT_STD environnement variable. Consider creating it to import std modules.".to_owned()),
            };

            let to_import = format!("{}/{}.nxt", folder, &s[4..]);
            if !Path::new(&to_import).exists() {
              return Err(format!("Could not find standard module `{}`", &s[4..]));
            }

            import_in_ast(&to_import)?;
          } else {
            return Err(format!("Unresolved import: `{}`", s));
          }
        }
      } else {
        continue;
      }
    }

    Ok(Value::Nil)
  }
}

fn process_std(name: &str, args: &Vec<Value>) -> (bool, Result<Value, String>) {
  let mut found = true;

  let toret = if &name == &"print" {
    stdlib::io::print(&args)
  } else if &name == &"puts" {
    stdlib::io::puts(&args)
  } else if &name == &"time:now" {
    stdlib::time::now()
  } else if &name == &"str:cat" {
    stdlib::str::cat(args)
  } else if &name == &"list" {
    stdlib::list::list(args)
  } else if &name == &"pop" {
    stdlib::list::pop(args)
  } else if &name == &"assert" {
    stdlib::assert::assert(args)
  } else {
    found = false;
    Ok(Value::Nil)
  };

  (found, toret)
}
