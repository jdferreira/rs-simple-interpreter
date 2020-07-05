#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kind {
    Integer,
    Plus,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub source: Vec<u8>,
}

impl Token {
    pub fn new(kind: Kind, source: Vec<u8>) -> Self {
        Token { kind, source }
    }

    pub fn new_empty(kind: Kind) -> Self {
        Token {
            kind,
            source: vec![],
        }
    }
}
