mod schema {
    kosame::table! {
        create table posts (
            id int,
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
        schema::posts {
            id,
            content,
            posts,
            //
            // where id = 5
            // order by name
        }
    };
    println!("{}", query);
}
