use crate::utils::element::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> Result<Value, String> {
  let start = SystemTime::now();
  let since_1970 = match start.duration_since(UNIX_EPOCH) {
    Ok(t) => t,
    Err(_) => return Err("Time went backwards !".to_owned()),
  };

  let toret = since_1970.as_secs_f32();

  Ok(Value::Number(toret))
}
