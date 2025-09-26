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

    pub fn to_sql_string(&self, buf: &mut String) {
        self.left.to_sql_string(buf);
        self.op.to_sql_string(buf);
        self.right.to_sql_string(buf);
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
}

impl BinOp {
    pub fn to_sql_string(&self, buf: &mut String) {
        match self {
            Self::Multiply => *buf += " * ",
            Self::Divide => *buf += " / ",
            Self::Modulo => *buf += " % ",
            Self::Add => *buf += " + ",
            Self::Subtract => *buf += " - ",
            Self::Eq => *buf += " = ",
            Self::Uneq => *buf += " <> ",
            Self::LessThan => *buf += " < ",
            Self::GreaterThan => *buf += " > ",
            Self::LessThanOrEq => *buf += " <= ",
            Self::GreaterThanOrEq => *buf += " >= ",
        }
    }
}
