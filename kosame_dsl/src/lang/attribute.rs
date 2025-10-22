use proc_macro_error::emit_call_site_error;
use syn::{
    Ident, LitStr, Path, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::driver::Driver;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kosame);

    custom_keyword!(driver);
    custom_keyword!(rename);
    custom_keyword!(ty);
}

#[derive(Default)]
pub struct CustomMeta {
    pub driver: Option<MetaDriver>,
    pub rename: Option<MetaRename>,
    pub type_override: Option<MetaTypeOverride>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MetaLocation {
    TableMacro,
    Table,
    Column,
    QueryMacro,
    Query,
}

impl CustomMeta {
    pub fn parse_attrs(attrs: &[syn::Attribute], location: MetaLocation) -> syn::Result<Self> {
        let mut result = Self::default();

        for attr in attrs.iter() {
            if attr.path().is_ident("kosame") {
                let list = attr.meta.require_list()?;
                let items =
                    list.parse_args_with(Punctuated::<MetaItem, Token![,]>::parse_terminated)?;

                for item in items {
                    macro_rules! fill_or_error {
                        ($name:ident, $str:literal, $location_allowed:expr) => {{
                            if result.$name.is_some() {
                                return Err(syn::Error::new(
                                    $name.path.span,
                                    format!("duplicate use of meta argument `{}`", $str),
                                ));
                            }
                            if !($location_allowed) {
                                return Err(syn::Error::new(
                                    $name.path.span,
                                    format!(
                                        "meta argument `{}` not allowed in this location",
                                        $str
                                    ),
                                ));
                            }
                            result.$name = Some($name);
                        }};
                    }

                    match item {
                        MetaItem::Driver(driver) => {
                            fill_or_error!(
                                driver,
                                "driver",
                                location == MetaLocation::TableMacro
                                    || location == MetaLocation::QueryMacro
                            );
                        }
                        MetaItem::Rename(rename) => {
                            fill_or_error!(rename, "rename", location == MetaLocation::Column);
                        }
                        MetaItem::TypeOverride(type_override) => {
                            fill_or_error!(type_override, "ty", location == MetaLocation::Column);
                        }
                    }
                }
            }
        }

        match location {
            MetaLocation::TableMacro | MetaLocation::QueryMacro if result.driver.is_none() => {
                emit_call_site_error!(
                    "missing `driver` attribute, e.g. #[kosame(driver = \"tokio-postgres\")]"
                );
            }
            _ => {}
        }

        Ok(result)
    }
}

enum MetaItem {
    Driver(MetaDriver),
    Rename(MetaRename),
    TypeOverride(MetaTypeOverride),
}

impl Parse for MetaItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::driver) {
            Ok(Self::Driver(input.parse()?))
        } else if lookahead.peek(kw::rename) {
            Ok(Self::Rename(input.parse()?))
        } else if lookahead.peek(kw::ty) {
            Ok(Self::TypeOverride(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct MetaDriver {
    pub path: kw::driver,
    pub eq_token: Token![=],
    pub value: LitStr,
}

impl Parse for MetaDriver {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            eq_token: input.parse()?,
            value: {
                let value: LitStr = input.parse()?;
                if Driver::try_from(value.value().as_ref()).is_err() {
                    return Err(syn::Error::new(value.span(), "unknown driver value"));
                }
                value
            },
        })
    }
}

pub struct MetaRename {
    pub path: kw::rename,
    pub eq_token: Token![=],
    pub value: Ident,
}

impl Parse for MetaRename {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct MetaTypeOverride {
    pub path: kw::ty,
    pub eq_token: Token![=],
    pub value: Path,
}

impl Parse for MetaTypeOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
