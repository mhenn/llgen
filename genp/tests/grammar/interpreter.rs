use ruge::{grammar::*, interpreter::*};

#[test]
fn interpreting_standard_grammar_too_long_chromosome() {
    let grammar = get_test_grammar();
    let chromosome = vec![13, 4, 9, 33, 16, 14, 3, 28, 12];
    let ret = interpret(&chromosome, &grammar);
    assert!(ret.is_err());
    assert!(matches!(
        ret.unwrap_err(),
        InterpreterError::TooManyCodonsError
    ));
}

#[test]
fn interpreting_standard_grammar_too_short_chromosome() {
    let grammar = get_test_grammar();
    let chromosome = vec![13, 4, 9, 33, 16, 14, 3];
    let ret = interpret(&chromosome, &grammar);
    assert!(ret.is_err());
    assert!(matches!(
        ret.unwrap_err(),
        InterpreterError::TooFewCodonsError
    ));
}

#[test]
fn interpreting_standard_grammar() {
    let grammar = get_test_grammar();
    let chromosome = vec![13, 4, 9, 33, 16, 14, 3, 28];
    let ret = interpret(&chromosome, &grammar);
    assert!(ret.is_ok());
    assert!(ret.unwrap() == vec!["0", "-", "x", "/", "x"]);
}
