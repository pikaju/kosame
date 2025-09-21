kosame_macro::table! {
    create table pip (
        id int,
        name text,
    );
}

fn main() {
    kosame_macro::query! {
        pip {

            where id = 5
            order by name
        }
    };
}
