use syn::{
    Attribute, LitStr, Meta, MetaList, Path, Token, parenthesized,
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kosame);

    custom_keyword!(rename);
    custom_keyword!(rename_all);
}

pub struct ParsedAttributes {
    attrs: Vec<Attribute>,
    rename: Option<LitStr>,
    rename_all: Option<LitStr>,
    type_override: Option<Path>,
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
                        MetaItem::Rename { path, value, .. } => {
                            if rename.is_some() {
                                return Err(syn::Error::new(
                                    path.span(),
                                    "duplicate meta field `rename`",
                                ));
                            }
                            rename = Some(value);
                        }
                        MetaItem::RenameAll { path, value, .. } => {
                            if rename_all.is_some() {
                                return Err(syn::Error::new(
                                    path.span(),
                                    "duplicate meta field `rename_all`",
                                ));
                            }
                            rename_all = Some(value);
                        }
                        MetaItem::TypeOverride { path, value, .. } => {
                            if type_override.is_some() {
                                return Err(syn::Error::new(
                                    path.span(),
                                    "duplicate meta field `type_override`",
                                ));
                            }
                            type_override = Some(value);
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
    Rename {
        path: Path,
        _eq_token: Token![=],
        value: LitStr,
    },
    RenameAll {
        path: Path,
        _eq_token: Token![=],
        value: LitStr,
    },
    TypeOverride {
        path: Path,
        _eq_token: Token![=],
        value: Path,
    },
}

impl Parse for MetaItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::rename) {
            Ok(Self::Rename {
                path: input.parse()?,
                _eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::rename_all) {
            Ok(Self::RenameAll {
                path: input.parse()?,
                _eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(Token![type]) {
            Ok(Self::TypeOverride {
                path: input.parse()?,
                _eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
