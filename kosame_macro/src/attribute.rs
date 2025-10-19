use proc_macro_error::emit_error;
use syn::{
    LitStr, Path, Token, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Bracket, Paren},
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kosame);

    custom_keyword!(rename);
    custom_keyword!(rename_all);
}

pub enum Attribute {
    Known(Known),
    Other(syn::Attribute),
}

impl Attribute {
    pub fn as_known(&self) -> Option<&Known> {
        if let Self::Known(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn parse_outer(input: ParseStream) -> syn::Result<Vec<Attribute>> {
        let mut attrs = Vec::new();
        while input.peek(Token![#]) {
            attrs.push(input.parse()?);
        }
        Ok(attrs)
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let _pound_token: Token![#] = fork.parse()?;
        let content;
        let _bracket = bracketed!(content in fork);
        if content.peek(kw::kosame) {
            Ok(Self::Known(input.parse()?))
        } else {
            let content;
            Ok(Self::Other(syn::Attribute {
                pound_token: input.parse()?,
                style: syn::AttrStyle::Outer,
                bracket_token: bracketed!(content in input),
                meta: content.parse()?,
            }))
        }
    }
}

pub struct Known {
    pound_token: Token![#],
    bracket_token: Bracket,
    path: kw::kosame,
    paren: Paren,
    meta: MetaList,
}

impl Parse for Known {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bracket_content;
        let paren_content;
        Ok(Self {
            pound_token: input.parse()?,
            bracket_token: bracketed!(bracket_content in input),
            path: bracket_content.parse()?,
            paren: parenthesized!(paren_content in bracket_content),
            meta: paren_content.parse()?,
        })
    }
}

pub struct MetaList {
    items: Punctuated<MetaItem, Token![,]>,
}

impl MetaList {
    pub fn rename(&self) -> Option<&LitStr> {
        self.items.iter().find_map(|item| match item {
            MetaItem::Rename { value, .. } => Some(value),
            _ => None,
        })
    }

    pub fn rename_all(&self) -> Option<&LitStr> {
        self.items.iter().find_map(|item| match item {
            MetaItem::RenameAll { value, .. } => Some(value),
            _ => None,
        })
    }

    pub fn type_override(&self) -> Option<&Path> {
        self.items.iter().find_map(|item| match item {
            MetaItem::TypeOverride { value, .. } => Some(value),
            _ => None,
        })
    }
}

impl Parse for MetaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items = input.parse_terminated(MetaItem::parse, Token![,])?;
        for (index, item) in items.iter().enumerate() {
            for i in 0..index {
                if items[i].path().get_ident().unwrap() == item.path().get_ident().unwrap() {
                    emit_error!(item.path().span(), "duplicate meta item",);
                }
            }
        }
        Ok(Self { items })
    }
}

pub enum MetaItem {
    Rename {
        path: Path,
        eq_token: Token![=],
        value: LitStr,
    },
    RenameAll {
        path: Path,
        eq_token: Token![=],
        value: LitStr,
    },
    TypeOverride {
        path: Path,
        eq_token: Token![=],
        value: Path,
    },
}

impl MetaItem {
    fn path(&self) -> &Path {
        match self {
            Self::Rename { path, .. } => path,
            Self::RenameAll { path, .. } => path,
            Self::TypeOverride { path, .. } => path,
        }
    }
}

impl Parse for MetaItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::rename) {
            Ok(Self::Rename {
                path: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::rename_all) {
            Ok(Self::RenameAll {
                path: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::rename_all) {
            Ok(Self::TypeOverride {
                path: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
