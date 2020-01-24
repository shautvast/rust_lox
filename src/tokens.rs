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

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    EOF
}