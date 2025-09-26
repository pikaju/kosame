use syn::{
    Path, Token,
    parse::{Parse, ParseBuffer, ParseStream},
};

pub struct TypeOverride {
    _colon: Token![:],
    type_path: Path,
}

impl TypeOverride {
    pub fn type_path(&self) -> &Path {
        &self.type_path
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Ok(if input.peek(Token![:]) {
            Some(input.parse()?)
        } else {
            None
        })
    }

    pub fn peek(input: &ParseBuffer<'_>) -> bool {
        input.peek(Token![:])
    }
}

impl Parse for TypeOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _colon: input.parse()?,
            type_path: input.parse()?,
        })
    }
}
