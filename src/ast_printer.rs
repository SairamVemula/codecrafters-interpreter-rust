use crate::{
    expr::{Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal, Unary},
    token::{self},
};

pub struct AstPrinter {}

impl ExprVisitor for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        match &expr {
            Literal::Null => "nil".into(),
            Literal::String(s) => s.clone(),
            Literal::Number(n) => format!("{:?}", n),
            Literal::Boolean(b) => b.to_string(),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print(&self, expr: &ExprEnum) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Box<ExprEnum>]) -> String {
        let mut str = String::new();

        str.push_str("(");
        str.push_str(name);

        exprs.iter().for_each(|expr| {
            str.push_str(" ");
            str.push_str(&expr.accept(self));
        });

        str.push_str(")");

        str
    }
}
