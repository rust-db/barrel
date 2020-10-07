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

use crate::backend::{SqlGenerator, SqlVariant};
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
        Self {
            schema: Some(schema.into()),
            ..self
        }
    }

    /// Creates the SQL for this migration for a specific backend
    ///
    /// This function copies state and does not touch the original
    /// migration layout. This allows you to call `revert` later on
    /// in the process to auto-infer the down-behaviour
    pub fn make<T: SqlGenerator>(&self) -> String {
        use DatabaseChange::*;

        /* What happens in make, stays in make (sort of) */
        let mut changes = self.changes.clone();
        let schema = self.schema.as_ref().map(|s| s.as_str());

        changes.iter_mut().fold(String::new(), |mut sql, change| {
            match change {
                &mut CreateTable(ref mut t, ref mut cb)
                | &mut CreateTableIfNotExists(ref mut t, ref mut cb) => {
                    cb(t); // Run the user code
                    let (cols, indices, foreign_keys) = t.make::<T>(false, schema);

                    let name = t.meta.name().clone();
                    sql.push_str(&match change {
                        CreateTable(_, _) => T::create_table(&name, schema),
                        CreateTableIfNotExists(_, _) => {
                            T::create_table_if_not_exists(&name, schema)
                        }
                        _ => unreachable!(),
                    });
                    sql.push_str(" (");
                    let l = cols.len();
                    for (i, slice) in cols.iter().enumerate() {
                        sql.push_str(slice);

                        if i < l - 1 {
                            sql.push_str(", ");
                        }
                    }

                    let l = foreign_keys.len();
                    for (i, slice) in foreign_keys.iter().enumerate() {
                        if cols.len() > 0 && i == 0 {
                            sql.push_str(", ")
                        }

                        sql.push_str(slice);

                        if i < l - 1 {
                            sql.push_str(", ")
                        }
                    }

                    sql.push_str(")");

                    // Add additional index columns
                    if indices.len() > 0 {
                        sql.push_str(";");
                        sql.push_str(&indices.join(";"));
                    }
                }
                &mut DropTable(ref name) => sql.push_str(&T::drop_table(name, schema)),
                &mut DropTableIfExists(ref name) => {
                    sql.push_str(&T::drop_table_if_exists(name, schema))
                }
                &mut RenameTable(ref old, ref new) => {
                    sql.push_str(&T::rename_table(old, new, schema))
                }
                &mut ChangeTable(ref mut t, ref mut cb) => {
                    cb(t);
                    let (cols, indices, fks) = t.make::<T>(true, schema);

                    sql.push_str(&T::alter_table(&t.meta.name(), schema));
                    sql.push_str(" ");

                    let l = cols.len();
                    for (i, slice) in cols.iter().enumerate() {
                        sql.push_str(slice);

                        if i < l - 1 {
                            sql.push_str(", ");
                        }
                    }

                    let l = fks.len();
                    for (i, slice) in fks.iter().enumerate() {
                        if cols.len() > 0 && i == 0 {
                            sql.push_str(", ")
                        }

                        sql.push_str("ADD ");
                        sql.push_str(slice);

                        if i < l - 1 {
                            sql.push_str(", ")
                        }
                    }

                    // Add additional index columns
                    if indices.len() > 0 {
                        sql.push_str(";");
                        sql.push_str(&indices.join(";"));
                    }
                }
                &mut CustomLine(ref raw_sql) => {
                    sql.push_str(raw_sql);
                }
            }

            sql.push_str(";");
            sql
        })
    }

    /// The same as `make` but making a run-time check for sql variant
    ///
    /// The `SqlVariant` type is populated based on the backends
    /// that are being selected at compile-time.
    ///
    /// This function panics if the provided variant is empty!
    pub fn make_from(&self, variant: SqlVariant) -> String {
        variant.run_for(self)
    }

    /// Inject a line of custom SQL into the top-level migration scope
    ///
    /// This is a bypass to the barrel typesystem, in case there is
    /// something your database supports that barrel doesn't, or if
    /// there is an issue with the way that barrel represents types.
    /// It does however mean that the SQL provided needs to be
    /// specific for one database, meaning that future migrations
    /// might become cumbersome.
    pub fn inject_custom<S: Into<String>>(&mut self, sql: S) {
        self.changes.push(DatabaseChange::CustomLine(sql.into()));
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

        match self.changes.last_mut().unwrap() {
            &mut DatabaseChange::CreateTable(ref mut t, _) => &mut t.meta,
            _ => unreachable!(),
        }
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

        match self.changes.last_mut().unwrap() {
            &mut DatabaseChange::CreateTableIfNotExists(ref mut t, _) => &mut t.meta,
            _ => unreachable!(),
        }
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
