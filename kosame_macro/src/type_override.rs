use syn::{
    Path, Token,
    parse::{Parse, ParseBuffer, ParseStream},
};

pub struct TypeOverride {
    _type: Token![type],
    type_path: Path,
}

impl TypeOverride {
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

    pub fn peek(input: &ParseBuffer<'_>) -> bool {
        input.peek(Token![type])
    }
}

impl Parse for TypeOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _type: input.parse()?,
            type_path: input.parse()?,
        })
    }
}
