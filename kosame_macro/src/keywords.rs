use syn::parse::Parse;

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
    create: kw::create,
    table: kw::table,
}

impl Parse for CreateTable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            create: input.parse()?,
            table: input.parse()?,
        })
    }
}

pub struct NotNull {
    not: kw::not,
    null: kw::null,
}

impl Parse for NotNull {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            not: input.parse()?,
            null: input.parse()?,
        })
    }
}

pub struct PrimaryKey {
    primary: kw::primary,
    key: kw::key,
}

impl Parse for PrimaryKey {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            primary: input.parse()?,
            key: input.parse()?,
        })
    }
}
