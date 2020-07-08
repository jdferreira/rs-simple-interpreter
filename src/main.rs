mod error;
mod interpreter;
mod lexer;
mod token;

#[cfg(test)]
mod tests;

use interpreter::Interpreter;
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

        let mut interpreter = match Interpreter::new(&text) {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match interpreter.interpret() {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
