use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

pub struct Relation {
    name: Ident,
    source_columns: Vec<Ident>,
    target_table: Path,
    target_columns: Vec<Ident>,
    relation_type: RelationType,
}

impl Relation {
    pub fn name(&self) -> &Ident {
        &self.name
    }
}

#[cfg(feature = "dsl")]
impl From<crate::dsl::schema::Relation> for Relation {
    fn from(value: crate::dsl::schema::Relation) -> Self {
        use crate::dsl::{path_ext::PathExt, schema::Arrow};

        Self {
            name: value.name,
            source_columns: value.source_columns.into_iter().collect(),
            target_table: value.target_table.to_call_site(3),
            target_columns: value.target_columns.into_iter().collect(),
            relation_type: match value.arrow {
                Arrow::ManyToOne(..) => RelationType::ManyToOne,
                Arrow::OneToMany(..) => RelationType::OneToMany,
            },
        }
    }
}

impl ToTokens for Relation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let name_string = name.to_string();

        let target_table = &self.target_table;

        let source_columns = &self.source_columns;
        let target_columns = &self.target_columns;

        let relation_type = match self.relation_type {
            RelationType::ManyToOne => quote! { ::kosame::relation::ManyToOne<T> },
            RelationType::OneToMany => quote! { ::kosame::relation::OneToMany<T> },
        };

        quote! {
            pub mod #name {
                pub use #target_table as target_table;

                pub mod source_columns {
                    #(pub use super::super::super::columns::#source_columns;)*
                }

                pub mod target_columns {
                    #(pub use super::target_table::columns::#target_columns;)*
                }

                pub const RELATION: ::kosame::schema::Relation = ::kosame::schema::Relation::new(
                    #name_string,
                    super::super::NAME,
                    &[#(&source_columns::#source_columns::COLUMN),*],
                    target_table::NAME,
                    &[#(&target_columns::#target_columns::COLUMN),*],
                );

                pub type Type<T> = #relation_type;
            }
        }
        .to_tokens(tokens);
    }
}

#[allow(unused)]
enum RelationType {
    ManyToOne,
    OneToMany,
}
