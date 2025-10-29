macro_rules! custom_keyword {
    ($kw:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $kw {
            #[allow(unused)]
            pub span: ::proc_macro2::Span,
        }

        impl $kw {
            fn peek(input: ::syn::parse::ParseStream) -> bool {
                ::syn::custom_keyword!($kw);
                input.peek($kw)
            }

            pub fn span(&self) -> &::proc_macro2::Span {
                &self.span
            }
        }

        impl ::syn::parse::Parse for $kw {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                ::syn::custom_keyword!($kw);
                let result = input.parse::<$kw>();
                match result {
                    Ok(result) => Ok(Self { span: result.span }),
                    Err(error) => {
                        let Ok(ident) = input.parse::<::syn::Ident>() else {
                            return Err(error);
                        };

                        ::proc_macro_error::dummy::set_dummy(::quote::quote! {
                            mod __kosame_autocomplete {
                                mod kw {
                                    pub mod $kw {}
                                }
                                use kw::#ident;
                            }
                        });
                        ::proc_macro_error::abort!(ident.span(), error.to_string());
                    }
                }
            }
        }
    };
}

pub(crate) use custom_keyword;
