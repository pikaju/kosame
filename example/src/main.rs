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

fn main() {
    let mut client = postgres::Client::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        postgres::NoTls,
    )
    .unwrap();

    println!("==== Query ====");
    let query = my_query::Query {};
    println!("{:?}", query.to_sql_string());
    println!("========");
    let result = client.query(&query.to_sql_string(), &[]).unwrap();
    for row in result {
        let row = my_query::Row::from(row);
        println!("{:?}", &row);
        println!("---");
        println!("{}", serde_json::to_string_pretty(&row).unwrap());
        println!("========");
    }

    kosame::query! {
        schema::posts {
            /// all the post fields
            // * as all_of_them,
            comments {
                post_id as postid,
                // content as posti type ::std::string::String,
                // post {
                //     *,
                // } as cool_post,
                limit 1
            },
            limit 1
            // where id = 5
            // order by name
        } as my_query
    };
}
