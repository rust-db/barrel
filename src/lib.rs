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

/// Represents a schema migration on a database
pub struct Migration {
    changes: Vec<DatabaseChange>,
}

impl Migration {
    pub fn new() -> Migration {
        return Migration {
            changes: Vec::new(),
        };
    }

    /// Create a new table with a specific name
    pub fn create_table<S: Into<String>, F: 'static>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut Table),
    {
        let t = Table::new(name);
        let c = DatabaseChange::CreateTable(t, Box::new(cb));
        self.changes.push(c);
    }

    /// Create a new table *only* if it doesn't exist yet
    pub fn create_table_if_not_exists<S: Into<String>, F: 'static>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut Table),
    {
        let t = Table::new(name);
        let c = DatabaseChange::CreateTableIfNotExists(t, Box::new(cb));
        self.changes.push(c);
    }

    /// Change fields on an existing table
    pub fn change_table<S: Into<String>, F: 'static>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut Table),
    {
        let t = Table::new(name);
        let c = DatabaseChange::ChangeTable(t, Box::new(cb));
        self.changes.push(c);
    }

    /// Rename a table
    pub fn rename_table<S: Into<String>>(&mut self, old: S, new: S) {
        self.changes
            .push(DatabaseChange::RenameTable(old.into(), new.into()));
    }

    /// Drop an existing table
    pub fn drop_table<S: Into<String>>(&mut self, name: S) {
        self.changes.push(DatabaseChange::DropTable(name.into()));
    }

    /// Only drop a table if it exists
    pub fn drop_table_if_exists<S: Into<String>>(&mut self, name: S) {
        self.changes
            .push(DatabaseChange::DropTableIfExists(name.into()));
    }
}

pub struct Table {
    name: String,
    changes: Vec<TableChange>,
}

impl Table {
    pub fn new<S: Into<String>>(name: S) -> Table {
        return Table {
            name: name.into(),
            changes: Vec::new(),
        };
    }

    pub fn add_column<S: Into<String>>(&mut self, name: S, _type: ColumnType) {
        self.changes.push(TableChange::AddColumn(name.into(), Column {
            nullable: false,
            _type: _type
        }));
    }

    pub fn remove_column<S: Into<String>>(&mut self, name: S) {
        self.changes.push(TableChange::RemoveColumn(name.into()));
    }

    pub fn rename_column<S: Into<String>>(&mut self, old: S, new: S) {
        self.changes.push(TableChange::RenameColumn(old.into(), new.into()));
    }
}


pub struct Column {
    nullable: bool,
    _type: ColumnType,
}

pub enum ColumnType {
    Text,
    Integer,
    Float,
    Boolean,
}
