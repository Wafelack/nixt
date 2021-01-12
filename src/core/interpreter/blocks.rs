use crate::core::interpreter::interpreter::*;
use crate::stdlib;
use crate::utils::{element::*, node::*};

impl Interpreter {
  pub fn process_node(&mut self, node: &Node) -> Result<(), String> {
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
          self.process_func(&instruction);
        } else if let NodeType::Loop = t {
          self.process_loop(&instruction)?;
        } else if let NodeType::Condition = t {
          self.process_if(&instruction)?;
        }
      }
    }
    Ok(())
  }
  pub fn process_inner_block(&self, val: &Node) -> Result<Value, String> {
    if val.get_child().len() < 1 {
      return Ok(Value::Nil);
    }

    match val.get_child()[0].get_type() {
      NodeType::Func => Ok(self.proc_fun_def(&val.get_child()[0])?),
      NodeType::Operator(op) => self.proc_operator(op, &val.get_child()[0]),
      NodeType::Block => Ok(self.process_inner_block(&val.get_child()[0])?),
      _ => Ok(Value::Nil),
    }
  }
}
