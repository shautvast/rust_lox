use std::collections::HashMap;

use crate::tokens::TokenType;
use crate::tokens::TokenType::*;

lazy_static! {
pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", AND);
        keywords.insert("class", CLASS);
        keywords.insert("else", ELSE);
        keywords.insert("false", FALSE);
        keywords.insert("for", FOR);
        keywords.insert("fun", FUN);
        keywords.insert("if", IF);
        keywords.insert("nil", NIL);
        keywords.insert("or", OR);
        keywords.insert("print", PRINT);
        keywords.insert("return", RETURN);
        keywords.insert("super", SUPER);
        keywords.insert("this", THIS);
        keywords.insert("true", TRUE);
        keywords.insert("var", VAR);
        keywords.insert("while", WHILE);
        keywords
    };
}