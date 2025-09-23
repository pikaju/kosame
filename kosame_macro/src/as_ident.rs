use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

pub struct AsIdent {
    _as: Token![as],
    ident: Ident,
}

impl AsIdent {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl Parse for AsIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _as: input.parse()?,
            ident: input.parse()?,
        })
    }
}
