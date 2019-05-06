//! Core migration creation handler
//!
//! A migration can be done for a specific schema which contains
//! multiple additions or removables from a database or table.
//!
//! At the end of crafting a migration you can use `Migration::exec` to
//! get the raw SQL string for a database backend or `Migration::revert`
//! to try to auto-infer the migration rollback. In cases where that
//! can't be done the `Result<String, RevertError>` will not unwrap.
//!
//! You can also use `Migration::exec` with your SQL connection for convenience
//! if you're a library developer.

use crate::table::{Table, TableMeta};
use crate::DatabaseChange;

use crate::backend::SqlGenerator;
use crate::connectors::SqlRunner;

use std::rc::Rc;

/// Represents a schema migration on a database
pub struct Migration {
    #[doc(hidden)]
    pub schema: Option<String>,
    #[doc(hidden)]
    pub changes: Vec<DatabaseChange>,
}

impl Migration {
    pub fn new() -> Migration {
        Migration {
            schema: None,
            changes: Vec::new(),
        }
    }

    /// Specify a database schema name for this migration
    pub fn schema<S: Into<String>>(self, schema: S) -> Migration {
        Self { schema: Some(schema.into()), ..self }
    }

    /// Creates the SQL for this migration for a specific backend
    ///
    /// This function copies state and does not touch the original
    /// migration layout. This allows you to call `revert` later on
    /// in the process to auto-infer the down-behaviour
    pub fn make<T: SqlGenerator>(&self) -> String {
        use DatabaseChange::*;
        let mut s = String::new();

        /* What happens in make, stays in make (sort of) */
        let mut changes = self.changes.clone();
        let schema = self.schema.as_ref().map(|s| s.as_str());

        for change in &mut changes {
            match change {
                &mut CreateTable(ref mut t, ref mut cb)
                | &mut CreateTableIfNotExists(ref mut t, ref mut cb) => {
                    cb(t); // Run the user code
                    let vec = t.make::<T>(false, schema);

                    let name = t.meta.name().clone();
                    s.push_str(&match change {
                        CreateTable(_, _) => T::create_table(&name, schema),
                        CreateTableIfNotExists(_, _) => T::create_table_if_not_exists(&name, schema),
                        _ => unreachable!(),
                    });
                    s.push_str(" (");
                    let l = vec.len();
                    for (i, slice) in vec.iter().enumerate() {
                        s.push_str(slice);

                        if i < l - 1 {
                            s.push_str(", ");
                        }
                    }
                    s.push_str(")");
                }
                &mut DropTable(ref name) => s.push_str(&T::drop_table(name, schema)),
                &mut DropTableIfExists(ref name) => s.push_str(&T::drop_table_if_exists(name, schema)),
                &mut RenameTable(ref old, ref new) => s.push_str(&T::rename_table(old, new, schema)),
                &mut ChangeTable(ref mut t, ref mut cb) => {
                    cb(t);
                    let vec = t.make::<T>(true, schema);
                    s.push_str(&T::alter_table(&t.meta.name(), schema));
                    s.push_str(" ");
                    let l = vec.len();
                    for (i, slice) in vec.iter().enumerate() {
                        s.push_str(slice);

                        if i < l - 1 {
                            s.push_str(", ");
                        }
                    }
                }
            }

            s.push_str(";");
        }

        return s;
    }

    /// Automatically infer the `down` step of this migration
    ///
    /// Will thrown an error if behaviour is ambiguous or not
    /// possible to infer (e.g. revert a `drop_table`)
    pub fn revert<T: SqlGenerator>(&self) -> String {
        unimplemented!()
    }

    /// Pass a reference to a migration toolkit runner which will
    /// automatically generate and execute
    pub fn execute<S: SqlGenerator, T: SqlRunner>(&self, runner: &mut T) {
        runner.execute(self.make::<S>());
    }

    /// Create a new table with a specific name
    pub fn create_table<S: Into<String>, F: 'static>(&mut self, name: S, cb: F) -> &mut TableMeta
    where
        F: Fn(&mut Table),
    {
        self.changes
            .push(DatabaseChange::CreateTable(Table::new(name), Rc::new(cb)));

        return match self.changes.last_mut().unwrap() {
            &mut DatabaseChange::CreateTable(ref mut t, _) => &mut t.meta,
            _ => unreachable!(),
        };
    }

    /// Create a new table *only* if it doesn't exist yet
    pub fn create_table_if_not_exists<S: Into<String>, F: 'static>(
        &mut self,
        name: S,
        cb: F,
    ) -> &mut TableMeta
    where
        F: Fn(&mut Table),
    {
        self.changes.push(DatabaseChange::CreateTableIfNotExists(
            Table::new(name),
            Rc::new(cb),
        ));

        return match self.changes.last_mut().unwrap() {
            &mut DatabaseChange::CreateTableIfNotExists(ref mut t, _) => &mut t.meta,
            _ => unreachable!(),
        };
    }

    /// Change fields on an existing table
    pub fn change_table<S: Into<String>, F: 'static>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut Table),
    {
        let t = Table::new(name);
        let c = DatabaseChange::ChangeTable(t, Rc::new(cb));
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
