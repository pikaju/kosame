use syn::{
    Attribute, LitStr, Path, Token,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kosame);

    custom_keyword!(rename);
    custom_keyword!(rename_all);
    custom_keyword!(ty);
}

pub struct ParsedAttributes {
    attrs: Vec<Attribute>,
    rename: Option<Rename>,
    rename_all: Option<RenameAll>,
    type_override: Option<TypeOverride>,
}

impl ParsedAttributes {
    pub fn require_no_global(&self) -> Result<(), syn::Error> {
        match &self.rename_all {
            Some(rename_all) => Err(syn::Error::new(
                rename_all.path.span(),
                "global attributes are not allowed in this position",
            )),
            _ => Ok(()),
        }
    }

    pub fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    pub fn rename(&self) -> Option<&LitStr> {
        self.rename.as_ref().map(|v| &v.value)
    }

    pub fn rename_all(&self) -> Option<&LitStr> {
        self.rename_all.as_ref().map(|v| &v.value)
    }

    pub fn type_override(&self) -> Option<&Path> {
        self.type_override.as_ref().map(|v| &v.value)
    }
}

impl Parse for ParsedAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        let mut rename = None;
        let mut rename_all = None;
        let mut type_override = None;
        for attr in attrs.iter() {
            if attr.path().is_ident("kosame") {
                let list = attr.meta.require_list()?;
                let items = Punctuated::<MetaItem, Token![,]>::parse_terminated
                    .parse2(list.tokens.clone())?;

                for item in items {
                    match item {
                        MetaItem::Rename(v) => {
                            if rename.is_some() {
                                return Err(syn::Error::new(
                                    v.path.span(),
                                    "duplicate meta field `rename`",
                                ));
                            }
                            rename = Some(v);
                        }
                        MetaItem::RenameAll(v) => {
                            if rename_all.is_some() {
                                return Err(syn::Error::new(
                                    v.path.span(),
                                    "duplicate meta field `rename_all`",
                                ));
                            }
                            rename_all = Some(v);
                        }
                        MetaItem::TypeOverride(v) => {
                            if type_override.is_some() {
                                return Err(syn::Error::new(
                                    v.path.span(),
                                    "duplicate meta field `type_override`",
                                ));
                            }
                            type_override = Some(v);
                        }
                    }
                }
            }
        }
        Ok(Self {
            attrs,
            rename,
            rename_all,
            type_override,
        })
    }
}

enum MetaItem {
    Rename(Rename),
    RenameAll(RenameAll),
    TypeOverride(TypeOverride),
}

impl Parse for MetaItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::rename) {
            Ok(Self::Rename(input.parse()?))
        } else if lookahead.peek(kw::rename_all) {
            Ok(Self::RenameAll(input.parse()?))
        } else if lookahead.peek(kw::ty) {
            Ok(Self::TypeOverride(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct Rename {
    path: kw::rename,
    _eq_token: Token![=],
    value: LitStr,
}

impl Parse for Rename {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct RenameAll {
    path: kw::rename_all,
    _eq_token: Token![=],
    value: LitStr,
}

impl Parse for RenameAll {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct TypeOverride {
    path: kw::ty,
    _eq_token: Token![=],
    value: Path,
}

impl Parse for TypeOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
