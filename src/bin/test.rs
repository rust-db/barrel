extern crate barrel;
use barrel::*;

fn main() {
    
    let mut s = Schema::new().with_schema("public").create_table("users", |t: &mut Table| {
        t.increments();
    });

    println!("{}", s.exec());

    // Schema::create_table("users", |t| {

    // });

    // Schema::with_schema("public").create_table
}
