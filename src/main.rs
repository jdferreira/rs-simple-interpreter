mod token;

use std::io::{self, BufRead, Write};
use token::{Kind as TokenKind, Token};

struct Interpreter {
    source: Vec<u8>,
    pos: usize,
    current_token: Option<Token>,
}

impl Interpreter {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Interpreter {
            source: text.into().into_bytes(),
            pos: 0,
            current_token: None,
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        if self.pos > self.source.len() - 1 {
            return Ok(Token::new_empty(TokenKind::Eof));
        }

        let current_char = self.source[self.pos];

        if current_char.is_ascii_digit() {
            self.pos += 1;
            return Ok(Token::new(TokenKind::Integer, vec![current_char]));
        }
        if current_char == b'+' {
            self.pos += 1;
            return Ok(Token::new(TokenKind::Plus, vec![current_char]));
        }

        Err(format!(
            "Cannot process the character '{}' at position {}",
            current_char as char, self.pos
        ))
    }

    fn eat(&mut self, expected: TokenKind) -> Result<(), String> {
        // Compare the current token type with the expected one and if they
        // match then go through the current token and assign the next token to
        // the self.current_token; otherwise return an error.

        if let Some(ref t) = self.current_token {
            if t.kind == expected {
                self.current_token = Some(self.next_token()?);

                Ok(())
            } else {
                Err(format!("Unexpected token {:?}", t))
            }
        } else {
            Err(format!("Unexpected empty token"))
        }
    }

    fn expr(&mut self) -> Result<i64, String> {
        // expt -> INTEGER PLUS INTEGER

        // set current token to the first token taken from the input
        self.current_token = Some(self.next_token()?);

        // we expect the current token to be a single-digit integer
        let left = self.current_token.clone().unwrap();
        self.eat(TokenKind::Integer)?;

        // we expect the current token to be a '+' token
        let op = self.current_token.clone().unwrap();
        self.eat(TokenKind::Plus)?;

        // we expect the current token to be a single-digit integer
        let right = self.current_token.clone().unwrap();
        self.eat(TokenKind::Integer)?;

        // after the above call the self.current_token is set to EOF token

        // at this point INTEGER PLUS INTEGER sequence of tokens has been
        // successfully found and the method can just return the result of
        // adding two integers, thus effectively interpreting client input
        let result = as_int(left.source) + as_int(right.source);

        Ok(result)
    }
}

fn as_int(source: Vec<u8>) -> i64 {
    let mut result: i64 = 0;

    for elem in source {
        assert!(elem >= b'0' && elem <= b'9');

        result *= 10;
        result += (elem - b'0') as i64;
    }

    result
}

fn run() -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();

    loop {
        print!("calc> ");
        if let Err(e) = io::stdout().flush() {
            Err(format!("{}", e))?;
        }

        let text = match stdin.next() {
            Some(Ok(text)) => text,
            Some(Err(e)) => Err(format!("{}", e))?,
            None => break Ok(()),
        };

        let mut interpreter = Interpreter::new(text);
        match interpreter.expr() {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e)
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
