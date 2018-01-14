extern crate barrel;

use barrel::schema::Schema;
use barrel::table::Table;

fn main() {
    let mut s = Schema::name("public").create_table("users", |t: &mut Table| {
        t.increments();
    });

    println!("{}", s.exec());
}