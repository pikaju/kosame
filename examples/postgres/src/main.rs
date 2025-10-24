use kosame::query::{Query, RecordArrayRunner};
use kosame::statement::Statement;

// Declare your database schema.
mod schema {
    kosame::pg_table! {
        // Kosame uses the familiar SQL syntax to define tables.
        create table posts (
            id int primary key default uuidv7(),
            #[kosame(rename = tiitle, ty = ::std::string::String)]
            title text not null,
            content text,
        );

        // Define a relation to another table. This enables relational queries.
        comments: (id) <= comments (post_id),
    }

    kosame::pg_table! {
        create table comments (
            id int primary key,
            post_id int not null,
            content text not null,
            upvotes int not null default 0,
        );

        // You may also define the inverse relation if you need it.
        post: (post_id) => posts (id),
    }
}

fn main() {
    let mut client = postgres::Client::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        postgres::NoTls,
    )
    .unwrap();

    let rows = kosame::pg_statement! {
        // select
        //     id as pip: i32,
        // from schema::posts
    }
    .exec_sync(&mut client)
    .unwrap();

    println!("{:#?}", rows[0].pip);
}
