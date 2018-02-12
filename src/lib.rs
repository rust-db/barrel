//! Powerful schema builder API in Rust, using Diesel in the backend.
//!
//! Barrel has two primary models, the `Migration` and the `Table`. A schema migration
//! is built with a variety of hooks that can be executed on tables, using static callbacks.
//!
//! ```
//! extern crate barrel;
//! use barrel::*;
//!
//! fn main() {
//!     let mut m = Migration::new();
//!     m.create_table("users", |t| {
//!         t.add_column("name", Type::Text);
//!         t.add_column("age", Type::Integer);
//!         t.add_column("owns_plushy_sharks", Type::Boolean);
//!     });
//!
//!     // I like plushy sharks
//!     m.rename_table("sharks", "plushies");
//! }
//! ```
//!
//! The above code, for example, will create a new table called "users". All tables implicitly
//! add an auto incrementing primary key called "id". This behaviour can't currently be turned
//! off. The callback is executed when calling `Migration::exec()`
//!
//! Barrel is designed to give you ease of use as well as power over how you write your
//! migrations and SQL schemas.

/// A generic trait that frameworks using barrel can implement
/// 
/// An object of this trait can be given to a `Migration` object to
/// automatically generate and run the given SQL string for a
/// database connection which is wrapped by it
pub trait DatabaseExecutor {

    /// Execute the migration on a backend
    fn execute<S: Into<String>>(&mut self, sql: S);
}

pub mod backend;
use backend::SqlGenerator;


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
    schema: String,
    changes: Vec<DatabaseChange>,
}

impl Migration {
    pub fn new() -> Migration {
        return Migration {
            schema: String::new(),
            changes: Vec::new(),
        };
    }

    /// Specify a database schema name for this migration
    pub fn schema<S: Into<String>>(mut self, schema: S) -> Migration {
        self.schema = schema.into();
        return self;
    }

    /// Creates the SQL for this migration for a specific backend
    /// 
    /// This function copies state and does not touch the original
    /// migration layout. This allows you to call `revert` later on
    /// in the process to auto-infer the down-behaviour
    pub fn make<T: SqlGenerator>(&self) -> String {
        let s = String::new();
        return s;
    }

    /// Automatically infer the `down` step of this migration
    /// 
    /// Will thrown an error if behaviour is ambigous or not
    /// possible to infer (e.g. revert a `drop_table`)
    pub fn revert<T: SqlGenerator>(&self) -> String {
        let s = String::new();
        return s;
    }

    /// Pass a reference to a migration toolkit runner which will
    /// automatically generate and execute 
    pub fn execute<T: DatabaseExecutor, S: SqlGenerator>(&self, runner: &mut T) {
        runner.execute(self.make::<S>());
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

    pub fn add_column<S: Into<String>>(&mut self, name: S, _type: Type) -> &mut Column {
        self.changes.push(TableChange::AddColumn(
            name.into(),
            Column {
                nullable: false,
                _type: _type,
                def: None,
            },
        ));

        return match self.changes.last_mut().unwrap() {
            &mut TableChange::AddColumn(_, ref mut c) => c,
            _ => unreachable!(),
        };
    }

    pub fn drop_column<S: Into<String>>(&mut self, name: S) {
        self.changes.push(TableChange::RemoveColumn(name.into()));
    }

    pub fn rename_column<S: Into<String>>(&mut self, old: S, new: S) {
        self.changes
            .push(TableChange::RenameColumn(old.into(), new.into()));
    }
}

/// 
pub struct TableMeta {
    has_id: bool,
    encoding: String,
}

pub struct Column {
    nullable: bool,
    _type: Type,
    def: Option<ColumnDefault>,
}

impl Column {

    /// Set a default value for this column
    pub fn default<T: Into<ColumnDefault>>(&mut self, data: T) -> &mut Column {
        let def = data.into();
        self.compare_types(&def);
        self.def = Some(def);
        return self;
    }

    /// Set a column to allow being null 
    pub fn nullable(&mut self) -> &mut Column {
        self.nullable = true;
        return self;
    }

    /// Check (at runtime) that the provided data matches the column type
    /// 
    /// This is not ideal. Not only is the code not very nice but it means that
    /// you can compile your migration tool without knowing if the migration will
    /// *actually* go through.
    /// 
    /// What would be much better is if the compiler could (somehow) check at
    /// compile-time if the data provided matches whatever the column type is.
    /// But I don't know how 😅
    fn compare_types(&self, def: &ColumnDefault) {
        match def {
            &ColumnDefault::Text(_) => if &self._type != &Type::Text {
                return;
            },
            &ColumnDefault::Integer(_) => if &self._type != &Type::Integer {
                return;
            },
            &ColumnDefault::Float(_) => if &self._type != &Type::Float {
                return;
            },
            &ColumnDefault::Boolean(_) => if &self._type != &Type::Boolean {
                return;
            },
        }
        panic!("Mismatched data type for `default` value!");
    }
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Text,
    Integer,
    Float,
    Boolean,
}

pub enum ColumnDefault {
    Text(String),
    Integer(i64),
    Float(f64), // Or just use 32-bit floats?
    Boolean(bool),
    // TODO: Figure out storage for other data types
}

impl From<&'static str> for ColumnDefault {
    fn from(data: &'static str) -> Self {
        return ColumnDefault::Text(data.into());
    }
}

impl From<i64> for ColumnDefault {
    fn from(data: i64) -> Self {
        return ColumnDefault::Integer(data);
    }
}

impl From<f64> for ColumnDefault {
    fn from(data: f64) -> Self {
        return ColumnDefault::Float(data);
    }
}

impl From<bool> for ColumnDefault {
    fn from(data: bool) -> Self {
        return ColumnDefault::Boolean(data);
    }
}
