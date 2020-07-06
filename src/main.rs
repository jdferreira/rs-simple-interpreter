mod lexer;
mod token;

use lexer::Lexer;
use std::io::{self, BufRead, Write};
use token::{Kind as TokenKind, Token};

struct Interpreter<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

enum TokenError<'a> {
    Unexpected {
        current: Token<'a>,
        expected: Vec<TokenKind>,
    },
    Other(String),
}

impl<'a> From<TokenError<'a>> for String {
    fn from(e: TokenError<'a>) -> String {
        match e {
            TokenError::Unexpected { current, expected } => format!(
                "Unexpected token {:?} (expecting {:?})",
                current, expected
            ),
            TokenError::Other(e) => e,
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, TokenError<'a>> {
        let current_token = match lexer.next_token() {
            Ok(t) => t,
            Err(e) => Err(TokenError::Other(e))?,
        };

        Ok(Interpreter {
            lexer,
            current_token,
        })
    }

    fn next_token(&mut self) -> Result<Token<'a>, TokenError<'a>> {
        match self.lexer.next_token() {
            Ok(t) => Ok(t),
            Err(e) => Err(TokenError::Other(e))?,
        }
    }

    fn eat_alt(&mut self, expected: &[TokenKind]) -> Result<Token<'a>, TokenError<'a>> {
        // Compare the current token type with the expected ones and if they
        // match then go through the current token and assign the next token to
        // the self.current_token; otherwise return an error.

        let token = self.current_token;

        if expected.contains(&token.kind) {
            self.current_token = self.next_token()?;
            Ok(token)
        } else {
            Err(TokenError::Unexpected {
                current: token,
                expected: expected.iter().cloned().collect(),
            })
        }
    }

    fn eat(&mut self, expected: TokenKind) -> Result<Token<'a>, TokenError<'a>> {
        self.eat_alt(&[expected])
    }

    /// `factor : INTEGER`
    fn factor(&mut self) -> Result<i64, TokenError<'a>> {
        Ok(self.eat(TokenKind::Integer)?.source.parse().unwrap())
    }

    /// `term : factor ((STAR | SLASH) factor)*`
    fn term(&mut self) -> Result<i64, TokenError<'a>> {
        let mut value = self.factor()?;

        loop {
            let op = match self.eat_alt(&[TokenKind::Star, TokenKind::Slash]) {
                Ok(op) => op,
                Err(TokenError::Unexpected { .. }) => break,
                Err(e) => Err(e)?,
            };

            let right = self.factor()?;

            value = match op.kind {
                TokenKind::Star => value * right,
                TokenKind::Slash => value / right,
                _ => unreachable!(),
            };
        }

        // at this point INTEGER PLUS INTEGER sequence of tokens has been
        // successfully found and the method can just return the result of
        // adding two integers, thus effectively interpreting client input
        Ok(value)
    }

    /// `expr : term ((PLUS | MINUS) term)* EOF`
    fn expr(&mut self) -> Result<i64, TokenError<'a>> {
        let mut value = self.term()?;

        loop {
            let op = match self.eat_alt(&[TokenKind::Plus, TokenKind::Minus]) {
                Ok(op) => op,
                Err(TokenError::Unexpected { .. }) => break,
                Err(e) => Err(e)?,
            };

            let right = self.term()?;

            value = match op.kind {
                TokenKind::Plus => value + right,
                TokenKind::Minus => value - right,
                _ => unreachable!(),
            };
        }

        // expect to be at the end of the text
        self.eat(TokenKind::Eof)?;

        Ok(value)
    }
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
            None => {
                println!();
                break Ok(());
            }
        };

        let lexer = Lexer::new(&text);
        let mut interpreter = Interpreter::new(lexer)?;

        match interpreter.expr() {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", String::from(e)),
        }
    }
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
