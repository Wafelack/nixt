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

pub fn push(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 2 {
    return Ok(Value::Nil);
  }
  if let Value::List(l) = &args[0] {
    let mut toret = l.clone();
    let listed = list(
      &args
        .iter()
        .skip(1)
        .map(|x| x.to_owned())
        .collect::<Vec<Value>>(),
    )?;
    if let Value::List(to_add) = listed {
      toret.extend(to_add);
      return Ok(Value::List(toret));
    } else {
      return Ok(args[0].to_owned());
    }
  } else if let Value::String(s) = &args[0] {
    let mut toret = s.clone();
    let listed = crate::stdlib::str::cat(
      &args
        .iter()
        .skip(1)
        .map(|x| x.to_owned())
        .collect::<Vec<Value>>(),
    )?;
    if let Value::String(to_add) = listed {
      toret.push_str(&to_add);
      return Ok(Value::String(toret));
    } else {
      return Ok(args[0].to_owned());
    }
  }

  Ok(Value::Nil)
}

pub fn index(args: &Vec<Value>) -> Result<Value, String> {
  if args.len() < 2 {
    return Ok(Value::Nil);
  }

  if let Value::String(s) = &args[0] {
    if let Value::Number(n) = args[1] {
      if n.floor() as usize >= s.len() {
        return Ok(Value::Nil);
      } else {
        return Ok(Value::String(format!(
          "{}",
          s.chars().collect::<Vec<char>>()[n.floor() as usize]
        )));
      }
    } else {
      return Ok(args[0].to_owned());
    }
  } else if let Value::List(l) = &args[0] {
    if let Value::Number(n) = args[1] {
      if n.floor() as usize >= l.len() {
        return Ok(Value::Nil);
      } else {
        return Ok(l[n.floor() as usize].to_owned());
      }
    } else {
      return Ok(args[0].to_owned());
    }
  } else {
    return Ok(args[0].to_owned());
  }
}
