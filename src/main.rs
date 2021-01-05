mod core;
mod tests;
mod utils;

use self::core::interpreter::*;
use self::core::lexer::*;
use self::core::parser::*;
use std::io::Write;
use utils::node::*;

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input.trim());
        let toks = lexer.scan_tokens();
        if lexer.had_error() {
            let errs = lexer.get_errors();
            for error in &errs {
                println!("{}", error);
            }
            panic!("{} lexing errors occured !", errs.len());
        }
        let mut parser = Parser::new(toks);
        let ast = parser.parse();
        if parser.had_error() {
            let errs = parser.get_errors();
            for error in &errs {
                println!("{}", error);
            }
            panic!("{} parsing errors occured !", errs.len());
        }
        let interpreted = interpret_operation(&ast);
        if interpreted.is_some() {
            println!("{}", interpreted.unwrap());
        } else {
            println!("{}", ast);
        }
    }
}
