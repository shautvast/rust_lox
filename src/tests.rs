#[cfg(test)]
use crate::scanner::scan_tokens;
use crate::tokens::TokenType::*;

#[test]
fn test_scan_empty_source() {
    let tokens = scan_tokens("").unwrap();
    assert_eq!(tokens.len(), 1);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, EOF);
    assert_eq!(token.line, 1);
}

#[test]
fn test_scan_single_char_tokens() {
    let tokens = scan_tokens(">").unwrap();
    assert_eq!(tokens.len(), 2);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, GREATER);
    assert_eq!(token.lexeme, ">");

    let token = tokens.get(1).unwrap();
    assert_eq!(token.token_type, EOF);
}

#[test]
fn test_scan_double_char_tokens() {
    let tokens = scan_tokens(">=").unwrap();
    assert_eq!(tokens.len(), 2);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, GREATEREQUAL);
    assert_eq!(token.lexeme, ">=");
}


#[test]
fn test_scan_string_literals() {
    let tokens = scan_tokens("\"hello world\"").unwrap();
    assert_eq!(tokens.len(), 2);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, STRING);
    assert_eq!(token.lexeme, "\"hello world\"");
    assert_eq!(token.get_literal_as_string().unwrap(), "hello world");
}

#[test]
fn test_scan_numeric_literals() {
    let tokens = scan_tokens("0.1").unwrap();
    assert_eq!(tokens.len(), 2);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, NUMBER);
    assert_eq!(token.lexeme, "0.1");
    assert_eq!(token.get_literal_as_float().unwrap(), 0.1);
}

#[test]
fn test_keywords() {
    let tokens = scan_tokens("fun").unwrap();
    assert_eq!(tokens.len(), 2);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, FUN);
}

#[test]
fn test_identifiers() {
    let tokens = scan_tokens("a").unwrap();
    assert_eq!(tokens.len(), 2);

    let token = tokens.get(0).unwrap();
    assert_eq!(token.token_type, IDENTIFIER);
}

#[test]
fn test_expression() {
    let tokens = scan_tokens("if a == 1 {b=\"hello world\"}").unwrap();
    assert_eq!(tokens.len(), 10);

    assert_eq!(tokens.get(0).unwrap().token_type, IF);
    assert_eq!(tokens.get(1).unwrap().token_type, IDENTIFIER);
    assert_eq!(tokens.get(2).unwrap().token_type, EQUALEQUAL);
    assert_eq!(tokens.get(3).unwrap().token_type, NUMBER);
    assert_eq!(tokens.get(4).unwrap().token_type, LEFTBRACE);
    assert_eq!(tokens.get(5).unwrap().token_type, IDENTIFIER);
    assert_eq!(tokens.get(6).unwrap().token_type, EQUAL);
    assert_eq!(tokens.get(7).unwrap().token_type, STRING);
    assert_eq!(tokens.get(8).unwrap().token_type, RIGHTBRACE);
    assert_eq!(tokens.get(9).unwrap().token_type, EOF);
}