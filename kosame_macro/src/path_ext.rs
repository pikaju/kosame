use syn::Path;

pub trait PathExt {
    fn is_absolute(&self) -> bool;
    fn is_relative(&self) -> bool;
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
}
