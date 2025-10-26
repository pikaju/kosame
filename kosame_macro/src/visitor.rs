use syn::Path;

use crate::expr::BindParam;

pub trait Visitor<'a> {
    fn visit_bind_param(&mut self, _bind_param: &'a BindParam) {}
    fn visit_table_ref(&mut self, _table_ref: &'a Path) {}
}
