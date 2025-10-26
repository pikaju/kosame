use crate::expr::BindParam;

pub trait Visitor<'a> {
    fn visit_bind_param(&mut self, _bind_param: &'a BindParam) {}
}
