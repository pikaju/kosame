use super::Expr;

pub struct Paren {
    expr: &'static Expr,
}

impl Paren {
    pub const fn new(expr: &'static Expr) -> Self {
        Self { expr }
    }

    pub fn to_sql_string(&self, buf: &mut String) {
        *buf += "(";
        self.expr.to_sql_string(buf);
        *buf += ")";
    }
}
