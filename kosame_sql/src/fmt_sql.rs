use crate::{Dialect, Formatter};

pub trait FmtSql {
    fn fmt_sql<D: Dialect>(&self, formatter: &mut Formatter<D>) -> crate::Result;
}
