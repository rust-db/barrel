//! Powerful schema builder API in Rust, using Diesel in the backend.
//!
//! Barrel has two primary models, the `Migration` and the `Table`. A schema migration
//! is built with a variety of hooks that can be executed on tables, using static callbacks.
//!
//! ```
//! extern crate barrel;
//! 
//! use barrel::*;
//! use barrel::backend::Pg;
//!
//! fn main() {
//!     let mut m = Migration::new();
//!     m.create_table("users", |t| {
//!         t.add_column("name", Type::Text);
//!         t.add_column("age", Type::Integer);
//!         t.add_column("owns_plushy_sharks", Type::Boolean);
//!     });
//!
//!     println!("{}", m.make::<Pg>());
//! }
//! ```
//!
//! The above code, for example, will create a new table called "users". All tables implicitly
//! add an auto incrementing primary key called "id". This behaviour can't currently be turned
//! off. The callback is executed when calling `Migration::exec()`
//!
//! Barrel is designed to give you ease of use as well as power over how you write your
//! migrations and SQL schemas.


pub mod backend;
pub mod connectors;

pub mod table;
pub use table::{Table, TableMeta, Column};

pub mod migration;
pub use migration::Migration;

/// An enum set that represents a single change on a table
pub enum TableChange {

    /// Add a column of a name and type
    AddColumn(String, Column),

    /// Change an existing column
    ChangeColumn(String, Column, Box<Fn(&mut Column)>),

    /// Simply rename a column
    RenameColumn(String, String),

    /// Remove a column
    RemoveColumn(String),
}

/// An enum set that represents a single change on a database
pub enum DatabaseChange {

    /// Create a new table
    CreateTable(Table, Box<Fn(&mut Table)>),

    /// Create a new table *only* if it doesn't exist yet
    CreateTableIfNotExists(Table, Box<Fn(&mut Table)>),

    /// Change fields on an existing table
    ChangeTable(Table, Box<Fn(&mut Table)>),

    /// Rename a table
    RenameTable(String, String),

    /// Drop an existing table
    DropTable(String),

    /// Only drop a table if it exists
    DropTableIfExists(String),
}


/// Type enum to specificy the `type` of an SQL column. NOTE: Not all types are
/// supported by all database backends!
/// 
/// When creating a column users need to specify the type and potential
/// metadata this type requires to the `add_column` function.
/// 
/// ```
/// t.add_column("posts", Type::Array(box Type::Foreign("posts")));
/// ```
#[derive(PartialEq, Debug)]
pub enum Type {
    
    /// Create a simple "text" field
    Text,

    /// Create a simple "binary" field
    Binary,

    /// Provide a size limit for this field 
    Varchar(usize),

    /// Creates a 64-bit integer
    Integer,

    /// Creates a 32-bit float
    Float,

    /// Creates a 64-bit float
    Double,

    /// Boring ol' boolean
    Boolean,

    /// Provide the name of a table to point to
    Foreign(&'static str),

    /// Any type can also exist as an array type
    Array(Box<Type>),
}
