use crate::utils::element::*;
use crate::utils::node::*;
use std::collections::BTreeMap;

fn is_defined(scope: &BTreeMap<String, Value>, name: &String) -> bool {
    if scope.contains_key(name) {
        return true;
    }
    false
}

pub struct Interpreter {
    ast: Node,
    scopes: Vec<BTreeMap<String, Value>>,
}

impl Interpreter {
    fn add_scope(&mut self) {
        self.scopes.push(BTreeMap::new())
    }
    fn remove_scope(&mut self) {
        self.scopes.pop();
    }
    fn var_def(&mut self, name: &Node, value: &Node) {
        if self.scopes.len() == 0 {
            panic!("ERROR: No scopes available. Consider adding a scope to your program");
        }
        let scope = &self.scopes[self.scopes.len() - 1];
        let name = if let NodeType::NodeIdentifier(s) = name.get_type() {
            s
        } else {
            panic!("ERROR: Found an invalid identifier in variable declaration");
        };

        if is_defined(&scope, &name) {
            panic!("ERROR: Attempted to redefine variable `{}` that is already present in the current scope", &name);
        }
        let value = self.proc_value(value);
        self.scopes.last_mut().unwrap().insert(name, value);
    }
    fn proc_fun_def(&mut self, val: &Node) -> Value {
        let mut argstr = Vec::<String>::new();

        let args = val.get_child()[0].get_child();

        for arg in &args {
            if let NodeType::NodeIdentifier(s) = arg.get_type() {
                argstr.push(s);
            } else {
                panic!("ERROR: Invalid argument in function declaration");
            }
        }

        Value::Func(Func {
            args: argstr,
            body: val.get_child()[1].clone(),
        })
    }
    fn proc_value(&mut self, val: &Node) -> Value {
        if val.get_type() == NodeType::Func {
            return self.proc_fun_def(val);
        } else {
            match val.get_type() {
                NodeType::NodeNumber(n) => return Value::Number(n),
                NodeType::NodeStr(s) => return Value::String(s),
                NodeType::NodeBool(b) => return Value::Bool(b),
                _ => return Value::Nil,
            }
        }
    }
    fn process_node(&mut self, node: &Node) {
        for instruction in node.get_child() {
            if instruction.get_type() == NodeType::Scope {
                self.add_scope();
                self.process_node(&instruction);
                println!(
                    "Current scope: {} -> {:?}",
                    self.scopes.len() - 1,
                    self.scopes
                );
                self.remove_scope();
            } else if instruction.get_type() == NodeType::Block {
                self.process_node(&instruction);
            } else {
                let t = instruction.get_type();
                let children = instruction.get_child();

                if let NodeType::Assignement(a) = t {
                    if a == AssignType::Let {
                        self.var_def(&children[0], &children[1]);
                    }
                }
            }
        }
    }
    pub fn run(ast: Node) {
        let mut interpreter = Self {
            ast: ast.clone(),
            scopes: vec![],
        };
        interpreter.process_node(&ast);
    }
}
