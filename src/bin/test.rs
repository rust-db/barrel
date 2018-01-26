extern crate barrel;

use barrel::{Schema, Table};
use barrel::generators::postgres::*;

fn main() {

    println!("Test!");

    let mut sql = Schema::<PGSQL>::new();
    sql.create_table("users", |t: &mut Table<PGSQL>| {
        t.increments();
        t.string("username");
        t.integer("plushy_sharks_owned");
    });


        t.timestamp("joined");
        t.timestamp("birthday");

    println!("{}", sql.exec());

}
