use kosame::query::{Query, RecordArrayRunner};
use kosame::statement::Statement;

mod kek {
    pub mod pip {
        pub mod lel {
            pub const KEK: i32 = 5;
        }
    }

    pub mod smep {
        pub mod lel {}
    }

    pub use pip::*;
    pub use smep::lel;
}

use kek::lel;

// Declare your database schema.
mod schema {
    kosame::pg_table! {
        // Kosame uses the familiar SQL syntax to define tables.
        create table posts (
            id int primary key default uuidv7(),
            #[kosame(rename = posts, ty = ::std::string::String)]
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

    // let rows = kosame::pg_query! {
    //     schema::posts {
    //         *,
    //         comments {
    //             *
    //         }
    //     }
    // }
    // .exec_sync(&mut client, &mut RecordArrayRunner {})
    // .unwrap();

    let statement = kosame::pg_statement! {
        select
            posts.posts as smep: ::std::string::String,
        from schema::posts
        inner join schema::comments as kek on posts.id = kek.post_id
        where posts.id > 4
    };
    use kosame::sql::FmtSql;
    let sql = statement
        .repr()
        .to_sql_string::<kosame::sql::postgres::Dialect>()
        .unwrap();
    println!("{}", sql);

    let rows = statement.exec_sync(&mut client).unwrap();

    println!("{:#?}", rows);
}
