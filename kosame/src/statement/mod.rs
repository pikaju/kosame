pub use kosame_repr::command::*;
use pollster::FutureExt;

use crate::{driver::Connection, params::Params};

pub trait Statement {
    type Params: std::fmt::Debug;
    type Row;

    const REPR: Command<'static>;

    fn repr(&self) -> &'static Command<'static> {
        &Self::REPR
    }

    fn params(&self) -> &Self::Params;

    fn exec<'c, C>(&self, connection: &mut C) -> impl Future<Output = crate::Result<Vec<Self::Row>>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        async {
            use kosame_sql::FmtSql;
            let mut sql = String::new();
            let mut formatter = kosame_sql::Formatter::<C::Dialect>::new(&mut sql);
            self.repr().fmt_sql(&mut formatter)?;

            let rows = connection
                .query(&sql, &self.params().to_driver())
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(rows.iter().map(Self::Row::from).collect())
        }
    }

    fn exec_sync<'c, C>(&self, connection: &mut C) -> crate::Result<Vec<Self::Row>>
    where
        C: Connection,
        Self::Params: Params<C::Params<'c>>,
        for<'b> Self::Row: From<&'b C::Row>,
    {
        self.exec(connection).block_on()
    }
}
