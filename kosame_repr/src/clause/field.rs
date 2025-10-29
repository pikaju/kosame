use std::{fmt::Write, ops::Deref};

use crate::expr::Expr;

pub struct Field<'a> {
    expr: Expr<'a>,
    alias: Option<&'a str>,
}

impl<'a> Field<'a> {
    #[inline]
    pub const fn new(expr: Expr<'a>, alias: Option<&'a str>) -> Self {
        Self { expr, alias }
    }

    #[inline]
    pub const fn expr(&self) -> &Expr<'a> {
        &self.expr
    }

    #[inline]
    pub const fn alias(&self) -> Option<&'a str> {
        self.alias
    }
}

impl kosame_sql::FmtSql for Field<'_> {
    #[inline]
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        self.expr.fmt_sql(formatter)?;
        if let Some(alias) = &self.alias {
            formatter.write_str(" as ")?;
            formatter.write_ident(alias)?;
        }
        Ok(())
    }
}

pub struct Fields<'a>(&'a [Field<'a>]);

impl<'a> Fields<'a> {
    #[inline]
    pub const fn new(fields: &'a [Field<'a>]) -> Self {
        Self(fields)
    }
}

impl<'a> Deref for Fields<'a> {
    type Target = &'a [Field<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl kosame_sql::FmtSql for Fields<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        for (index, field) in self.0.iter().enumerate() {
            field.fmt_sql(formatter)?;
            if index != self.0.len() - 1 {
                formatter.write_str(", ")?;
            }
        }
        Ok(())
    }
}
