mod core;
mod stdlib;
mod tests;
mod utils;

use self::core::interpreter::interpreter::*;
use self::core::lexer::*;
use self::core::parser::*;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        repl()?;
    }
    if !Path::new(&args[1]).exists() {
        return Err("File not found".to_owned());
    }

    let content = match fs::read_to_string(&args[1]) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };
    let mut lexer = Lexer::new(&content);
    let toks = lexer.scan_tokens();
    let errs_lex = lexer.get_errors();
    if *(&errs_lex.is_some()) {
        let err_unwraped = &errs_lex.unwrap();
        for error in err_unwraped {
            println!("{}", error);
        }
        panic!("{} lexing errors occured !", err_unwraped.len());
    }
    let mut parser = Parser::new(toks);
    let ast = parser.parse();
    let errs = parser.get_errors();
    if errs.is_some() {
        let err_unwraped = &errs.unwrap();
        for error in err_unwraped {
            println!("{}", error);
        }
        panic!("{} parsing errors occured !", err_unwraped.len());
    }
    Interpreter::new(Some(&ast))?;

    Ok(())
}

fn repl() -> Result<(), String> {
    let mut interpreter = Interpreter::new(None)?;
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input.trim());
        let toks = lexer.scan_tokens();
        let errs_lex = lexer.get_errors();
        if *(&errs_lex.is_some()) {
            let err_unwraped = &errs_lex.unwrap();
            for error in err_unwraped {
                println!("{}", error);
            }
            panic!("{} lexing errors occured !", err_unwraped.len());
        }
        let mut parser = Parser::new(toks);
        let ast = parser.parse();
        let errs = parser.get_errors();
        if errs.is_some() {
            let err_unwraped = &errs.unwrap();
            for error in err_unwraped {
                println!("{}", error);
            }
            panic!("{} parsing errors occured !", err_unwraped.len());
        }
        interpreter.process_ast(&ast)?;
    }
}
