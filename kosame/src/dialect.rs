use std::fmt::Write;

use crate::query::BindParamOrdinal;

pub trait Dialect {
    fn ident_esc() -> (&'static str, &'static str);
    fn fmt_bind_param(
        formatter: &mut impl Write,
        name: &str,
        ordinal: BindParamOrdinal,
    ) -> std::fmt::Result;
}

pub enum Postgres {}

impl Dialect for Postgres {
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

pub enum MySql {}

impl Dialect for MySql {
    fn ident_esc() -> (&'static str, &'static str) {
        ("`", "`")
    }

    fn fmt_bind_param(
        formatter: &mut impl Write,
        name: &str,
        _ordinal: BindParamOrdinal,
    ) -> std::fmt::Result {
        write!(formatter, ":{name}")
    }
}

pub enum Sqlite {}

impl Dialect for Sqlite {
    fn ident_esc() -> (&'static str, &'static str) {
        ("\"", "\"")
    }

    fn fmt_bind_param(
        formatter: &mut impl Write,
        name: &str,
        _ordinal: BindParamOrdinal,
    ) -> std::fmt::Result {
        write!(formatter, ":{name}")
    }
}

pub struct MsSql;

impl Dialect for MsSql {
    fn ident_esc() -> (&'static str, &'static str) {
        ("[", "]")
    }

    fn fmt_bind_param(
        formatter: &mut impl Write,
        name: &str,
        _ordinal: BindParamOrdinal,
    ) -> std::fmt::Result {
        write!(formatter, "@{name}")
    }
}
