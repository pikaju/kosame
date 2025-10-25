use std::fmt::Write;

use crate::{expr::Expr, schema::Table};

pub struct From<'a> {
    table: &'a Table<'a>,
}

impl<'a> From<'a> {
    #[inline]
    pub const fn new(table: &'a Table<'a>) -> Self {
        Self { table }
    }

    #[inline]
    pub const fn table(&self) -> &'a Table<'a> {
        self.table
    }
}

impl kosame_sql::FmtSql for From<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" from ")?;
        formatter.write_ident(self.table.name())?;

        Ok(())
    }
}

pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl kosame_sql::FmtSql for JoinType {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        match self {
            Self::Inner => formatter.write_str(" inner join "),
            Self::Left => formatter.write_str(" left join "),
            Self::Right => formatter.write_str(" right join "),
            Self::Full => formatter.write_str(" full join "),
        }
    }
}

pub struct TableAlias<'a> {
    alias: &'a str,
    columns: Option<&'a [&'a str]>,
}

impl kosame_sql::FmtSql for TableAlias<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" as ")?;
        formatter.write_ident(self.alias)?;
        if let Some(columns) = self.columns {
            formatter.write_str(" (")?;
            for (index, column) in columns.iter().enumerate() {
                formatter.write_ident(column)?;
                if index != columns.len() - 1 {
                    formatter.write_str(", ")?;
                }
            }
            formatter.write_str(")")?;
        }
        Ok(())
    }
}

pub enum FromItem<'a> {
    Table {
        table: &'a Table<'a>,
        alias: Option<TableAlias<'a>>,
    },
    // Subquery {
    //     lateral: bool,
    //     select: Select<'a>,
    //     alias: Option<TableAlias<'a>>,
    // },
    Join {
        left: &'a FromItem<'a>,
        join_type: JoinType,
        right: &'a FromItem<'a>,
        on: Expr<'a>,
    },
    // NaturalJoin {
    //     left: &'a FromItem<'a>,
    //     join_type: JoinType,
    //     right: &'a FromItem<'a>,
    // },
    // CrossJoin {
    //     left: &'a FromItem<'a>,
    //     right: &'a FromItem<'a>,
    // },
}

impl kosame_sql::FmtSql for FromItem<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        match self {
            Self::Table { table, alias } => {
                formatter.write_ident(table.name())?;
                if let Some(alias) = alias {
                    alias.fmt_sql(formatter)?;
                }
            }
            Self::Join {
                left,
                join_type,
                right,
                on,
            } => {
                left.fmt_sql(formatter)?;
                join_type.fmt_sql(formatter)?;
                right.fmt_sql(formatter)?;
                formatter.write_str(" ")?;
                on.fmt_sql(formatter)?;
            }
        }

        Ok(())
    }
}
