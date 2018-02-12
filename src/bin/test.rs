extern crate barrel;
use barrel::*;
use barrel::backend::Pg;

fn main() {
    let mut m = Migration::new();

    m.create_table("users", |t: &mut Table| {
        t.add_column("name", Type::Text).default("Anonymous");
        t.add_column("age", Type::Integer);
        t.add_column("owns_plushy_sharks", Type::Boolean);
    });

    println!("{}", m.make::<Pg>());
}
