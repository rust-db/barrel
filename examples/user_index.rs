
use barrel::{Migration, types};

fn main() {
    let mut m = Migration::new();
    m.create_table("users", |t| {
        t.add_column("first_name", types::varchar(64).nullable(false));
        t.add_column("last_name", types::varchar(64).nullable(false));
        t.add_column("birthday", types::date().nullable(false));

        t.add_index(
            "names",
            types::index(vec!["first_name", "last_name"])
                .unique(true)
                .nullable(false),
        );
    });
}
