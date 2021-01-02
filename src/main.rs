mod lexer;
mod tokens;

use lexer::*;
use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input.trim());
        let toks = lexer.scan_tokens();
        for tok in toks {
            println!("{:?}", tok);
        }
    }
}
