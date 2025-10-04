pub enum Dialect {}

impl crate::sql::Dialect for Dialect {
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
