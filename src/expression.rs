use crate::tokens::{Token, Value};

pub trait Visitor<R> {
    fn visit_expr(&mut self, expr: &Expr) -> R;
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Unary(Token, Box<Expr>),
}

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        return match expr {
            Expr::Binary(left, operator, right) => {
                self.parenthesize(&operator.lexeme, &[left, right])
            }
            Expr::Grouping(expression) => {
                self.parenthesize("group", &[expression])
            }
            Expr::Literal(value) => {
                format!("{:?}", value)
            }
            Expr::Unary(operator, right) => {
                self.parenthesize(&operator.lexeme, &[right])
            }
        };
    }
}

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, expressions: &[&Expr]) -> String {
        let mut buf = String::from("(");
        buf.push_str(name);
        buf.push_str(" ");

        let mut index = 0;
        for expr in expressions {
            if index > 0 { buf.push_str(" "); }
            buf.push_str(&self.visit_expr(expr));
            index += 1;
        }

        buf.push_str(")");
        buf
    }
}