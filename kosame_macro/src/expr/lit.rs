use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub enum Lit {
    Int(syn::LitInt),
    Float(syn::LitFloat),
    Str(syn::LitStr),
    Bool(syn::LitBool),
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
                    format!(
                        "unsupported literal type `{}`",
                        lit.to_token_stream().to_string()
                    ),
                ));
            }
        })
    }
}
