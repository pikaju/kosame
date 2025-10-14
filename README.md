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

To execute the query, the Kosame macro automatically generates the following row structs (simplified):
```rust
struct Row {
    id: i32,
    title: String,
    content: Option<String>,
    comments: Vec<RowComments>,
}

struct RowComments {
    id: i32,
    content: String,
    upvotes: i32,
}
```

If the `serde` feature is enabled, the row structs implement `serde::Serialize`, making it trivial to return from an API endpoint. 
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

Before you can write queries with Kosame, you must declare your database schema. Instead of inventing a new syntax, Kosame tries to follow the existing `CREATE TABLE` syntax closely.
```rust
kosame::table! {
    create table posts (
        id int primary key default uuidv7(),
        title text not null,
        content text, // trailing comma is allowed
    );
}
```
This means declaring your schema may be as simple as copying a `pg_dump` into the Kosame macro. However, to force consistency, all SQL keywords must be lowercase.
Kosame has a basic SQL expression parser, which allows you to define the `default` expression of a column.

### Column aliases and type overrides

If you are not happy with the name your column has in your database schema or want to refer to it by a different name in Rust, you can use a column alias:
```rust
kosame::table! {
    create table my_table (
        MyColumn text not null,
    );

    MyColumn as my_column,
}
```

Kosame attempts to guess the Rust type of a column based on its database type. For example, a PostgreSQL column of type `text` will be represented by a Rust `String`. If you want to use a different type, or if the database type is unknown to Kosame (e.g. for PostgreSQL custom types), you can specify a type override:

```rust
use smol_str;

kosame::table! {
    create table my_table (
        my_column text not null,
    );

    my_column: smol_str::SmolStr,
}
```
Note that the specified type must be declared or `use`d in the scope surrounding the `kosame::table!` call.

Aliases and type overrides can be combined as follows:

```rust
use smol_str::SmolStr;

kosame::table! {
    create table my_table (
        MyColumn text not null,
    );

    MyColumn as my_column: SmolStr,
}
```

### Relations

In addition to column aliases and type overrides, you can also declare relation fields. Relations tell Kosame how different tables can be queries together.
```rust
kosame::table! {
    create table posts_table (
        id int primary key default uuidv7(),
        content text not null,
    );

    comments: (id) <= my_module::comments_table (post_id),
}

mod my_module {
    kosame::table! {
        create table comments_table (
            id int primary key default uuidv7(),
            post_id int not null,
            content text not null,
        );

        post: (post_id) => super::posts_table (id),
    }
}
```

In this example, we have a table `posts_table` and a table `comments_table`. For each row in `posts_table`, we expect there to be any number of comments. Conversely, each row in the `comments_table` has exactly one post associated with it, as defined by the `post_id` column.

The relation field declaration
```
comments: (id) <= my_module::comments_table (post_id)
```
describes a relation called `comments`. It states that there is another table named `comments_table` in the module `my_module` which has a column `post_id` that "points to" the `id` column of `posts_table`. Although a Kosame relation does not have to map to a database foreign key, you may think of the `<=` as pointing in the direction of the foreign key "pointer". With this relation field, we can query all comments associated with a given post:
```rust
kosame::query! {
    posts_table {
        id,
        content,
        comments {
            id,
            content,
        }
    }
}
```

In the comments table, we have the inverse relation:
```
post: (post_id) => super::posts_table (id),
```
It states that `post` is a row in the `super::posts_table`, and it is referred to by matching the `comment_table`'s `post_id` column with the `post_table`'s `id` column. Note that the arrow (`=>`) points in the other direction here. In this case, Kosame expects there to be at most one post per comment.
```rust
kosame::query! {
    my_module::comments_table {
        id,
        content,
        post {
            id,
            content,
        }
    }
}
```
