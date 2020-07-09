use crate::ast::Ast;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::{Kind as TokenKind, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Result<Self, Error<'a>> {
        let mut lexer = Lexer::new(source);
        let current_token = lexer.next_token()?;

        Ok(Parser {
            lexer,
            current_token,
        })
    }

    fn advance(&mut self) -> Result<Token<'a>, Error<'a>> {
        let token = self.current_token;

        self.current_token = self.lexer.next_token()?;

        Ok(token)
    }

    fn try_match(&mut self, expected: &[TokenKind]) -> bool {
        expected.contains(&self.current_token.kind)
    }

    fn eat(&mut self, expected: TokenKind) -> Result<Token<'a>, Error<'a>> {
        self.eat_alt(&[expected])
    }

    fn eat_alt(&mut self, expected: &[TokenKind]) -> Result<Token<'a>, Error<'a>> {
        // Compare the current token type with the expected ones and if they
        // match then go through the current token and assign the next token to
        // the self.current_token; otherwise return an error.

        if self.try_match(expected) {
            Ok(self.advance()?)
        } else {
            Err(Error::UnexpectedToken {
                current: self.current_token,
                pos: self.lexer.pos(),
                expected: expected.iter().cloned().collect(),
            })
        }
    }

    /*

    def factor(self):
        """factor : INTEGER | LPAREN expr RPAREN"""
        token = self.current_token
        if token.type == INTEGER:
            self.eat(INTEGER)
            return Num(token)
        elif token.type == LPAREN:
            self.eat(LPAREN)
            node = self.expr()
            self.eat(RPAREN)
            return node

    */

    /// `factor : MINUS factor | INTEGER | LPAREN expr RPAREN`
    fn factor(&mut self) -> Result<Ast<'a>, Error<'a>> {
        if self.current_token.kind == TokenKind::Minus {
            let token = self.current_token;
            self.advance()?;
            Ok(Ast::UnaryOperator(token, Box::new(self.factor()?)))
        } else if self.current_token.kind == TokenKind::Integer {
            let token = self.current_token;
            self.advance()?;
            Ok(Ast::Number(token, token.source.parse().unwrap()))
        } else if self.current_token.kind == TokenKind::LParen {
            self.advance()?;
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

    /// `term : factor ((STAR | SLASH) factor)*`
    fn term(&mut self) -> Result<Ast<'a>, Error<'a>> {
        let mut value = self.factor()?;

        while self.try_match(&[TokenKind::Star, TokenKind::Slash]) {
            let op = self.advance()?;
            let right = self.factor()?;
            value = Ast::BinaryOperator(Box::new(value), op, Box::new(right));
        }

        // at this point INTEGER PLUS INTEGER sequence of tokens has been
        // successfully found and the method can just return the result of
        // adding two integers, thus effectively interpreting client input
        Ok(value)
    }

    /// `expr : term ((PLUS | MINUS) term)* EOF`
    fn expr(&mut self) -> Result<Ast<'a>, Error<'a>> {
        let mut value = self.term()?;

        while self.try_match(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.advance()?;
            let right = self.term()?;
            value = Ast::BinaryOperator(Box::new(value), op, Box::new(right));
        }

        Ok(value)
    }

    pub fn parse(&mut self) -> Result<Ast<'a>, Error<'a>> {
        let result = self.expr()?;

        self.eat(TokenKind::Eof)?;

        Ok(result)
    }
}
