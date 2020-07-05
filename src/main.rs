mod token;

use std::io::{self, BufRead, Write};
use std::iter::Peekable;
use std::str::Chars;
use token::{Kind as TokenKind, Token};

struct Interpreter<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    pos: usize,
    current_token: Option<Token<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        Interpreter {
            source,
            chars: source.chars().peekable(),
            pos: 0,
            current_token: None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let result = self.chars.next();

        if let Some(c) = result {
            self.pos += c.len_utf8();
        }

        result
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    fn next_token(&mut self) -> Result<Token<'a>, String> {
        let c = {
            let mut c = match self.next_char() {
                Some(c) => c,
                None => return Ok(Token::new_empty(TokenKind::Eof)),
            };

            if c.is_ascii_whitespace() {
                loop {
                    if let Some(next_c) = self.peek_char() {
                        c = self.next_char().unwrap();

                        if !next_c.is_ascii_whitespace() {
                            break;
                        }
                    } else {
                        return Ok(Token::new_empty(TokenKind::Eof))
                    }
                }
            }

            c
        };

        if c.is_ascii_digit() {
            let mut len = 1;

            while let Some(next_c) = self.peek_char() {
                if next_c.is_ascii_digit() {
                    self.next_char();
                    len += 1;
                } else {
                    break;
                }
            }

            Ok(Token::new(TokenKind::Integer, self.span(len)))
        } else if c == '+' {
            Ok(Token::new(TokenKind::Plus, self.span(1)))
        } else {
            Err(format!(
                "Cannot process the character '{}' at position {}",
                c, self.pos
            ))
        }
    }

    fn span(&self, len: usize) -> &'a str {
        self.source.get(self.pos - len..self.pos).unwrap()
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
        let result = left.source.parse::<i64>().unwrap() + right.source.parse::<i64>().unwrap();

        Ok(result)
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
