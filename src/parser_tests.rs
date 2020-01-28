use crate::expression::Expr::Binary;
use crate::expression::Expr::Literal;
use crate::parser::parse;
use crate::scanner::scan_tokens;
use crate::tokens::Token;
use crate::tokens::TokenType::PLUS;
use crate::tokens::Value::{None,Numeric};

#[test]
fn test_scan_empty_source() {
    let tokens = scan_tokens("").unwrap();
    let expression = parse(tokens);

    assert_eq!(expression, Literal(None));
}

#[test]
fn test_scan_arithmetic() {
    let tokens = scan_tokens("1+1").unwrap();
    let expression = parse(tokens);

    assert_eq!(expression, Binary(Box::new(Literal(Numeric(1.0))),
                                  Token {
                                      token_type: PLUS,
                                      lexeme: String::from("+"),
                                      literal: None,
                                      line: 1,
                                  },
                                  Box::new(Literal(Numeric(1.0)))));
}
