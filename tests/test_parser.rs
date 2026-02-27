use psvm::parser::{parse_keyword, parse_operator, Keyword, Operator};

#[test]
fn test_parse_keyword() {
    assert_eq!(parse_keyword("let"), Some(Keyword::Let));
    assert_eq!(parse_keyword("logShow"), Some(Keyword::LogShow));
    assert_eq!(parse_keyword("unknown"), None);
}

#[test]
fn test_parse_operator() {
    assert_eq!(parse_operator("+"), Some(Operator::Add));
    assert_eq!(parse_operator("-"), Some(Operator::Sub));
    assert_eq!(parse_operator("*"), Some(Operator::Mul));
    assert_eq!(parse_operator("/"), Some(Operator::Div));
    assert_eq!(parse_operator("?"), None);
}
