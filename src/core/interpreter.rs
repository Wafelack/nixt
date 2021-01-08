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
    fn var_def(&mut self, is_const: bool, name: &Node, value: &Node) {
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
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name, (value, is_const));
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
        match val.get_type() {
            NodeType::NodeNumber(n) => return Value::Number(n),
            NodeType::NodeStr(s) => return Value::String(s),
            NodeType::NodeBool(b) => return Value::Bool(b),
            NodeType::Block => return self.process_inner_block(&val),
            NodeType::NodeIdentifier(s) => {
                return self
                    .get_value(s)
                    .unwrap_or_else(|| panic!("ERROR: Attempted to access an undefined variable"))
            }
            _ => return Value::Nil,
        }
    }
    fn get_value(&mut self, value: String) -> Option<Value> {
        for i in (0..self.scopes.len()).rev() {
            let scope = &self.scopes[i];
            if scope.contains_key(&value) {
                return Some((scope[&value].0).clone());
            }
        }
        None
    }
    fn process_inner_block(&mut self, val: &Node) -> Value {
        if val.get_child().len() < 1 {
            return Value::Nil;
        }

        match val.get_child()[0].get_type() {
            NodeType::Func => self.proc_fun_def(&val.get_child()[0]),
            NodeType::Operator(op) => todo!(),
            NodeType::Block => self.process_inner_block(val),
            _ => Value::Nil,
        }
    }
    fn process_node(&mut self, node: &Node) {
        for instruction in node.get_child() {
            if instruction.get_type() == NodeType::Scope {
                self.add_scope();
                self.process_node(&instruction);
                print!("{} -> ", self.scopes.len() - 1,);
                display_scope(&self.scopes[self.scopes.len() - 1]);
                self.remove_scope();
            } else if instruction.get_type() == NodeType::Block {
                if self.scopes.len() == 0 {
                    panic!("ERROR: No scopes available. Consider adding a scope to your program");
                }
                self.process_node(&instruction);
            } else {
                if self.scopes.len() == 0 {
                    panic!("ERROR: No scopes available. Consider adding a scope to your program");
                }
                let t = instruction.get_type();
                let children = instruction.get_child();

                if let NodeType::Assignement(a) = t {
                    if a == AssignType::Set {
                        todo!()
                    } else {
                        self.var_def(
                            if a == AssignType::Const { true } else { false },
                            &children[0],
                            &children[1],
                        );
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
