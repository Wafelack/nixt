use crate::utils::element::Value;
use std::path::Path;

pub fn exist(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Bool(false));
    }
    if let Value::String(name) = &args[0] {
        Ok(Value::Bool(Path::new(&name).exists()))
    } else {
        Ok(Value::Bool(false))
    }
}

pub fn read_dir(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Nil);
    }

    if let Value::String(name) = &args[0] {
            if Path::new(name).exists() {
                Ok(Value::List(see_dir(name)?))
            } else {
                Ok(Value::Nil)
            }
    } else {
        Ok(Value::Nil)
    }
}

use std::fs;

pub fn remove_dir(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Nil);
    }

    if let Value::String(name) = &args[0] {
        fs::remove_dir_all(name).map_err(|e| e.to_string())?;
        Ok(Value::Nil)
    } else {
        Ok(Value::Nil)
    }
}

pub fn remove_file(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Nil);
    }

    if let Value::String(name) = &args[0] {
        fs::remove_file(name).map_err(|e| e.to_string())?;
        Ok(Value::Nil)
    } else {
        Ok(Value::Nil)
    }
}

pub fn create_dir(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Nil);
    }

    if let Value::String(name) = &args[0] {
        fs::create_dir_all(name).map_err(|e| e.to_string())?;
        Ok(Value::Nil)
    } else {
        Ok(Value::Nil)
    }
}

pub fn create_file(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Nil);
    }

    if let Value::String(name) = &args[0] {
        fs::File::create(name).map_err(|e| e.to_string())?;
        Ok(Value::Nil)
    } else {
        Ok(Value::Nil)
    }
}

use std::io::prelude::*;

pub fn write_file(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 3 {
        return Ok(Value::Nil);
    }

    if let Value::String(name) = &args[0] {
        if let Value::String(mode) = &args[1] {

            let mut file = match mode.as_str() {
                "a" => {
                    fs::OpenOptions::new().append(true).open(name).map_err(|e| e.to_string())?
                }
                _ => {
                    fs::OpenOptions::new().write(true).open(name).map_err(|e| e.to_string())?
                },
            };

            let toret = match crate::stdlib::str::cat(&args[2..].to_vec())? {
                Value::String(s) => s,
                _ => panic!("src/stdlib/fs/fs::write_file - This should not be called, please report this in an issue"),
            };

            let toret = file.write(toret.as_bytes()).map_err(|e| e.to_string())?;

            Ok(Value::Number(toret as f32))
        } else {
            Ok(Value::Nil)
        }
    } else {
        Ok(Value::Nil)
    }
}

pub fn read_file(args: &Vec<Value>) -> Result<Value, String> {
    if args.len() < 1 {
        return Ok(Value::Nil);
    }
    if let Value::String(name) = &args[0] {
            Ok(Value::String(fs::read_to_string(name).map_err(|e| e.to_string())?))
    } else {
        Ok(Value::Nil)
    }
}

fn see_dir(name: &str) -> Result<Vec<Value>,String> {
    let mut toret = Vec::<Value>::new();

    let entries = fs::read_dir(name).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;

        if entry.path().is_dir() {
            toret.push(Value::String(entry.path().to_str().unwrap().to_owned()));
            let sub = see_dir(entry.path().to_str().unwrap())?;
            toret.extend_from_slice(&sub);
        } else {
            toret.push(Value::String(entry.path().to_str().unwrap().to_owned()));
        }
    }
    Ok(toret)
}