extern crate barrel;
use barrel::*;

fn main() {
    let mut s = Schema::name("public").create_table("users", |t: &mut Table| {
        t.increments();
    });

    println!("{}", s.exec());
}