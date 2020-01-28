use crate::expression::Expr;
use crate::expression::Expr::*;
use crate::tokens::{Token, TokenType};
use crate::tokens::TokenType::*;
use crate::tokens::Value::*;

pub fn parse(tokens: Vec<Token>) -> Expr {
    Parser::new(tokens).parse()
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&[BANGEQUAL, EQUALEQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        while self.match_token(&[GREATER, GREATEREQUAL, LESS, LESSEQUAL]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn match_token(&mut self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(*token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        return if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        };
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        while self.match_token(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary(Box::new(expr), operator, Box::new(right));
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Unary(operator, Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(&[FALSE]) {
            return Literal(Boolean(false));
        }
        if self.match_token(&[TRUE]) {
            return Literal(Boolean(true));
        }

        if self.match_token(&[NIL]) {
            return Literal(None);
        }
        if self.match_token(&[NUMBER, STRING]) {
            return Literal(self.previous().literal);
        }

        if self.match_token(&[LEFTPAREN]) {
            let expr = self.expression();
            self.consume_token(RIGHTPAREN, "Expect ')' after expression.");
            return Grouping(Box::new(expr));
        } else {
            Literal(None)
        }
    }

    fn consume_token(&mut self, token_type: TokenType, _message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!()
    }
}

