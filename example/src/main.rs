mod schema {
    kosame::table! {
        create table posts (
            id int,
            title text,
            content text,
        );
    }

    kosame::table! {
        create table comments (
            id int,
            post_id int,
            content text,
        );

        posts: (post_id) => posts (id),
    }
}

fn main() {
    let (result, query) = kosame::query! {
        schema::comments {
            id,
            content,
            posts {
                id,
                title,
                content
            },
            //
            // where id = 5
            // order by name
        }
    };
    println!("{}", query);
    println!("{:?}", result);
}
