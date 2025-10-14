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

kosame::query! {
    schema::posts {
        content,
        where id = :id
    }
    as my_query
}

async fn fetch_row(
    client: &mut tokio_postgres::Client,
    id: i32,
) -> Result<Vec<my_query::Row>, Box<dyn Error>> {
    let rows = my_query::Query::new(my_query::Params { id: &id })
        .execute(client, &mut RecordArrayRunner {})
        .await?;
    Ok(rows)
}

async fn fetch_post(
    client: &mut tokio_postgres::Client,
    id: i32,
) -> Result<Option<impl serde::Serialize + Debug>, Box<dyn Error>> {
    let row = kosame::query! {
        #[serde(rename_all = "camelCase")]
        schema::posts {
            id as my_id,

            /// Rust documentation comments, like this one, are also attributes. This means you can easily document your query and query fields like this!
            content,

            comments {
            id as my_id,
                #[serde(rename = "cool_content")]
                content as comment_content,
            }
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
    println!("{}", serde_json::to_string_pretty(&post).unwrap());
    println!("{:#?}", post);

    // let smep = {
    //     mod internal {
    //         #[derive(Debug)]
    //         pub struct Kek {
    //             pub x: i32,
    //         }
    //     }
    //
    //     pub struct Smep {}
    //
    //     pub trait SmepTrait {
    //         fn make_kek(&self) -> internal::Kek;
    //     }
    //
    //     impl SmepTrait for Smep {
    //         fn make_kek(&self) -> internal::Kek {
    //             internal::Kek { x: 5 }
    //         }
    //     }
    //
    //     Smep { lel: 5 }
    // };
    // let lel = smep.make_kek();
    // println!("{:?}", lel);

    // let kek = 5;
    // let id: i32 = 5;
    // let limit: i64 = 3;
    //
    // let rows = kosame::query! {
    //     #[serde(rename_all = "camelCase")]
    //     schema::posts {
    //         /// all the post fields
    //         title,
    //         cast(:id as int) as id: I32,
    //         cast(now() as text) as pip: ::std::string::String,
    //         comments {
    //             id,
    //             post_id as postid: I32,
    //             content: ::std::string::String,
    //             post { * } as cool_post,
    //             offset 1
    //         },
    //         where content is null
    //         order by :kek + 5 desc nulls last, id + 6
    //         limit :limit
    //     }
    // }
    // .execute(&mut client, &mut RecordArrayRunner {})
    // .await
    // .unwrap();
    //
    // println!("{:#?}", rows);
    //
    // kosame::query! {
    //     #[serde(rename_all = "camelCase")]
    //     schema::posts {
    //         *,
    //         comments { sum(cast(1 as int)) as mysum: ::std::option::Option<i64> },
    //         limit :limit
    //     } as my_query
    // }
    //
    // let rows = my_query::Query::new(my_query::Params { limit: &1i64 })
    //     .execute(&mut client, &mut RecordArrayRunner {})
    //     .await
    //     .unwrap();
    // println!("{}", serde_json::to_string_pretty(&rows).unwrap());
}
