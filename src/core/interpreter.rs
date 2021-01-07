use crate::utils::element::*;
use std::collections::BTreeMap;

pub struct Interpreter {
    ast: Node,
    scopes: Vec<BTreeMap<String, Value>>,
    errors: Vec<String>,
}

impl Interpreter {
    pub fn new(ast: Node) -> Self {
        Self {
            ast: ast,
            scopes: vec![],
            errors: vec![],
        }
    }
    pub fn add_scope(&mut self) {
        self.scopes.push(BTreeMap::new())
    }
    pub fn remove_scope(&mut self) {
        self.scopes.pop();
    }
}
