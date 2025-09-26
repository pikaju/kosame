use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
};

use super::Expr;

pub struct Binary {
    lhs: Box<Expr>,
    op: BinOp,
    rhs: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, op: BinOp, right: Expr) -> Self {
        Self {
            lhs: Box::new(left),
            op,
            rhs: Box::new(right),
        }
    }
}

impl ToTokens for Binary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lhs = &self.lhs;
        let op = &self.op;
        let rhs = &self.rhs;
        quote! {
            ::kosame::expr::Binary::new(&#lhs, #op, &#rhs)
        }
        .to_tokens(tokens);
    }
}

#[derive(PartialEq, Eq)]
pub enum Associativity {
    Left,
}

#[allow(dead_code)]
pub enum BinOp {
    Add(Token![+]),
    Subtract(Token![-]),
    Multiply(Token![*]),
    Divide(Token![/]),
}

impl BinOp {
    pub fn peek(input: ParseStream) -> Option<BinOp> {
        input.fork().parse::<BinOp>().ok()
    }

    pub fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    pub fn precedence(&self) -> u32 {
        match self {
            Self::Add(_) => 1,
            Self::Subtract(_) => 1,
            Self::Multiply(_) => 2,
            Self::Divide(_) => 2,
        }
    }
}

impl Parse for BinOp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![+]) {
            Ok(Self::Add(input.parse()?))
        } else if lookahead.peek(Token![-]) {
            Ok(Self::Subtract(input.parse()?))
        } else if lookahead.peek(Token![*]) {
            Ok(Self::Multiply(input.parse()?))
        } else if lookahead.peek(Token![/]) {
            Ok(Self::Divide(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for BinOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Add(_) => quote! { ::kosame::expr::BinOp::Add }.to_tokens(tokens),
            Self::Subtract(_) => quote! { ::kosame::expr::BinOp::Subtract }.to_tokens(tokens),
            Self::Multiply(_) => quote! { ::kosame::expr::BinOp::Multiply }.to_tokens(tokens),
            Self::Divide(_) => quote! { ::kosame::expr::BinOp::Divide }.to_tokens(tokens),
        }
    }
}
