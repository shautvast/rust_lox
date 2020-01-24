use crate::tokens::{Token, TokenType};
use crate::tokens::TokenType::*;

/// public function for scanning lox source
/// outputs a Vector of Tokens
pub fn scan_tokens(source: &str) -> Result<Vec<Token>, &'static str> {
    let mut scanner = Scanner::new(source);

    while !scanner.is_at_end() {
        scanner.start = scanner.current;
        scanner.scan_token()
    }

    scanner.tokens.push(Token {
        token_type: EOF,
        lexeme: "lexeme",
        literal: Box::new(""),
        line: scanner.line,
    });

    if scanner.error_occured {
        Err("Error occurred")
    } else {
        Ok(scanner.tokens)
    }
}

///struct used internally to keep state while scanning
struct Scanner<'a> {
    // the source to scan
    source: &'a str,

    // the tokens that will be the output of the scan function
    tokens: Vec<Token<'a>>,

    // start of unscanned source (updated after part of the source was scanned)
    start: usize,

    // current character index while scanning
    current: usize,

    // flag indicating compilation error
    error_occured: bool,

    // current line (mainly used to report the line after a compilation error occurred)
    line: usize,
}

impl Scanner<'_> {
    /// create Scanner struct using the source
    fn new(source: &str) -> Scanner {
        Scanner { tokens: Vec::new(), source, start: 0, current: 0, line: 1, error_occured: false }
    }

    ///scans the source, character by character
    fn scan_token(&mut self) {}

    /// adds a token of the given type
    fn add_token(&mut self, token_type: TokenType) {}

    /// returns true iff the end of the source has been reached
    fn is_at_end(&self) -> bool {
        true
    }
}
