#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kind {
    Integer,
    Plus,
    Minus,
    Star,
    Slash,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: Kind,
    pub source: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: Kind, source: &'a str) -> Self {
        Token { kind, source }
    }

    pub fn new_empty(kind: Kind) -> Self {
        Token {
            kind,
            source: "",
        }
    }
}
