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
    // let mut client = postgres::Client::connect(
    //     "postgres://postgres:postgres@localhost:5432/postgres",
    //     postgres::NoTls,
    // )
    // .unwrap();
    //
    // println!("==== Query ====");
    // println!("{:?}", &my_query::Query {}.sql_string());
    // println!("========");
    // let result = client.query(&my_query::Query {}.sql_string(), &[]).unwrap();
    // for row in result {
    //     println!("{:?}", my_query::Row::from(row));
    // }
    // println!("==== End ====");

    // kosame::query! {
    //     schema::posts {
    //         id,
    //         title,
    //         content,
    //         // comments {
    //         //     id,
    //         // }
    //         // where id = 5
    //         // order by name
    //     } as my_query
    // };

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
