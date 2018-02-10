//! Powerful schema builder API in Rust, using Diesel in the backend.
//! 
//! Barrel has two primary models, the schema and the table. A schema is built
//! with a variety of hooks that can be executed on tables, using static callbacks.
//! 
//! ```
//! use barrel::{Schema, Table};
//! use barrel::generators::postgres::*; // Pick the backend of your choice here
//!
//! let mut sql = Schema::<PGSQL>::new();
//! sql.create_table("users", |t: &mut Table<PGSQL>| {
//!     t.increments();
//!     t.string("username");
//!     t.integer("plushy_sharks_owned");
//! });
//! println!("{}", sql.exec());
//! ```
//! 
//! The above code, for example, will create a new table in the "public" schema, called "users"
//! and then execute the table hook on it when invoking `schema.exec()`. The hook creates an
//! auto-incrementing primary intex. By default the name "id" is assumed.
//! 
//! Barrel is designed to give you ease of use as well as power over how you write your 
//! migrations and SQL schemas.
//! 
//! ## Connect to Database
//! 
//! Barrel uses the Diesel connections and currently only supports postgresql databases. To
//! create a connection, use the `Connector` module
//! 
//! ```notest
//! let mut connection = Connector::<DieselPGSQL>::new("postgres://<username>:<password>@<server>/<database>");
//! connection.batch_exec(&migration);
//! ```
//! 
//! Pull-Requests with more/ better documentation welcome 💚

#[macro_use]
extern crate barrel_derive;
pub use barrel_derive::*;

// #[cfg(feature = "default")]
// extern crate diesel;
// pub mod connectors;

// pub mod table;
// pub mod schema;
// pub mod generators;

// /* Conveniently expose core structures */
// pub use table::Table;
// pub use schema::Schema;
// pub use connectors::Connector;

// /* Test module */
// mod test;


pub trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
pub struct FrenchToast;

#[derive(HelloWorld)]
pub struct Waffles;


pub fn test() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}


/// Represents the type of a column in SQL
pub enum Type {
    Text,
    Integer,

    // Float,
    // Boolean,
    // Date,
    // Time,
    // Timestamp,
}


// #[derive(Typed)]
pub enum TypeDefault {
    Text(Type, String),
    Integer(Type, i64),
    Float(Type, f64),
}

// // impl From<(Type, String)> for TypeDefault {
//     fn from(data: (Type, String)) -> Self {
//         return match data.0 {
//             Type::Text => TypeDefault::Text(data.0, data.1),
//             _ => panic!("Invalid data type"),
//         };
//     }
// }

// impl From<(Type, i64)> for TypeDefault {
//     fn from(data: (Type, i64)) -> Self {
//         return match data.0 {
//             Type::Integer => TypeDefault::Integer(data.0, data.1),
//             _ => panic!("Invalid data type"),
//         };
//     }
// }
