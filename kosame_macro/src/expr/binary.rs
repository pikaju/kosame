use super::Expr;
use super::Visitor;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream},
};

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

    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.lhs.accept(visitor);
        self.rhs.accept(visitor);
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
    // multiplication, division, modulo
    Multiply(Token![*]),
    Divide(Token![/]),
    Modulo(Token![%]),
    // addition, subtraction
    Add(Token![+]),
    Subtract(Token![-]),
    // comparison operators
    Eq(Token![=]),
    Uneq(Token![<], Token![>]),
    LessThan(Token![<]),
    GreaterThan(Token![>]),
    LessThanOrEq(Token![<], Token![=]),
    GreaterThanOrEq(Token![>], Token![=]),
}

impl BinOp {
    pub fn peek(input: ParseStream) -> Option<BinOp> {
        input.fork().parse::<BinOp>().ok()
    }

    pub fn associativity(&self) -> Associativity {
        Associativity::Left
    }

    pub fn precedence(&self) -> u32 {
        // Taken from https://www.postgresql.org/docs/18/sql-syntax-lexical.html#SQL-PRECEDENCE
        match self {
            Self::Multiply(_) => 9,
            Self::Divide(_) => 9,
            Self::Modulo(_) => 9,
            Self::Add(_) => 8,
            Self::Subtract(_) => 8,
            Self::Eq(_) => 5,
            Self::Uneq(..) => 5,
            Self::LessThan(_) => 5,
            Self::GreaterThan(_) => 5,
            Self::LessThanOrEq(..) => 5,
            Self::GreaterThanOrEq(..) => 5,
        }
    }
}

impl Parse for BinOp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![+]) {
            return Ok(Self::Add(input.parse()?));
        } else if lookahead.peek(Token![-]) {
            return Ok(Self::Subtract(input.parse()?));
        } else if lookahead.peek(Token![*]) {
            return Ok(Self::Multiply(input.parse()?));
        } else if lookahead.peek(Token![/]) {
            return Ok(Self::Divide(input.parse()?));
        } else if lookahead.peek(Token![%]) {
            return Ok(Self::Modulo(input.parse()?));
        }

        if lookahead.peek(Token![=]) {
            return Ok(Self::Eq(input.parse()?));
        } else if lookahead.peek(Token![<]) {
            if input.peek2(Token![>]) {
                return Ok(Self::Uneq(input.parse()?, input.parse()?));
            } else if input.peek2(Token![=]) {
                return Ok(Self::LessThanOrEq(input.parse()?, input.parse()?));
            } else {
                return Ok(Self::LessThan(input.parse()?));
            }
        } else if lookahead.peek(Token![>]) {
            if input.peek2(Token![=]) {
                return Ok(Self::GreaterThanOrEq(input.parse()?, input.parse()?));
            } else {
                return Ok(Self::GreaterThan(input.parse()?));
            }
        }

        Err(lookahead.error())
    }
}

impl ToTokens for BinOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        macro_rules! branches {
            ($($variant:ident)*) => {
                match self {
                    $(Self::$variant(..) => quote! { ::kosame::expr::BinOp::$variant }.to_tokens(tokens)),*
                }
            };
        }

        branches!(
            Multiply
            Divide
            Modulo
            Add
            Subtract
            Eq
            Uneq
            LessThan
            GreaterThan
            LessThanOrEq
            GreaterThanOrEq
        );
    }
}
