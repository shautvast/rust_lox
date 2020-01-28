use std::fmt;

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Value {
    Text(String),
    Numeric(f64),
    Boolean(bool),
    None,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Text(value) => {
                write!(f, "{}", value.to_string())
            }
            Value::Numeric(value) => {
                write!(f, "{}", value)
            }
            Value::Boolean(value) => {
                write!(f, "{}", value)
            }
            Value::None => {
                write!(f, "Nil")
            }
        }
    }
}

/// struct that contains a single token
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Token {
    // the type
    pub token_type: TokenType,

    // the actual part of the code that resulted in this token
    pub lexeme: String,

    // numeric (ie 1,2, 1.0 etc) and alphanumeric (any quoted text) values
    pub literal: Value,

    // the line that contains the code for this token instance
    pub line: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
    SLASH,

    // One or two character tokens.
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals.
    STRING,
    NUMBER,
    IDENTIFIER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,         // end of file
}