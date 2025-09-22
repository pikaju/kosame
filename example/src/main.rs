use postgres::GenericClient;

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
    let mut client = postgres::Client::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        postgres::NoTls,
    )
    .unwrap();

    println!("==== Query ====");
    let result = client.query("select id from posts", &[]).unwrap();
    for row in result {
        println!("{:?}", row);
    }
    println!("==== End ====");

    let (result, query) = kosame::query! {
        schema::comments {
            id,
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
        }
    };

    println!("{}", query);
    println!("{:?}", result);

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
