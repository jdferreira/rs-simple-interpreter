use std::str::Chars;
use crate::token::{Token, Kind as TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    pos: usize,
    byte_pos: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let current_char = chars.next();

        Lexer {
            source,
            chars,
            pos: 0,
            byte_pos: 0,
            current_char,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    fn advance(&mut self) -> &'a str {
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

    pub fn next_token(&mut self) -> Result<Token<'a>, String> {
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
            } else if c == '(' {
                Ok(Token::new(TokenKind::LParen, self.advance()))
            } else if c == ')' {
                Ok(Token::new(TokenKind::RParen, self.advance()))
            } else {
                Err(format!(
                    "Cannot process the character '{}' at position {}",
                    c, self.pos
                ))
            };
        }

        Ok(Token::new(TokenKind::Eof, ""))
    }
}
