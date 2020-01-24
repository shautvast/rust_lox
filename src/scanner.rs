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
    fn scan_token(&mut self) {
        let next_char = self.advance();

        // determine what to do with every character
        match next_char {
            '(' => self.add_token(LEFTPAREN),
            ')' => self.add_token(RIGHTPAREN),
            '{' => self.add_token(LEFTBRACE),
            '}' => self.add_token(RIGHTBRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            _ => {}
        }
    }

    /// advance (consume) one character and return that
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1..self.current].chars().next().unwrap()
    }

    /// adds a token of the given type
    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        let token = Token { token_type: token_type, lexeme: text, literal: Box::new(""), line: self.line };
        self.tokens.push(token);
    }

    /// returns true iff the end of the source has been reached
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
