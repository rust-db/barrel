extern crate barrel;
extern crate diesel;

// use barrel::{Schema, Table, Connector};
// use barrel::generators::postgres::*;
// use barrel::connectors::diesel::*;

// fn main() {

//     let mut sql = Schema::<Pg>::new();
//     sql.create_table("users", |t: &mut Table<Pg>| {
//         t.increments();
//         t.string("username");
//         t.integer("plushy_sharks_owned");

//         t.timestamp("joined");
//         t.timestamp("birthday");
//     });

//     let migration = sql.exec();
//     println!("{}", migration);

//     let mut connection = Connector::<DieselPGSQL>::new("postgres://rust:1234@localhost/barrel");
//     connection.batch_exec(&migration);
// }

/// A default type wrapper which allows generic use
enum DefType {
    Text(String),
//     Integer(i64),
//     // Float(f64),
//     // Boolean(bool),
//     // Date(String),
//     // Time(u64),
//     // Timestamp(u64),
}

// enum Type {
//     Text,
//     Integer,
//     // Float,
//     // Boolean,
//     // Date,
//     // Time,
//     // Timestamp,
// }

// struct Meh {
//     default: DefType,
// }

// fn some_default<T>(t: Type, def: T) -> Meh {
//     return match t {
//         Type::Text => Meh { default: DefType::Text(def) }
//     }
// }

// struct Meh<T> {
//     default: DefType<T>,
// }

// impl<T> Meh<T> {
//     pub fn new(data: T) -> Meh<T> {
//         return Meh { default: DefType(data) };
//     }

//     pub fn set(&mut self, data: T) {
//         self.default = Some(data);
//     }
// }


// use barrel::schema::Type;
fn main() {

    let mut meh = Meh::new();
    meh.set("Some data");

    // let meh = make_meh("default");
/*
    |t| {
        t.add_column("name", Type::Text);
    });
*/

}