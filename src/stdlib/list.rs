use crate::utils::element::Value;

pub fn list(args: &Vec<Value>) -> Result<Value, String> {
  Ok(Value::List(args.to_owned()))
}

pub fn pop(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }
  if let Value::List(l) = &args[0] {
    return Ok(Value::List((&l[..(l.len() - 1)]).to_owned()));
  } else if let Value::String(s) = &args[0] {
    return Ok(Value::String((&s[..(s.len() - 1)]).to_owned()));
  }

  Ok(Value::Nil)
}
