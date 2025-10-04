use std::fmt::Write;

use crate::sql;

use super::Expr;

pub struct Unary {
    op: UnaryOp,
    operand: &'static Expr,
}

impl Unary {
    pub const fn new(op: UnaryOp, operand: &'static Expr) -> Self {
        Self { op, operand }
    }

    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        match self.op.position() {
            Position::Prefix => {
                self.op.fmt_sql(formatter)?;
                self.operand.fmt_sql(formatter)?;
            }
            Position::Postfix => {
                self.operand.fmt_sql(formatter)?;
                self.op.fmt_sql(formatter)?;
            }
        }
        Ok(())
    }
}

pub enum UnaryOp {
    Not,
}

enum Position {
    Prefix,
    Postfix,
}

impl UnaryOp {
    pub fn position(&self) -> Position {
        match self {
            Self::Not => Position::Prefix,
        }
    }

    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        match self {
            Self::Not => formatter.write_str("not "),
        }
    }
}
