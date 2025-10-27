use std::fmt::Write;

use crate::expr::Expr;

pub struct GroupBy<'a> {
    items: &'a [GroupByItem<'a>],
}

impl<'a> GroupBy<'a> {
    #[inline]
    pub const fn new(items: &'a [GroupByItem]) -> Self {
        Self { items }
    }
}

impl kosame_sql::FmtSql for GroupBy<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_str(" group by ")?;
        for (index, item) in self.items.iter().enumerate() {
            item.fmt_sql(formatter)?;
            if index != self.items.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        Ok(())
    }
}

pub struct GroupByItem<'a> {
    expr: Expr<'a>,
}

impl<'a> GroupByItem<'a> {
    #[inline]
    pub const fn new(expr: Expr<'a>) -> Self {
        Self { expr }
    }
}

impl kosame_sql::FmtSql for GroupByItem<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        self.expr.fmt_sql(formatter)?;
        Ok(())
    }
}
