macro_rules! custom_keyword {
    ($kw:ident) => {
        pub mod $kw {
            pub mod $kw {}
        }
    };
}

macro_rules! keyword_group {
    ($group:ident { $($kw:ident),* }) => {
        pub mod $group {
            $(
                pub use super::$kw::*;
            )*
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

keyword_group!(column_constraint {
    not,
    default,
    primary,
    references
});

// Clause

custom_keyword!(select);

custom_keyword!(update);
custom_keyword!(set);

custom_keyword!(insert);
custom_keyword!(into);

custom_keyword!(delete);
custom_keyword!(using);

custom_keyword!(group);
custom_keyword!(by);

custom_keyword!(having);
custom_keyword!(values);

custom_keyword!(offset);

custom_keyword!(limit);

custom_keyword!(order);

custom_keyword!(asc);
custom_keyword!(desc);

custom_keyword!(nulls);
custom_keyword!(first);
custom_keyword!(last);

custom_keyword!(returning);

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

// Expr

custom_keyword!(is);
custom_keyword!(distinct);
custom_keyword!(and);
custom_keyword!(or);
custom_keyword!(cast);
