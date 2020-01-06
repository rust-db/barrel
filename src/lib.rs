//! Powerful schema migration builder, that let's you write your SQL
//! migrations in Rust.
//!
//! `barrel` makes writing migrations for different databases as easy
//! as possible.  It provides you with a common API over SQL, with
//! certain features only provided for database specific
//! implementations.  This way you can focus on your Rust code, and
//! stop worrying about SQL.
//!
//! `barrel` has three primary models: the
//! [Migration](migration/struct.Migration.html) which represents all
//! changes and changes made on a database level, the
//! [Table](table/struct.Table.html) and the
//! [Type](types/struct.Type.html).
//!
//! When creating or altering tables a lambda which exposes `&mut
//! Table` is provided for initialisation. Adding columns is then as
//! easy as calling `add_column(...)` on the table.
//!
//! Each column is statically typed and some types require some
//! metadata in order to compile the migration (for example
//! `Varchar(255)`). You can also provide default types and override
//! encodings, nullability or uniqueness of columns.  Some checks are
//! performed at compile-time however most things (including) correct
//! default values) are only checked at runtime.
//!
//! **Note** Since version `0.3.0` it is required to provide a
//! database backend in order to compile `barrel`.
//!
//! The following code is a simple example of how to get going with
//! `barrel`
//!
//! ```rust
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
//!
//! `barrel` also supports more advanced types, such as `foreign(...)`
//! and `array(...)` however currently doesn't support nested Array
//! types on foreign keys (such as `array(array(foreign(...)))`). Each
//! column addition returns a Column object which can then be used to
//! provide further configuration.
//!
//! To generate SQL strings you have two options. If you just want to
//! run the migration yourself simply run `Migration::exec()` where
//! you provide a generic `SqlGenerator` type according to your
//! database backend
//!
//! ```rust
//! # #[cfg(feature = "pg")]
//! # use barrel::backend::Pg;
//! # use barrel::Migration;
//! # let mut m = Migration::new();
//! // Example for pgsql
//! # #[cfg(feature = "pg")]
//! m.make::<Pg>();
//! ```
//!
//! Alternatively, if you're a library developer and you want to more
//! easily embed `barrel` into your library you can simply implement
//! the `DatabaseExecutor` trait for a type of yours that knows how to
//! execute SQL.  Running a migration with `barrel` is then super
//! easy.
//!
//! ```rust
//! use barrel::connectors::SqlRunner;
//! # use barrel::Migration;
//! # #[cfg(feature = "pg")]
//! # use barrel::backend::Pg;
//!
//! struct MyRunner;
//! impl SqlRunner for MyRunner {
//!     fn execute<S: Into<String>>(&mut self, sql: S) {
//!         # let s: String = sql.into();
//!         // ...
//!     }
//! }
//!
//! # let mut m = Migration::new();
//! # let mut executor = MyRunner;
//! # #[cfg(feature = "pg")]
//! m.execute::<Pg, _>(&mut executor);
//! ```
//!
//! In this case `executor` is your provided type which implements the
//! required trait. You can read more about this in the
//! [connectors](connectors/index.html) module docs.
//!
//! If you find database-specific features or documentation lacking,
//! don't hesitate to open an issue/PR about it.

#[cfg(feature = "diesel")]
pub mod integrations;
#[cfg(feature = "diesel")]
pub use integrations::*;

pub mod backend;
pub mod connectors;
pub mod migration;
pub mod table;
pub mod types;

pub use backend::SqlVariant;
pub use migration::Migration;
pub use table::{Table, TableMeta};

#[cfg(test)]
mod tests;

use std::rc::Rc;

/// An enum set that represents a single change on a table
#[derive(Clone)]
pub enum TableChange {
    /// Add a column of a name and type
    AddColumn(String, types::Type),

    /// Change an existing column
    ChangeColumn(String, types::Type, Rc<dyn Fn(&mut types::Type)>),

    /// Simply rename a column
    RenameColumn(String, String),

    /// Remove a column
    DropColumn(String),

    /// Add some custom SQL if all else fails
    CustomLine(String),
}

/// An enum set that represents a single change on a database
#[derive(Clone)]
pub enum DatabaseChange {
    /// Create a new table
    CreateTable(Table, Rc<dyn Fn(&mut Table)>),

    /// Create a new table *only* if it doesn't exist yet
    CreateTableIfNotExists(Table, Rc<dyn Fn(&mut Table)>),

    /// Change fields on an existing table
    ChangeTable(Table, Rc<dyn Fn(&mut Table)>),

    /// Rename a table
    RenameTable(String, String),

    /// Drop an existing table
    DropTable(String),

    /// Only drop a table if it exists
    DropTableIfExists(String),
}

/// An enum set that represents operations done with and on indices
#[derive(Clone)]
pub enum IndexChange {
    /// Add a multi-column index
    AddIndex {
        index: String,
        table: String,
        columns: types::Type, // Should always be a `Index` type
    },

    /// Remove a multi-column index
    RemoveIndex(String, String),
}
