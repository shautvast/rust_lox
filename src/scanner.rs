use std::any::Any;

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
            '/' => {
                if self.match_char('/') {
                    while self.peek(0) != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            '!' => {
                let token = if self.match_char('=') { BANGEQUAL } else { BANG };
                self.add_token(token);
            }
            '=' => {
                let token = if self.match_char('=') { EQUALEQUAL } else { EQUAL };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_char('=') { GREATEREQUAL } else { GREATER };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_char('=') { LESSEQUAL } else { LESS };
                self.add_token(token);
            }
            '\n' => {
                self.line += 1;
            }
            ' ' => {}
            '\t' => {}
            '\r' => {}
            '\"' => self.string(),
            _ => {}
        }
    }

    /// handle string literals
    /// advances until a terminating double quote is found and then adds the string token to the list
    /// raises an interpreter error when the double-quote is not found and the end of the source has been reached
    fn string(&mut self) {
        while self.peek(0) != '\"' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.report_error(self.line, "unterminated string");
        } else {
            self.advance();

            let value = String::from(&self.source[self.start + 1..self.current - 1]);
            self.add_token_literal(STRING, Box::new(value));
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

    /// adds a token of the given type and content
    fn add_token_literal(&mut self, token_type: TokenType, literal: Box<dyn Any>) {
        let text = &self.source[self.start..self.current];
        let token = Token { token_type: token_type, lexeme: text, literal: literal, line: self.line };
        self.tokens.push(token);
    }

    /// returns true iff the end of the source has been reached
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// returns the character not yet advanced to.
    /// the integer ahead parameter can be used to look farther ahead.
    /// peek(0) is the first etc.
    fn peek(&self, ahead: usize) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + ahead..self.current + ahead + 1].chars().next().unwrap()
        }
    }

    /// Advances only if the next character matches the given expected character and returns true,
    /// or only returns false if there is no match.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current..self.current + 1].chars().next().unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    /// prints the error and sets the flag
    pub fn report_error(&mut self, line: usize, message: &str) {
        self.error_occured = true;
        println!("[line {} ] Error {} ", line, message);
    }
}
