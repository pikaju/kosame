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

pub struct Formatter<'a, D> {
    buf: &'a mut (dyn Write + 'a),
    _dialect: std::marker::PhantomData<D>,
}

impl<'a, D> Formatter<'a, D>
where
    D: Dialect,
{
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            buf,
            _dialect: Default::default(),
        }
    }

    pub fn write_ident(&mut self, ident: &str) -> std::fmt::Result {
        let (prefix, suffix) = D::ident_esc();
        write!(self, "{prefix}{ident}{suffix}")
    }

    pub fn write_bind_param(&mut self, name: &str, ordinal: u32) -> Result<(), std::fmt::Error> {
        D::fmt_bind_param(self, name, ordinal)
    }
}

impl<'a, D> Write for Formatter<'a, D> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}
