use super::Interpreter;

#[test]
fn it_correctly_interpret_expressions() {
    let tests = vec![
        ("3", 3),
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
