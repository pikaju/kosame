use kosame::query::{Query, RecordArrayRunner};

// Declare your database schema.
mod schema {
    kosame::table! {
        // Kosame uses the familiar SQL syntax to define tables.
        create table posts (
            id int primary key default uuidv7(),
            title text not null,
            content text,
        );

        // Define a relation to another table. This enables relational queries.
        comments: (id) <= comments (post_id),
    }

    kosame::table! {
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

    let rows = kosame::query! {
        schema::posts {
            *, // Select all columns from the posts table.
        }
    }
    .exec_sync(&mut client, &mut RecordArrayRunner {})
    .unwrap();
    println!("{:#?}", rows);
}
