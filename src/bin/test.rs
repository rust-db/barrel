extern crate barrel;
use barrel::*;

fn main() {
    let mut m = Migration::new();
    m.create_table("users", |t| {
        t.add_column("name", Type::Text).default("Anonymous");
        t.add_column("age", Type::Integer);
        t.add_column("owns_plushy_sharks", Type::Boolean);
    });

    // I like plushy sharks
    m.rename_table("sharks", "plushies");
}
