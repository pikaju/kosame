macro_rules! custom_keyword {
    ($kw:ident) => {
        ::syn::custom_keyword!($kw);

        impl $kw {
            #[allow(unused)]
            pub fn parse_autocomplete(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                let result = input.parse::<Self>();
                match result {
                    Ok(result) => Ok(Self { span: result.span }),
                    Err(error) => {
                        let Ok(ident) = input.parse::<::syn::Ident>() else {
                            return Err(error);
                        };

                        ::proc_macro_error::dummy::set_dummy(::quote::quote! {
                            use ::kosame::keyword::$kw::#ident;
                        });
                        ::proc_macro_error::abort!(ident.span(), error.to_string());
                    }
                }
            }
        }
    };
}

// Table

custom_keyword!(create);
custom_keyword!(table);

// Column

custom_keyword!(not);
custom_keyword!(null);

custom_keyword!(default);

custom_keyword!(primary);
custom_keyword!(key);

custom_keyword!(references);

// Clause

custom_keyword!(select);
custom_keyword!(update);

// From

custom_keyword!(from);

custom_keyword!(join);
custom_keyword!(inner);
custom_keyword!(left);
custom_keyword!(right);
custom_keyword!(full);
custom_keyword!(on);

custom_keyword!(natural);
custom_keyword!(cross);

custom_keyword!(lateral);

// Attribute

custom_keyword!(kosame);

custom_keyword!(driver);
custom_keyword!(rename);
custom_keyword!(ty);

custom_keyword!(__pass);
custom_keyword!(__table);
