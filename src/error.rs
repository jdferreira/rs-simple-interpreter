use crate::token::{Kind as TokenKind, Token};
use std::fmt;

#[derive(Debug)]
pub enum Error<'a> {
    UnexpectedToken {
        current: Token<'a>,
        pos: usize,
        expected: Vec<TokenKind>,
    },
    UnexpectedCharacter {
        character: char,
        pos: usize,
    },
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::UnexpectedToken {
                current,
                pos,
                expected,
            } => {
                let token_msg = if current.kind == TokenKind::Eof {
                    format!("end of source")
                } else {
                    format!("'{}' (token type {:?})", current.source, current.kind)
                };

                format!(
                    "Unexpected {} at position {} (expecting one of {:?})",
                    token_msg, pos, expected
                )
            }
            Error::UnexpectedCharacter { character, pos } => {
                format!("Unexpected character '{}' at position {}", character, pos)
            }
        };

        write!(f, "{}", msg)
    }
}
