use barrel::backend::Sqlite;
use barrel::{types, Migration, Table};

fn main() {
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {
        t.add_column("id", types::text().primary(true));
        t.add_column("name", types::varchar(255).default("Anonymous")); // Default name is "Anonymous"
        t.add_column("description", types::text().nullable(true)); // Can be null
        t.add_column("age", types::integer());
        t.add_column(
            "posts",
            types::foreign(
                "posts",
                "id",
                types::ReferentialAction::Unset,
                types::ReferentialAction::Unset,
            ),
        );
        t.add_column("created_at", types::date());
        t.add_column("owns_plushy_sharks", types::boolean());
    });

    println!("{}", m.make::<Sqlite>());
}
