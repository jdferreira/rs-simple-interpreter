use crate::error::Error;
use crate::token::Kind as TokenKind;
use crate::Interpreter;

#[test]
fn it_correctly_interpret_expressions() {
    let tests = vec![
        ("3", 3),
        ("-5", -5),
        ("1 * -7", -7),
        ("7 - 8 / 4", 5),
        ("2 + 7 * 4", 30),
        ("7 + (((3 + 2)))", 12),
        ("14 + 2 * 3 - 6 / 2", 17),
        ("7 + 3 * (10 / (12 / (3 + 1) - 1))", 22),
        ("7 + 3 * (10 / (12 / (4) - 1)) / (2 + 3) - 5 - 3 + (8)", 10),
    ];

    for (source, result) in tests {
        assert_eq!(
            Interpreter::new(source).unwrap().interpret().unwrap(),
            result
        );
    }
}

#[test]
fn it_expects_the_correct_token_on_errors() {
    let result = Interpreter::new("3-*1").unwrap().interpret();

    assert!(result.is_err());

    let err = result.unwrap_err();

    if let Error::UnexpectedToken {
        current,
        pos,
        expected,
    } = err
    {
        assert_eq!(current.kind, TokenKind::Star);
        assert_eq!(pos, 3);
        assert!(expected.contains(&TokenKind::Integer));
        assert!(expected.contains(&TokenKind::Minus));
        assert!(expected.contains(&TokenKind::LParen));
    } else {
        panic!("Unexpected error");
    }
}
