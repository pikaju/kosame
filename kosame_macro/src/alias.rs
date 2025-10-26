use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

pub struct Alias {
    pub _as: Token![as],
    pub ident: Ident,
}

impl Alias {
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
