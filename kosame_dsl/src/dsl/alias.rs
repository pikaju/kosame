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
        Self::peek(input).then(|| input.parse()).transpose()
    }

    pub fn peek(input: ParseStream) -> bool {
        input.peek(Token![as])
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
