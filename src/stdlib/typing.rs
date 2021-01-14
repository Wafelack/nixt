use crate::utils::element::Value;

pub fn r#type_of(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  let toret = if let Value::String(_) = args[0] {
    "string"
  } else if let Value::Bool(_) = args[0] {
    "bool"
  } else if let Value::Func(_) = args[0] {
    "function"
  } else if let Value::List(_) = args[0] {
    "list"
  } else if let Value::Number(_) = args[0] {
    "number"
  } else {
    "nil"
  };

  Ok(Value::String(toret.to_owned()))
}
