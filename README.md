<div align="center">
  <img width="256" src="./misc/readme/logo-white.svg#gh-dark-mode-only" />
  <img width="256" src="./misc/readme/logo-black.svg#gh-light-mode-only" />
</div>

<div align="center">
  <h3>Macro-based Rust ORM focused on developer ergonomics</h3> 
</div>

<br />

Kosame (小雨, Japanese for "light rain" or "drizzle") is a Rust ORM inspired by [Prisma](https://github.com/prisma/prisma) and [Drizzle](https://github.com/drizzle-team/drizzle-orm).
Most Rust ORMs ask the developer to write both the query they want to perform as well as the resulting struct type to store the query rows in, even though they are tightly coupled. Some TypeScript ORMs manage to solve this by inferring the row types from the query itself. They also offer relational queries, allowing developers to go from flat tables to a nested struct hirarchy. Kosame was born out of a desire to have this level of developer ergonomics in Rust, using macro magic.

Kosame requires no active database connection during development and has no build step. Despite this, Kosame offers strong typing and rust-analyzer auto-completions.

**Kosame is currently a prototype and not recommended for production use.**

## Showcase

```rust
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

            // Include all related comments using the relation defined above.
            comments {
                id,
                content,
                upvotes,

                // Familiar syntax for "where", "order by", "limit", and "offset".
                order by upvotes desc
                limit 3
            },

            // The function parameter `id: i32` is used as a query parameter here.
            where id = :id
        }
    }
    .execute(
        client,
        // RecordArrayRunner describes the strategy to fetch rows from the database. In this case,
        // we run just a single SQL query that makes use of PostgreSQL's arrays and anonymous records.
        &mut RecordArrayRunner {},
    )
    .await?
    .into_iter()
    .next();

    Ok(row)
}
```

The result type implements `serde::Serialize`, making it trivial to return from an API endpoint. 
Using `serde_json`, we can print the result of the `fetch_post` function for post ID `5`:
```json
{
  "id": 5,
  "title": "my post",
  "content": "hi this is a post",
  "comments": [
    {
      "id": 18,
      "content": "im commenting something",
      "upvotes": 4
    },
    {
      "id": 19,
      "content": "im another comment",
      "upvotes": 0
    }
  ]
}
```

## Planned features

Kosame is an early prototype. There are many features and performance optimizations left to implement, including but not limited to:
* Support for other database management systems. Currently, only PostgreSQL (using [`tokio_postgres`](https://docs.rs/tokio-postgres/latest/tokio_postgres/)) is supported.
* Automatically generated database migrations based on changes in the Kosame schema.
* Database mutations, i.e. `insert`, `update`, and `delete`. As of right now, Kosame only supports read-queries.
* Support for more SQL expression syntax.
* Alternative query runners, similar to the [`relationLoadStrategy` that Prisma offers](https://www.prisma.io/blog/prisma-orm-now-lets-you-choose-the-best-join-strategy-preview).

## Defining the schema

