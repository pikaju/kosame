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
    println!("{:?}", query.to_sql_string());
    println!("========");
    let result = client.query(&query.to_sql_string(), &[]).unwrap();
    for row in result {
        let row = my_query::Row::from(row);
        println!("{:?}", row.comments[0].post[0].title);
        println!("{:?}", row);
    }
    println!("==== End ====");

    // impl<'a> ::kosame::pg::FromSql<'a> for my_query::RowComments {
    //     fn accepts(ty: &::kosame::pg::Type) -> bool {
    //         ty.name() == "record"
    //     }
    //
    //     fn from_sql(
    //         ty: &::kosame::pg::Type,
    //         raw: &[u8],
    //     ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
    //         Ok(Self {
    //             ..Default::default()
    //         })
    //     }
    // }

    kosame::query! {
        schema::posts {
            id,
            content,
            comments {
                id,
                content,
                post {
                    id,
                    title,
                }
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
