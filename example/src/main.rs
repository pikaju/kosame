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
    let (result, query) = kosame::query! {
        schema::comments {
            id,
            post {
                id,
                title,
                content,
            },
            content,
            post_id,
            //
            // where id = 5
            // order by name
        }
    };
    println!("{}", query);
    println!("{:?}", result);
    println!("{:?}", result.post.id);

    let (result, query) = kosame::query! {
        schema::posts {
            id,
            title,
            content,
            comments {
                id,
            },
            //
            // where id = 5
            // order by name
        }
    };
    println!("{}", query);
    println!("{:?}", result);
    println!("{:?}", result.comments.id);
}
