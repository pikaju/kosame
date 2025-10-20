use std::fmt::Write;

use crate::sql;

pub enum Lit {
    Int(i64),
    Float(f64),
    Str(&'static str),
    Bool(bool),
    Null,
}

impl Lit {
    #[inline]
    pub fn fmt_sql<D: sql::Dialect>(&self, formatter: &mut sql::Formatter<D>) -> std::fmt::Result {
        match self {
            Self::Int(inner) => write!(formatter, "{}", inner),
            Self::Float(inner) => write!(formatter, "{}", inner),
            Self::Str(inner) => write!(formatter, "'{}'", inner.replace("'", "''")),
            Self::Bool(inner) => write!(formatter, "{}", inner),
            Self::Null => formatter.write_str("null"),
        }
    }
}
