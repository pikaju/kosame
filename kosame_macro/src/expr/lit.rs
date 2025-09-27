use super::Visitor;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

pub enum Lit {
    Int(syn::LitInt),
    Float(syn::LitFloat),
    Str(syn::LitStr),
    Bool(syn::LitBool),
}

impl Lit {
    pub fn accept(&self, _visitor: &mut impl Visitor) {}
}

impl Parse for Lit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
        };
        token_stream.to_tokens(tokens);
    }
}
