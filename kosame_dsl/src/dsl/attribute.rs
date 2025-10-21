use syn::{
    Ident, Path, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(kosame);

    custom_keyword!(rename);
    custom_keyword!(ty);
}

#[derive(Default)]
pub struct CustomMeta {
    rename: Option<Rename>,
    type_override: Option<TypeOverride>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MetaLocation {
    TableMacro,
    Table,
    Column,
    QueryMacro,
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

        Ok(result)
    }

    pub fn rename(&self) -> Option<&Ident> {
        self.rename.as_ref().map(|inner| &inner.value)
    }

    pub fn type_override(&self) -> Option<&Path> {
        self.type_override.as_ref().map(|inner| &inner.value)
    }
}

enum MetaItem {
    Rename(Rename),
    TypeOverride(TypeOverride),
}

impl MetaItem {
    fn allowed_in_location(&self, location: MetaLocation) -> bool {
        match self {
            Self::Rename(_) => match location {
                MetaLocation::Table | MetaLocation::Column => true,
                _ => false,
            },
            Self::TypeOverride(_) => match location {
                MetaLocation::Table | MetaLocation::Column => true,
                _ => false,
            },
        }
    }
}

impl Parse for MetaItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::rename) {
            Ok(Self::Rename(input.parse()?))
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
    value: Ident,
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
