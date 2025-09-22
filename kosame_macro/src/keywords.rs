use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

mod kw {
    syn::custom_keyword!(create);
    syn::custom_keyword!(table);

    syn::custom_keyword!(not);
    syn::custom_keyword!(null);

    syn::custom_keyword!(default);

    syn::custom_keyword!(primary);
    syn::custom_keyword!(key);
    syn::custom_keyword!(references);
}

pub struct CreateTable {
    _create: kw::create,
    _table: kw::table,
}

impl Parse for CreateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _create: input.parse()?,
            _table: input.parse()?,
        })
    }
}

pub struct NotNull {
    _not: kw::not,
    _null: kw::null,
}

impl Parse for NotNull {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _not: input.parse()?,
            _null: input.parse()?,
        })
    }
}

pub struct PrimaryKey {
    _primary: kw::primary,
    _key: kw::key,
}

impl Parse for PrimaryKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _primary: input.parse()?,
            _key: input.parse()?,
        })
    }
}

pub struct AsIdent {
    r#as: Token![as],
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
            r#as: input.parse()?,
            ident: input.parse()?,
        })
    }
}
