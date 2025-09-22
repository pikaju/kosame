use convert_case::Casing;
use proc_macro2::Span;
use syn::Ident;

#[derive(Clone)]
pub struct RelationPath {
    segments: Vec<Ident>,
}

impl RelationPath {
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    pub fn append(&mut self, segment: Ident) {
        self.segments.push(segment);
    }

    pub fn segments(&self) -> &[Ident] {
        &self.segments
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn to_struct_name(&self, prefix: &str) -> Ident {
        let mut struct_name = prefix.to_string();
        for segment in &self.segments {
            struct_name += &segment.to_string().to_case(convert_case::Case::Pascal);
        }
        Ident::new(&struct_name, Span::call_site())
    }

    pub fn to_module_name(&self, prefix: &str) -> Ident {
        let mut module_name = prefix.to_string();
        for segment in &self.segments {
            module_name += "_";
            module_name += &segment.to_string();
        }
        Ident::new(&module_name, Span::call_site())
    }
}
