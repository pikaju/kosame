use std::fmt::Write;

use crate::query::BindParamOrdinal;

pub enum Dialect {}

impl crate::sql::Dialect for Dialect {
    fn ident_esc() -> (&'static str, &'static str) {
        ("\"", "\"")
    }

    fn fmt_bind_param(
        formatter: &mut impl Write,
        _name: &str,
        ordinal: BindParamOrdinal,
    ) -> std::fmt::Result {
        write!(formatter, "${}", ordinal + 1)
    }
}
