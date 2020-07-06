mod token;

use std::io::{self, BufRead, Write};
use std::str::Chars;
use token::{Kind as TokenKind, Token};

struct Interpreter<'a> {
    source: &'a str,
    chars: Chars<'a>,
    pos: usize,
    byte_pos: usize,
    current_char: Option<char>,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let current_char = chars.next();

        Interpreter {
            source,
            chars,
            pos: 0,
            byte_pos: 0,
            current_char,
        }
    }

    fn advance(&mut self) -> &'a str{
        let start = self.byte_pos;

        if let Some(c) = self.current_char {
            self.pos += 1;
            self.byte_pos += c.len_utf8();
        }

        self.current_char = self.chars.next();

        &self.source[start..self.byte_pos]
    }

    fn skip_whitespace(&mut self) {
        while self
            .current_char
            .map(|c| c.is_ascii_whitespace())
            .unwrap_or(false)
        {
            self.advance();
        }
    }

    fn integer(&mut self) -> &'a str {
        let start = self.byte_pos;

        while self
            .current_char
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            self.advance();
        }

        &self.source[start..self.byte_pos]
    }

    fn next_token(&mut self) -> Result<Token<'a>, String> {
        while let Some(c) = self.current_char {
            if c.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            return if c.is_ascii_digit() {
                Ok(Token::new(TokenKind::Integer, self.integer()))
            } else if c == '+' {
                Ok(Token::new(TokenKind::Plus, self.advance()))
            } else if c == '-' {
                Ok(Token::new(TokenKind::Minus, self.advance()))
            } else if c == '*' {
                Ok(Token::new(TokenKind::Star, self.advance()))
            } else if c == '/' {
                Ok(Token::new(TokenKind::Slash, self.advance()))
            } else {
                Err(format!(
                    "Cannot process the character '{}' at position {}",
                    c, self.pos
                ))
            };
        }

        Ok(Token::new_empty(TokenKind::Eof))
    }

    fn eat(&mut self, expected: TokenKind) -> Result<Token<'a>, String> {
        self.eat_alt(&[expected])
    }

    fn eat_alt(&mut self, expected: &[TokenKind]) -> Result<Token<'a>, String> {
        // Compare the current token type with the expected one and if they
        // match then go through the current token and assign the next token to
        // the self.current_token; otherwise return an error.

        let token = self.next_token()?;

        if expected.contains(&token.kind) {
            Ok(token)
        } else {
            Err(format!("Unexpected token {:?}", token))
        }
    }

    fn expr(&mut self) -> Result<i64, String> {
        // expt -> INTEGER PLUS INTEGER
        // expt -> INTEGER MINUS INTEGER
        // expt -> INTEGER STAR INTEGER
        // expt -> INTEGER SLASH INTEGER

        // we expect the current token to be an integer
        let left = self.eat(TokenKind::Integer)?.source.parse::<i64>().unwrap();

        // we expect the current token to be a binary operator
        let op = self.eat_alt(&[
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
        ])?;

        // we expect the current token to be an integer
        let right = self.eat(TokenKind::Integer)?.source.parse::<i64>().unwrap();

        // after the above call the self.current_token is set to EOF token
        self.eat(TokenKind::Eof)?;

        // at this point INTEGER PLUS INTEGER sequence of tokens has been
        // successfully found and the method can just return the result of
        // adding two integers, thus effectively interpreting client input
        Ok(match op.kind {
            TokenKind::Plus => left + right,
            TokenKind::Minus => left - right,
            TokenKind::Star => left * right,
            TokenKind::Slash => left / right,
            _ => unreachable!(),
        })
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

        let mut interpreter = Interpreter::new(&text);

        match interpreter.expr() {
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
