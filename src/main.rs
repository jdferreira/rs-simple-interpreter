mod ast;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod token;

#[cfg(test)]
mod tests;

use parser::Parser;
use std::io::{self, BufRead, Write};

fn run() -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();

    loop {
        print!("calc> ");
        if let Err(e) = io::stdout().flush() {
            return Err(format!("{}", e));
        }

        let text = match stdin.next() {
            Some(Ok(text)) => text,
            Some(Err(e)) => Err(format!("{}", e))?,
            None => {
                println!();
                break Ok(());
            }
        };

        if text == "" {
            continue;
        }

        let mut parser = match Parser::new(&text) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let node = match parser.parse() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("{}", interpreter::interpret_node(&node))
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
