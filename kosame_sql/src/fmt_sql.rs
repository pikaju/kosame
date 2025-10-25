use crate::{Dialect, Formatter};

pub trait FmtSql {
    fn fmt_sql<D>(&self, formatter: &mut Formatter<D>) -> crate::Result
    where
        D: Dialect;

    fn to_sql_string<D>(&self) -> Result<String, crate::Error>
    where
        D: Dialect,
    {
        let mut result = String::new();
        let mut formatter = Formatter::<D>::new(&mut result);
        self.fmt_sql(&mut formatter)?;
        Ok(result)
    }
}
