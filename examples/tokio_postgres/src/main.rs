use std::{error::Error, fmt::Debug};

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

async fn fetch_post(
    client: &mut tokio_postgres::Client,
    id: i32,
) -> Result<Option<impl serde::Serialize + Debug>, Box<dyn Error>> {
    let row = kosame::query! {
        schema::posts {
            *, // Select all columns from the posts table.

            comments {
                id,
                content,
                upvotes + 1 as upvotes: i32,

                // Familiar syntax for "where", "order by", "limit", and "offset".
                order by upvotes desc
                limit 3
            },

            // The function parameter `id: i32` is used as a query parameter here.
            where id = :id
        }
    }
    .exec_opt(
        client,
        // RecordArrayRunner describes the strategy to fetch rows from the database. In this case,
        // we run just a single SQL query that makes use of PostgreSQL's arrays and anonymous records.
        &mut RecordArrayRunner {},
    )
    .await?;

    Ok(row)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (mut client, connection) = tokio_postgres::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let post = fetch_post(&mut client, 5).await.unwrap();
    println!("{:#?}", post);
    println!("{}", serde_json::to_string_pretty(&post).unwrap());
}
