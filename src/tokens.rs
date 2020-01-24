use std::any::Any;
use std::fmt;

/// struct that contains a single token
pub struct Token<'a> {
    // the type
    pub lexeme: &'a str,

    // the actual part of the code that resulted in this token
    pub literal: Box<dyn Any>,

    // numeric (ie 1,2, 1.0 etc) and alphanumeric (any quoted text) values
    pub line: usize,

    // the line that contains the code for this token instance
    pub token_type: TokenType,
}

impl Token<'_> {
    pub fn get_literal_as_string(&self) -> Option<&str> {
        self.literal.downcast_ref::<String>().map(|s| s.as_str())
    }

    pub fn get_literal_as_float(&self) -> Option<f64> {
        self.literal.downcast_ref::<f64>().map(|f| *f)
    }
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lit = match self.literal.downcast_ref::<String>() {
            Some(as_string) => {
                as_string.to_string()
            }
            None => {
                format!("{:?}", self.literal)
            }
        };

        write!(f, "Token [ type: {:?}, lexeme: {}, literal: {}, line: {} ]", self.token_type, self.lexeme, lit, self.line)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,
    // (
    RIGHTPAREN,
    // )
    LEFTBRACE,
    // [
    RIGHTBRACE,
    // ]
    COMMA,
    // ,
    DOT,
    // .
    MINUS,
    // -
    PLUS,
    // +
    SEMICOLON,
    // ;
    STAR,
    // *
    SLASH,      // /

    // One or two character tokens.
    BANG,
    // !
    BANGEQUAL,
    // !=
    EQUAL,
    // =
    EQUALEQUAL,
    // ==
    GREATER,
    // >
    GREATEREQUAL,
    // >=
    LESS,
    // <
    LESSEQUAL,      // <=

    // Literals.
    STRING,
    NUMBER,

    EOF,         // end of file
}