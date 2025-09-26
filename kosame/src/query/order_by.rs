use crate::expr::Expr;

pub struct OrderBy {
    entries: &'static [OrderByEntry],
}

impl OrderBy {
    pub const fn new(entries: &'static [OrderByEntry]) -> Self {
        Self { entries }
    }

    pub(crate) fn to_sql_string(&self, buf: &mut String) {
        *buf += " order by ";
        for (index, entry) in self.entries.iter().enumerate() {
            entry.to_sql_string(buf);
            if index != self.entries.len() - 1 {
                *buf += ", ";
            }
        }
    }
}

pub struct OrderByEntry {
    expr: Expr,
    dir: Option<OrderByDir>,
    nulls: Option<OrderByNulls>,
}

impl OrderByEntry {
    pub const fn new(expr: Expr, dir: Option<OrderByDir>, nulls: Option<OrderByNulls>) -> Self {
        Self { expr, dir, nulls }
    }

    pub(crate) fn to_sql_string(&self, buf: &mut String) {
        self.expr.to_sql_string(buf);
        match self.dir {
            Some(OrderByDir::Asc) => *buf += " asc",
            Some(OrderByDir::Desc) => *buf += " desc",
            None => {}
        }
        match self.nulls {
            Some(OrderByNulls::First) => *buf += " nulls first",
            Some(OrderByNulls::Last) => *buf += " nulls last",
            None => {}
        }
    }
}

pub enum OrderByDir {
    Asc,
    Desc,
}

pub enum OrderByNulls {
    First,
    Last,
}
