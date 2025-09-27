mod binary;
mod bind_param;
mod column_ref;
mod lit;
mod paren;

mod visitor;

pub use binary::{Associativity, BinOp, Binary};
pub use bind_param::BindParam;
pub use column_ref::ColumnRef;
pub use lit::Lit;
pub use paren::Paren;
pub use visitor::Visitor;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

pub enum Expr {
    Binary(Binary),
    BindParam(BindParam),
    ColumnRef(ColumnRef),
    Lit(Lit),
    Paren(Paren),
}

impl Expr {
    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        macro_rules! branches {
            ($($variant:ident)*) => {
                match self {
                    $(Self::$variant(inner) => inner.accept(visitor)),*
                }
            };
        }

        branches!(Binary BindParam ColumnRef Lit Paren);
    }

    fn parse_prefix(input: ParseStream) -> syn::Result<Expr> {
        if input.peek(syn::token::Paren) {
            Ok(Expr::Paren(input.parse()?))
        } else if BindParam::peek(input) {
            Ok(Expr::BindParam(input.parse()?))
        } else if input.fork().parse::<Lit>().is_ok() {
            Ok(Expr::Lit(input.parse()?))
        } else if input.fork().parse::<ColumnRef>().is_ok() {
            Ok(Expr::ColumnRef(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected expression"))
        }
    }

    // fn peek_terminator(input: ParseStream) -> bool {
    //     Alias::peek(input) || TypeOverride::peek(input) || input.peek(Token![,])
    // }

    fn parse_expr(input: ParseStream, min_precedence: u32) -> syn::Result<Expr> {
        let mut lhs = Self::parse_prefix(input)?;

        loop {
            if let Some(bin_op) = BinOp::peek(input) {
                let precedence = bin_op.precedence();
                if precedence < min_precedence {
                    break;
                }

                let next_precedence = if bin_op.associativity() == Associativity::Left {
                    precedence + 1
                } else {
                    precedence
                };

                let bin_op = input.parse()?;
                let rhs = Self::parse_expr(input, next_precedence)?;

                lhs = Expr::Binary(Binary::new(lhs, bin_op, rhs))
            } else {
                break;
            }
        }

        Ok(lhs)
    }
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Self::parse_expr(input, 0)
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        macro_rules! branches {
            ($($variant:ident)*) => {
                match self {
                    $(Self::$variant(inner) => quote! { ::kosame::expr::Expr::$variant(#inner) }.to_tokens(tokens)),*
                }
            };
        }

        branches!(Binary BindParam ColumnRef Lit Paren);
    }
}
