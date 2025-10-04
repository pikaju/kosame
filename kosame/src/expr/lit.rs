use std::fmt::Write;

use crate::{dbms::Dialect, sql_formatter::SqlFormatter};

pub enum Lit {
    Int(i64),
    Float(f64),
    Str(&'static str),
    Bool(bool),
}

impl Lit {
    pub fn fmt_sql<D: Dialect>(&self, formatter: &mut SqlFormatter<D>) -> std::fmt::Result {
        match self {
            Self::Int(inner) => write!(formatter, "{}", inner),
            Self::Float(inner) => write!(formatter, "{}", inner),
            Self::Str(inner) => write!(formatter, "'{}'", inner.replace("'", "''")),
            Self::Bool(inner) => write!(formatter, "{}", inner),
        }
    }
}
