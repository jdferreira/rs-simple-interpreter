use crate::ast::Ast;
use crate::token::Kind as TokenKind;

pub fn interpret_node(ast: &Ast) -> i64 {
    match ast {
        Ast::Number(_, value) => *value,
        Ast::UnaryOperator(token, child) => match token.kind {
            TokenKind::Minus => -interpret_node(child),
            _ => unreachable!(),
        },
        Ast::BinaryOperator(left, token, right) => {
            let left = interpret_node(left);
            let right = interpret_node(right);

            match token.kind {
                TokenKind::Plus => left + right,
                TokenKind::Minus => left - right,
                TokenKind::Star => left * right,
                TokenKind::Slash => left / right,
                _ => unreachable!(),
            }
        }
    }
}
