use crate::utils::element::Value;
use std::io::Write;

pub fn print(to_display: &Vec<Value>) {
  for val in to_display {
    print!("{}", val);
  }
  println!();
}

pub fn puts(to_display: &Vec<Value>) {
  for val in to_display {
    print!("{}", val);
  }
  std::io::stdout().flush().unwrap();
}
