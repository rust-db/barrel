extern crate barrel;
use barrel::*;

fn main() {
    let mut m = Migration::new();

    m.create_table("users", |t: &mut Table| {
        t.add_column("name", Type::Text).default("Anonymous");
        t.add_column("age", Type::Integer);
        t.add_column("owns_plushy_sharks", Type::Boolean);
    });

    m.change_table("admins", |t: &mut Table| {
        t.drop_column("is_admin");
        t.rename_column("is_bob", "named_bob"); // This database sounds terrible
    });

    // I like plushy sharks
    m.rename_table("sharks", "plushies");

    // Actually nevermind
    m.drop_table("plushies");
}
