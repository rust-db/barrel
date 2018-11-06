//! Powerful schema migration builder API in Rust.
//!
//! Barrel is meant to make writing migrations for different databases as easy
//! as possible. It has three primary models:
//! the [Migration](migration/struct.Migration.html) which represents
//! all changes and changes made on a database level,
//! the [Table](table/struct.Table.html) and the
//! [Column](column/struct.Column.html).
//!
//! When creating or altering tables a lambda which exposes `&mut Table` is
//! provided for initialisation. Adding columns is then as easy as calling
//! `add_column(...)` on the table.
//!
//! Each column is statically typed and some types require some metadata in order
//! to compile the migration (for example `Varchar(255)`). You can also provide
//! default types and override encodings, nullability or uniqueness of columns.
//! Some checks are performed at compile-time however most things (including)
//! correct default values) are only checked at runtime.
//!
//! **Note** Since version `0.3.0` it is required to provide a database backend
//! in order to compile `barrel`.
//!
//! The following code is a simple example of how to get going with `barrel`
//!
//! ```
//! extern crate barrel;
//!
//! use barrel::{types, Migration};
//!
//! fn main() {
//!     let mut m = Migration::new();
//!     m.create_table("users", |t| {
//!         t.add_column("name", types::varchar(255));
//!         t.add_column("age", types::integer());
//!         t.add_column("owns_plushy_sharks", types::boolean());
//!     });
//! }
//! ```
//! `barrel` also supports more advanced types, such as `Foreign(...)`
//! and `Array(...)` however currently doesn't support nested Array types on
//! foreign keys (such as `Array(Array(Foreign(...)))`). Each column addition
//! returns a Column object which can then be used to provide further
//! configuration.
//!
//! To generate SQL strings you have two options. If you just want to run the
//! migration yourself simply run `Migration::exec()` where you provide a
//! generic `SqlGenerator` type according to your database backend
//!
//! ```norun
//! // Example for pgsql
//! m.make::<Pg>();
//! ```
//!
//! Alternatively, if you're a library developer and you want to more easily
//! embed `barrel` into your library you can simply implement the
//! `DatabaseExecutor` trait for a type of yours that knows how to execute SQL.
//! Running a migration with `barrel` is then super easy.
//!
//! ```norun
//! m.execute(executor);
//! ```
//!
//! In this case `executor` is your provided type which implements the required
//! trait. You can read more about this in the `connectors` module docs.
//!
//! **Important**: This crate is still early in development and the API might
//! change rapidely between pre-release versions. I will try as best I can to
//! include changes in the `CHANGELOG` but can not guarantee perfect coverage.
//!
//! Also, if there is missing or invalid documentation for this crate, PR's are
//! always welcome 💚

// #[cfg(not(any(feature = "sqlite", feature = "pg")))]
// compile_error!("`barrel` cannot be built without a database backend speccified via cargo `--features`");

// TODO: Make this "diesel" block prettier
#[cfg(feature = "diesel-filled")]
extern crate tempdir;
#[cfg(feature = "diesel-filled")]
pub mod integrations;
#[cfg(feature = "diesel-filled")]
pub use integrations::*;
#[cfg(feature = "diesel-filled")]
extern crate diesel;

pub mod backend;
pub mod connectors;

pub mod table;
pub use table::{Column, Table, TableMeta};

pub mod migration;
pub use migration::Migration;

pub mod types;

#[cfg(test)]
mod tests;

use std::rc::Rc;

/// An enum set that represents a single change on a table
#[derive(Clone)]
pub enum TableChange {
    /// Add a column of a name and type
    AddColumn(String, types::Type),

    /// Change an existing column
    ChangeColumn(String, types::Type, Rc<Fn(&mut types::Type)>),

    /// Simply rename a column
    RenameColumn(String, String),

    /// Remove a column
    DropColumn(String),
}

/// An enum set that represents a single change on a database
#[derive(Clone)]
pub enum DatabaseChange {
    /// Create a new table
    CreateTable(Table, Rc<Fn(&mut Table)>),

    /// Create a new table *only* if it doesn't exist yet
    CreateTableIfNotExists(Table, Rc<Fn(&mut Table)>),

    /// Change fields on an existing table
    ChangeTable(Table, Rc<Fn(&mut Table)>),

    /// Rename a table
    RenameTable(String, String),

    /// Drop an existing table
    DropTable(String),

    /// Only drop a table if it exists
    DropTableIfExists(String),
}

