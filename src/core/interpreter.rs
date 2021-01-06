use crate::utils::element::*;
use std::collections::BTreeMap;

pub struct Interpreter {
  ast: Node,
  scopes: Vec<BTreeMap<String, Value>>,
  errors: Vec<String>,
  had_error: bool,
}
