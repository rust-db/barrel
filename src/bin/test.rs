extern crate barrel;

use barrel::{Schema, Table};
use barrel::generators::postgres::*;

fn main() {

    let mut sql = Schema::<Pg>::new();
    sql.create_table("users", |t: &mut Table<Pg>| {
        t.increments();
        t.string("username");
        t.integer("plushy_sharks_owned");

        t.timestamp("joined");
        t.timestamp("birthday");
    });

    println!("{}", sql.exec());
}
