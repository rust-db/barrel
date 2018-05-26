extern crate barrel;

use barrel::*;
use barrel::backend::Sqlite;

fn main() {
    use Type::*;
    let mut m = Migration::new();
    // A new table is automatically created with an "id" primary key
    // To disable that call `without_id` on the return of `create_table`
    m.create_table("users", |t: &mut Table| {
        
        t.add_column("name", Varchar(255)).default("Anonymous"); // Default name is "Anonymous"
        t.add_column("description", Text).nullable(); // Can be null
        t.add_column("age", Integer);
        t.add_column("posts", Foreign("posts"));
        t.add_column("owns_plushy_sharks", Boolean);
    });

    println!("{}", m.make::<Sqlite>());
}
