use crate::utils::element::Value;

pub fn cos(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.cos()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn acos(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.acos()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn sin(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.sin()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn asin(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.asin()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn tan(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.tan()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn atan(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }

  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.atan()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn floor(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }
  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.floor()))
  } else {
    Ok(Value::Nil)
  }
}

pub fn ceil(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 1 {
    return Ok(Value::Nil);
  }
  if let Value::Number(n) = args[0] {
    Ok(Value::Number(n.ceil()))
  } else {
    Ok(Value::Nil)
  }
}
