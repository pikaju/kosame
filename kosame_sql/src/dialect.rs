use std::fmt::Write;

pub trait Dialect {
    fn ident_esc() -> (&'static str, &'static str);
    fn fmt_bind_param(formatter: &mut impl Write, name: &str, ordinal: u32) -> std::fmt::Result;
}
