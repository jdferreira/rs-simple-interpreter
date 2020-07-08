mod lexer;
mod token;
mod error;

#[cfg(test)]
mod tests;

use lexer::Lexer;
use std::io::{self, BufRead, Write};
use token::{Kind as TokenKind, Token};
use error::Error;

#[derive(Debug)]
struct Interpreter<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Result<Self, Error<'a>> {
        let mut lexer = Lexer::new(source);
        let current_token = lexer.next_token()?;

        Ok(Interpreter {
            lexer,
            current_token,
        })
    }

    fn eat_alt(&mut self, expected: &[TokenKind]) -> Result<Token<'a>, Error<'a>> {
        // Compare the current token type with the expected ones and if they
        // match then go through the current token and assign the next token to
        // the self.current_token; otherwise return an error.

        if expected.contains(&self.current_token.kind) {
            Ok(self.advance()?)
        } else {
            Err(Error::UnexpectedToken {
                current: self.current_token,
                pos: self.lexer.pos(),
                expected: expected.iter().cloned().collect(),
            })
        }
    }

    fn eat(&mut self, expected: TokenKind) -> Result<Token<'a>, Error<'a>> {
        self.eat_alt(&[expected])
    }

    /// `factor : MINUS factor | INTEGER | LPAREN expr RPAREN`
    fn factor(&mut self) -> Result<i64, Error<'a>> {
        if self.current_token.kind == TokenKind::Minus {
            self.eat(TokenKind::Minus)?;
            Ok(-self.factor()?)
        } else if self.current_token.kind == TokenKind::Integer {
            Ok(self.eat(TokenKind::Integer)?.source.parse().unwrap())
        } else if self.current_token.kind == TokenKind::LParen {
            self.eat(TokenKind::LParen)?;
            let result = self.expr()?;
            self.eat(TokenKind::RParen)?;
            Ok(result)
        } else {
            Err(Error::UnexpectedToken {
                current: self.current_token,
                pos: self.lexer.pos(),
                expected: vec![TokenKind::Integer, TokenKind::Minus, TokenKind::LParen],
            })
        }
    }

    fn try_match(&mut self, expected: &[TokenKind]) -> bool {
        expected.contains(&self.current_token.kind)
    }

    fn advance(&mut self) -> Result<Token<'a>, Error<'a>> {
        let token = self.current_token;

        self.current_token = self.lexer.next_token()?;

        Ok(token)
    }

    /// `term : factor ((STAR | SLASH) factor)*`
    fn term(&mut self) -> Result<i64, Error<'a>> {
        let mut value = self.factor()?;

        while self.try_match(&[TokenKind::Star, TokenKind::Slash]) {
            let op = self.advance()?;
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
    fn expr(&mut self) -> Result<i64, Error<'a>> {
        let mut value = self.term()?;

        while self.try_match(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.advance()?;
            let right = self.term()?;

            value = match op.kind {
                TokenKind::Plus => value + right,
                TokenKind::Minus => value - right,
                _ => unreachable!(),
            };
        }

        Ok(value)
    }

    fn interpret(&mut self) -> Result<i64, Error<'a>> {
        let result = self.expr()?;

        self.eat(TokenKind::Eof)?;

        Ok(result)
    }
}

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
