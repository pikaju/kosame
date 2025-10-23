use std::ops::Deref;

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
