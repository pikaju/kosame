use syn::{
    Path, Token,
    parse::{Parse, ParseStream},
};

pub struct AsType {
    _type: Token![type],
    type_path: Path,
}

impl AsType {
    pub fn type_path(&self) -> &Path {
        &self.type_path
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Ok(if input.peek(Token![type]) {
            Some(input.parse()?)
        } else {
            None
        })
    }
}

impl Parse for AsType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _type: input.parse()?,
            type_path: input.parse()?,
        })
    }
}
