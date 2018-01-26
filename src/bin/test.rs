extern crate barrel;
extern crate diesel;

use barrel::{Schema, Table};
use barrel::generators::postgres::*;

fn main() {
    // let res: PgConnection = Connection::establish("postgres://rust:1234@localhost/barrel").unwrap();
    // res.batch_execute(&migration);


    /* ************************************************************************************ */

    let mut sql = Schema::<PGSQL>::new();
    sql.create_table("users", |t: &mut Table<PGSQL>| {
        t.increments();
        t.string("username");
        t.integer("plushy_sharks_owned");

        t.timestamp("joined");
        t.timestamp("birthday");
    });

    let migration = sql.exec();
    println!("{}", migration);

    // let connection = Connector::<PGSQL, Diesel>::new("postgres://rust:1234@localhost/barrel");
    // connection.batch_exec(&migration);

}
