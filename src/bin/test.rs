extern crate barrel;
extern crate diesel;

use barrel::{Schema, Table, Connector};
use barrel::generators::postgres::*;
use barrel::connectors::diesel::*;

fn main() {

    let mut sql = Schema::<Pg>::new();
    sql.create_table("users", |t: &mut Table<Pg>| {
        t.increments();
        t.string("username");
        t.integer("plushy_sharks_owned");

        t.timestamp("joined");
        t.timestamp("birthday");
    });

    let migration = sql.exec();
    println!("{}", migration);

    let mut connection = Connector::<DieselPGSQL>::new("postgres://rust:1234@localhost/barrel");
    connection.batch_exec(&migration);
}
