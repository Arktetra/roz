struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

struct Unary {
    Operator: Token,
    right: Expr,
}

struct Literal {
    value: Literal,
}

struct Grouping {
    expression: Expr,
}

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Unary()
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
}
