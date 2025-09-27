use super::BindParam;

pub trait Visitor<'a> {
    fn visit_bind_param(&mut self, bind_param: &'a BindParam) {}
}
