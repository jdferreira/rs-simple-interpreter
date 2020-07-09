use crate::token::Token;

pub enum Ast<'a> {
    Number(Token<'a>, i64),
    UnaryOperator(Token<'a>, Box<Ast<'a>>),
    BinaryOperator(Box<Ast<'a>>, Token<'a>, Box<Ast<'a>>),
}
