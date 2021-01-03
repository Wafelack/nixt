mod core;
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
        let mut parser = Parser::new(toks);
        let ast = parser.parse();
        println!("{:?}", ast);
    }
}
