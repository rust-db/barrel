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
//! // create table "users" ("id" serial primary key, "username" varchar(255), "plushy_sharks_owned" int)
//! ```
//! 
//! The above code, for example, will create a new table in the "public" schema, called "users"
//! and then execute the table hook on it when invoking `schema.exec()`. The hook creates an
//! auto-incrementing primary intex. By default the name "id" is assumed.
//! 
//! Barrel is designed to give you ease of use as well as power over how you write your 
//! migrations and SQL schemas.
//! 
//! Pull-Requests with more/ better documentation welcome 💚


pub mod table;
pub mod schema;
pub mod generators;

/* Conveniently expose core structures */
pub use table::Table;
pub use schema::Schema;

/* Test module */
mod test;
