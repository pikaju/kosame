use kosame::{dialect::Postgres, sql_writer::SqlFormatter};

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

fn main() {
    let mut client = postgres::Client::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        postgres::NoTls,
    )
    .unwrap();

    let kek = 5i32;
    let id: i32 = 6;
    let limit = 3i64;

    let query = kosame::query! {
        #[serde(rename_all = "camelCase")]
        schema::posts {
            /// all the post fields
            * as all_of_them,
            "kek" as pip: ::std::string::String,
            comments {
                id,
                post_id as postid: I32,
                content: ::std::string::String,
                post { * } as cool_post,
                offset 1
            },
            where id = :id
            order by id + :kek desc nulls last, id + 6
            limit 5
        }
    };

    use kosame::query::Query;

    println!("==== Query ====");
    let mut sql = String::new();
    let mut formatter = SqlFormatter::new(&mut sql);
    query
        .root()
        .fmt_sql::<Postgres>(&mut formatter, None)
        .unwrap();
    println!("{:?}", sql);
    println!("========");

    // let params = my_query::Params {
    //     id: &id,
    //     pip: &0i32,
    // };

    let result = client.query(&sql, &query.params().array()).unwrap();
    for row in result {
        let row = query.from_row(&row);
        println!("{:?}", &row);
        println!("---");
        println!("{}", serde_json::to_string_pretty(&row).unwrap());
        println!("========");
    }
}
