use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

pub struct Alias {
    _as: Token![as],
    ident: Ident,
}

impl Alias {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        Ok(if input.peek(Token![as]) {
            Some(input.parse()?)
        } else {
            None
        })
    }
}

impl Parse for Alias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _as: input.parse()?,
            ident: input.parse()?,
        })
    }
}
