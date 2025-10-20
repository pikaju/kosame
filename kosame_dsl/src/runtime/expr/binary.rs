use std::fmt::Write;

use crate::sql;

use super::Expr;

pub struct Binary {
    left: &'static Expr,
    op: BinOp,
    right: &'static Expr,
}

impl Binary {
    pub const fn new(left: &'static Expr, op: BinOp, right: &'static Expr) -> Self {
        Self { left, op, right }
    }

    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        self.left.fmt_sql(formatter)?;
        self.op.fmt_sql(formatter)?;
        self.right.fmt_sql(formatter)?;
        Ok(())
    }
}

pub enum BinOp {
    // multiplication, division, modulo
    Multiply,
    Divide,
    Modulo,
    // addition, subtraction
    Add,
    Subtract,
    // comparison operators
    Eq,
    Uneq,
    LessThan,
    GreaterThan,
    LessThanOrEq,
    GreaterThanOrEq,
    // is
    Is,
    IsNot,
    IsDistinctFrom,
    // logical
    And,
    Or,
}

impl BinOp {
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        match self {
            Self::Multiply => formatter.write_str(" * "),
            Self::Divide => formatter.write_str(" / "),
            Self::Modulo => formatter.write_str(" % "),
            Self::Add => formatter.write_str(" + "),
            Self::Subtract => formatter.write_str(" - "),
            Self::Eq => formatter.write_str(" = "),
            Self::Uneq => formatter.write_str(" <> "),
            Self::LessThan => formatter.write_str(" < "),
            Self::GreaterThan => formatter.write_str(" > "),
            Self::LessThanOrEq => formatter.write_str(" <= "),
            Self::GreaterThanOrEq => formatter.write_str(" >= "),
            Self::Is => formatter.write_str(" is "),
            Self::IsNot => formatter.write_str(" is not "),
            Self::IsDistinctFrom => formatter.write_str(" is distinct from "),
            Self::And => formatter.write_str(" and "),
            Self::Or => formatter.write_str(" or "),
        }
    }
}
