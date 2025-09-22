mod schema {
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

        post: (post_id) => posts (id),
    }
}

fn main() {
    println!("{}", KEK);
    let mut client = postgres::Client::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        postgres::NoTls,
    )
    .unwrap();

    println!("==== Query ====");
    println!("{:?}", &internal::Query {}.to_sql_string());
    println!("========");
    let result = client
        .query(&internal::Query {}.to_sql_string(), &[])
        .unwrap();
    for row in result {
        println!("{:?}", internal::Row::from(row));
    }
    println!("==== End ====");

    kosame::query! {
        schema::posts {
            id,
            title,
            content,
            // post {
            //     id,
            //     title,
            //     comments {
            //         id,
            //         post {
            //             comments {}
            //         }
            //     }
            // }
            //
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
