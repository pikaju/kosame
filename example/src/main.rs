pub mod schema {
    kosame::table! {
        create table posts (
            id int,
            title text,
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
    println!("{:?}", query.as_sql_str());
    println!("========");
    let result = client.query(query.as_sql_str(), &[]).unwrap();
    for row in result {
        println!("{:?}", my_query::Row::from(row));
    }
    println!("==== End ====");

    kosame::query! {
        schema::posts {
            id,
            title,
            content,
            comments {
                id,
                content,
                post {
                    id,
                },
            }
            // where id = 5
            // order by name
        } as my_query
    };

    // println!("{}", query);
    // println!("{:?}", result);

    // let (result, query) = kosame::query! {
    //     schema::posts {
    //         id,
    //         title,
    //         comments {
    //             id,
    //         },
    //         //
    //         // where id = 5
    //         // order by name
    //     }
    // };
    // println!("{}", query);
    // println!("{:?}", result);
}
