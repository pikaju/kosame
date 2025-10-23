mod field;
mod from;
mod group_by;
mod having;
mod limit;
mod offset;
mod order_by;
mod select;
mod r#where;

pub use field::*;
pub use from::*;
pub use group_by::*;
pub use having::*;
pub use limit::*;
pub use offset::*;
pub use order_by::*;
pub use select::*;
pub use r#where::*;

pub fn peek_clause(input: syn::parse::ParseStream) -> bool {
    From::peek(input)
        || Where::peek(input)
        || GroupBy::peek(input)
        || Having::peek(input)
        || OrderBy::peek(input)
        || Limit::peek(input)
        || Offset::peek(input)
}
