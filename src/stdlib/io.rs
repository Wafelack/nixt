use crate::utils::element::Value;
use std::io::Write;

pub fn print(to_display: &Vec<Value>) -> Result<Value, String> {
  for val in to_display {
    print!("{}", val);
  }
  println!();
  Ok(Value::Nil)
}

pub fn puts(to_display: &Vec<Value>) -> Result<Value, String> {
  for val in to_display {
    print!("{}", val);
  }
  Ok(Value::Nil)
}

pub fn input(args: &Vec<Value>) -> Result<Value, String> {
  let prompt = if args.len() < 1 {
    ""
  } else {
    if let Value::String(s) = &args[0] {
      s.as_str()
    } else {
      ""
    }
  };

  print!("{}", prompt);
  let mut toret = String::new();
  std::io::stdout().flush().unwrap();
  std::io::stdin().read_line(&mut toret).unwrap();
  Ok(Value::String(toret.trim().to_owned()))

}