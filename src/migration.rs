//! A module which handles a migration state
//! 
//! 

use super::table::{Table, TableMeta};
use super::{DatabaseChange, Type};

use super::connectors::DatabaseExecutor;
use super::backend::SqlGenerator;


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
    pub fn create_table<S: Into<String>, F: 'static>(&mut self, name: S, cb: F) -> &mut TableMeta
    where
        F: Fn(&mut Table),
    {
        let t = Table::new(name);
        t.add_column("id", Type::Integer).increments();
        let c = DatabaseChange::CreateTable(t, Box::new(cb));
        self.changes.push(c);

        return match self.changes.last_mut().unwrap() {
            &mut DatabaseChange::CreateTable(ref mut t, _) => &mut t.meta,
            _ => unreachable!(),
        };
    }

    /// Create a new table *only* if it doesn't exist yet
    pub fn create_table_if_not_exists<S: Into<String>, F: 'static>(&mut self, name: S, cb: F) -> &mut TableMeta
    where
        F: Fn(&mut Table),
    {
        let t = Table::new(name);
        t.add_column("id", Type::Integer).increments();
        let c = DatabaseChange::CreateTableIfNotExists(t, Box::new(cb));
        self.changes.push(c);

        return match self.changes.last_mut().unwrap() {
            &mut DatabaseChange::CreateTable(ref mut t, _) => &mut t.meta,
            _ => unreachable!(),
        };
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

