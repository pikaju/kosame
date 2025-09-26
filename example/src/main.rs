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

    use kosame::query::Query;

    println!("==== Query ====");
    println!("{:?}", my_query::Query::ROOT.to_sql_string(None));
    println!("========");
    let result = client
        .query(&my_query::Query::ROOT.to_sql_string(None), &[])
        .unwrap();
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
            * as all_of_them,
            id = 5 as pip: Bool,
            comments {
                id,
                post_id as postid: I32,
                content: ::std::string::String,
                post {
                    *,
                } as cool_post,
                where id = 19
                limit 3 - 1
            },
            where id < 7
            order by id + 5 desc nulls last, id + 6
            limit 3
        } as my_query
    };
}
