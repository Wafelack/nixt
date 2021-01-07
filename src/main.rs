mod core;
mod tests;
mod utils;

use self::core::lexer::*;
use self::core::parser::*;
use std::io::Write;

fn main() {
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
        println!("{}", ast);
        let errs = parser.get_errors();
        if errs.is_some() {
            let err_unwraped = &errs.unwrap();
            for error in err_unwraped {
                println!("{}", error);
            }
            panic!("{} parsing errors occured !", err_unwraped.len());
        }
    }
}
