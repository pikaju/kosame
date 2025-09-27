use std::collections::HashMap;

use super::node;
use crate::expr;
use syn::Ident;

pub struct BindParam<'a> {
    name: &'a Ident,
    ordinal: u32,
}

pub struct BindParamsBuilder<'a> {
    params: HashMap<&'a Ident, &'a expr::BindParam>,
}

impl BindParamsBuilder<'_> {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
}

impl<'a> BindParamsBuilder<'a> {
    pub fn build(self) -> BindParams<'a> {
        self.params.into_iter().map(|(k, _)| k).collect()
    }
}

impl<'a> node::Visitor<'a> for BindParamsBuilder<'a> {
    fn visit_field(&mut self, field: &'a super::QueryField) {
        if let super::QueryField::Expr { expr, .. } = field {
            expr.accept(self);
        }
    }

    fn visit_filter(&mut self, filter: &'a Option<super::Filter>) {
        if let Some(filter) = filter {
            filter.expr().accept(self);
        }
    }

    fn visit_order_by(&mut self, order_by: &'a Option<super::OrderBy>) {
        if let Some(order_by) = order_by {
            filter.expr().accept(self);
        }
    }

    fn visit_limit(&mut self, _limit: &'a Option<super::Limit>) {}
    fn visit_offset(&mut self, _offset: &'a Option<super::Offset>) {}
}

impl<'a> expr::Visitor<'a> for BindParamsBuilder<'a> {
    fn visit_bind_param(&mut self, bind_param: &'a expr::BindParam) {
        self.params.insert(bind_param.name(), bind_param);
    }
}

pub struct BindParams<'a> {
    params: Box<[BindParam<'a>]>,
}

impl<'a> FromIterator<&'a Ident> for BindParams<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Ident>>(iter: T) -> Self {
        Self {
            params: iter
                .into_iter()
                .enumerate()
                .map(|(index, ident)| BindParam {
                    name: ident,
                    ordinal: index as u32 + 1,
                })
                .collect(),
        }
    }
}
