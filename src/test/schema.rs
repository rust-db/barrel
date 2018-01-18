//!

use schema::Schema;

fn test_stuff() {

    let mut schema = Schema::new();
    schema.create_table("users", |_| {});
    schema.create_table_if_not_exists("users", |_| {});
    schema.rename_table("users", "zombies");
    schema.drop_table("users");
    schema.drop_table_if_exists("users");
    schema.table("users", |_| {});

    println!("{}", schema.exec());
}