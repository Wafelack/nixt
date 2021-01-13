use crate::utils::element::Value;

pub fn assert(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() != 1 {
    return Err(format!(
      "Invalid arguments number: expected 1 found {}",
      args.len()
    ));
  }
  if let Value::Bool(b) = &args[0] {
    if *b {
      return Ok(Value::Nil);
    } else {
      return Err("Code panicked at assertion failed".to_owned());
    }
  }
  Ok(Value::Nil)
}
