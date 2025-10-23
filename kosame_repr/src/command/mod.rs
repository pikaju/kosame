mod select;

pub use select::*;

pub enum Command<'a> {
    Select(Select<'a>),
}

impl kosame_sql::FmtSql for Command<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        match self {
            Self::Select(inner) => inner.fmt_sql(formatter),
        }
    }
}
