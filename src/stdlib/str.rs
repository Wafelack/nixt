use crate::utils::element::Value;

pub fn cat(args: &Vec<Value>) -> Result<Value, String> {
  let mut toret = String::new();

  for arg in args {
    if let Value::String(s) = arg {
      toret.push_str(&s);
    } else if let Value::Number(f) = arg {
      toret.push_str(&format!("{}", f));
    } else if let Value::Bool(b) = arg {
      toret.push_str(&format!("{}", b));
    } else if let Value::Nil = arg {
      toret.push_str("nil");
    }
  }

  Ok(Value::String(toret))
}
