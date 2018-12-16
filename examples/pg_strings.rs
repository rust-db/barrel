extern crate barrel;

use barrel::backend::Pg;
use barrel::types;
use barrel::*;

fn main() {
    let mut m = Migration::new();

    // A new table is automatically created with an "id" primary key
    // To disable that call `without_id` on the return of `create_table`
    m.create_table("users", |t: &mut Table| {
        t.add_column("name", types::varchar(255)); // Default name is "Anonymous"
        t.add_column("description", types::text().nullable(true)); // Can be null
        t.add_column("age", types::integer());
        t.add_column("posts", types::foreign("posts"));
        t.add_column("owns_plushy_sharks", types::boolean());
    });

    println!("{}", m.make::<Pg>());
}
