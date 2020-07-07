#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Integer,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: Kind,
    pub source: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: Kind, source: &'a str) -> Self {
        Token { kind, source }
    }
}
