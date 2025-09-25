use syn::{
    Token,
    parse::{Parse, ParseStream},
};

use super::Expr;

pub struct Binary {
    left: Box<Expr>,
    op: BinOp,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, op: BinOp, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Associativity {
    Left,
}

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
