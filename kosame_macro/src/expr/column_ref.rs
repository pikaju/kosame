use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

pub struct ColumnRef {
    name: Ident,
}

impl Parse for ColumnRef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}
