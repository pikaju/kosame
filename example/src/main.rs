use kosame::query::RecordArrayRunner;

pub mod schema {
    kosame::table! {
        create table posts (
            id int primary key default uuidv7(),
            title text not null,
            content text,
        );

        comments: (id) <= comments (post_id),
    }

    kosame::table! {
        create table comments (
            id int,
            post_id int,
            content text,
        );

        post: (post_id) => crate::schema::posts (id),
    }
}

type I32 = i32;
type Bool = bool;

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

    let kek = 5;
    let id: i32 = 5;
    let limit: i64 = 3;

    let rows = kosame::query! {
        #[serde(rename_all = "camelCase")]
        schema::posts {
            /// all the post fields
            * as all_of_them,
            cast(:id as int) as id: I32,
            cast(now() as text) as pip: ::std::string::String,
            comments {
                id,
                post_id as postid: I32,
                content: ::std::string::String,
                post { * } as cool_post,
                offset 1
            },
            where content is null
            order by :kek + 5 desc nulls last, id + 6
            limit :limit
        }
    }
    .execute(&mut client, &mut RecordArrayRunner {})
    .await
    .unwrap();

    println!("{:#?}", rows);

    kosame::query! {
        #[serde(rename_all = "camelCase")]
        schema::posts {
            *,
            comments { sum(cast(1 as int)) as mysum: ::std::option::Option<i32> },
            limit :limit
        } as my_query
    }

    let rows = my_query::Query::new(my_query::Params { limit: &1i64 })
        .execute(&mut client, &mut RecordArrayRunner {})
        .await
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&rows).unwrap());
}
