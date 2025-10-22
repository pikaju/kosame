pub enum Dialect {}

impl crate::Dialect for Dialect {
    fn ident_esc() -> (&'static str, &'static str) {
        ("\"", "\"")
    }

    fn fmt_bind_param(formatter: &mut impl Write, name: &str, _ordinal: u32) -> std::fmt::Result {
        write!(formatter, ":{name}")
    }
}
