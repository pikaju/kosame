use proc_macro2::Span;
use syn::{Ident, Path, PathSegment, Token, punctuated::Punctuated};

pub trait PathExt {
    fn is_absolute(&self) -> bool;
    #[allow(unused)]
    fn is_relative(&self) -> bool;
    fn to_call_site(&self, nesting_levels: usize) -> Path;
}

impl PathExt for Path {
    fn is_absolute(&self) -> bool {
        self.leading_colon.is_some()
            || self
                .segments
                .iter()
                .next()
                .is_some_and(|segment| segment.ident == "crate")
    }

    fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    fn to_call_site(&self, nesting_levels: usize) -> Path {
        if self.is_absolute() {
            self.clone()
        } else {
            let mut result = Path {
                leading_colon: None,
                segments: Punctuated::<PathSegment, Token![::]>::new(),
            };
            result.segments.extend(std::iter::repeat_n(
                PathSegment::from(Ident::new("super", Span::call_site())),
                nesting_levels,
            ));
            result.segments.extend(self.segments.iter().cloned());
            result
        }
    }
}
