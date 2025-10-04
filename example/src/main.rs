use kosame::query::RecordArrayRunner;
use tokio_postgres::Client;

pub mod schema {
    kosame::table! {
        create table posts (
            id int primary key,
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
            "k'ek'" as pip: ::std::string::String,
            comments {
                id,
                post_id as postid: I32,
                content: ::std::string::String,
                post { * } as cool_post,
                offset 1
            },
            where id = :id
            // order by :kek + 5 desc nulls last, id + 6
            limit 3
        }
    }
    .execute(&mut client, &mut RecordArrayRunner {})
    .await
    .unwrap();

    use kosame::query::Query;

    println!("{:#?}", rows);
}
