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

    let id: i32 = 5;

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
            order by id + 5 desc nulls last, id + 6
            limit 3
        }
    };

    use kosame::query::Query;

    println!("==== Query ====");
    println!("{:?}", query.root().to_sql_string(None));
    println!("========");

    // let params = my_query::Params {
    //     id: &id,
    //     pip: &0i32,
    // };

    let params = query.params().array();

    let result = client
        .query(&query.root().to_sql_string(None), &params)
        .unwrap();
    for row in result {
        let row = query.from_row(&row);
        println!("{:?}", &row);
        println!("---");
        println!("{}", serde_json::to_string_pretty(&row).unwrap());
        println!("========");
    }
}
