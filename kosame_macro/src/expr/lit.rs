use super::Visitor;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(null);
}

#[allow(dead_code)]
pub enum Lit {
    Int(syn::LitInt),
    Float(syn::LitFloat),
    Str(syn::LitStr),
    Bool(syn::LitBool),
    Null(kw::null),
}

impl Lit {
    pub fn accept<'a>(&'a self, _visitor: &mut impl Visitor<'a>) {}
}

impl Parse for Lit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::null) {
            return Ok(Self::Null(input.parse()?));
        }
        let lit = input.parse::<syn::Lit>()?;
        Ok(match lit {
            syn::Lit::Int(inner) => Self::Int(inner),
            syn::Lit::Float(inner) => Self::Float(inner),
            syn::Lit::Str(inner) => Self::Str(inner),
            syn::Lit::Bool(inner) => Self::Bool(inner),
            _ => {
                return Err(syn::Error::new(
                    lit.span(),
                    format!("unsupported literal type `{}`", lit.to_token_stream()),
                ));
            }
        })
    }
}

impl ToTokens for Lit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token_stream = match self {
            Self::Int(inner) => quote! { kosame::expr::Lit::Int(#inner as i64) },
            Self::Float(inner) => quote! { kosame::expr::Lit::Float(#inner as f64) },
            Self::Str(inner) => quote! { kosame::expr::Lit::Str(#inner) },
            Self::Bool(inner) => quote! { kosame::expr::Lit::Bool(#inner) },
            Self::Null(_) => quote! { kosame::expr::Lit::Null },
        };
        token_stream.to_tokens(tokens);
    }
}
