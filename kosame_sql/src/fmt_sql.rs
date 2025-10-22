use crate::{Dialect, Formatter};

pub trait FmtSql {
    fn fmt_sql<D>(&self, formatter: &mut Formatter<D>) -> crate::Result
    where
        D: Dialect;
}
