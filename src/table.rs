//! A module that represents tables and columns
//!
//! A table is a collection of columns and some metadata. Creating
//! a table gives you access to the metadata fields that can only
//! be set when creating the table.
//!
//! You can also change existing tables with a closure that can
//! then access individual columns in that table.

use super::backend::SqlGenerator;
use super::{IndexChange, TableChange};
use crate::types::Type;
use std::fmt::{Debug, Formatter, Result as FmtResult};

impl Debug for TableChange {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("TableChange")
    }
}

impl Debug for IndexChange {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("IndexChange")
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub meta: TableMeta,
    columns: Vec<TableChange>,
    indices: Vec<IndexChange>,
}

impl Table {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            meta: TableMeta::new(name.into()),
            columns: vec![],
            indices: vec![],
        }
    }

    /// Add a new column to a table
    ///
    /// ```rust
    /// # use barrel::{types, Migration};
    /// # let mut m = Migration::new();
    /// # m.create_table("users", |table| {
    /// table.add_column("id", types::primary());
    /// table.add_column("name", types::varchar(64));
    /// # });
    /// ```
    pub fn add_column<S: Into<String>>(&mut self, name: S, _type: Type) -> &mut Type {
        self.columns
            .push(TableChange::AddColumn(name.into(), _type));

        match self.columns.last_mut().unwrap() {
            &mut TableChange::AddColumn(_, ref mut c) => c,
            _ => unreachable!(),
        }
    }

    pub fn drop_column<S: Into<String>>(&mut self, name: S) {
        self.columns.push(TableChange::DropColumn(name.into()));
    }

    pub fn rename_column<S: Into<String>>(&mut self, old: S, new: S) {
        self.columns
            .push(TableChange::RenameColumn(old.into(), new.into()));
    }

    pub fn inject_custom<S: Into<String>>(&mut self, sql: S) {
        self.columns.push(TableChange::CustomLine(sql.into()));
    }

    /// Add a new index to a table, spanning over multiple columns
    pub fn add_index<S: Into<String>>(&mut self, name: S, columns: Type) {
        match columns.inner {
            crate::types::BaseType::Index(_) => {}
            _ => panic!("Calling `add_index` with a non-`Index` type is not allowed!"),
        }

        self.indices.push(IndexChange::AddIndex {
            table: self.meta.name.clone(),
            index: name.into(),
            columns,
        });
    }

    /// Drop an index on this table
    pub fn drop_index<S: Into<String>>(&mut self, name: S) {
        self.indices.push(IndexChange::RemoveIndex(
            self.meta.name.clone(),
            name.into(),
        ));
    }

    /// Generate Sql for this table, returned as two vectors
    ///
    /// The first vector (`.0`) represents all column changes done to the table,
    /// the second vector (`.1`) contains all index and suffix changes.
    ///
    /// It is very well possible for either of them to be empty,
    /// although both being empty *might* signify an error.
    pub fn make<T: SqlGenerator>(
        &mut self,
        ex: bool,
        schema: Option<&str>,
    ) -> (Vec<String>, Vec<String>) {
        use IndexChange as IC;
        use TableChange as TC;

        let columns = self
            .columns
            .iter_mut()
            .map(|change| match change {
                &mut TC::AddColumn(ref name, ref col) => T::add_column(ex, name, &col),
                &mut TC::DropColumn(ref name) => T::drop_column(name),
                &mut TC::RenameColumn(ref old, ref new) => T::rename_column(old, new),
                &mut TC::ChangeColumn(ref mut name, _, _) => T::alter_table(name, schema),
                &mut TC::CustomLine(ref sql) => sql.clone(),
            })
            .collect();

        let indeces = self
            .indices
            .iter()
            .map(|change| match change {
                IC::AddIndex {
                    index,
                    table,
                    columns,
                } => T::create_index(table, schema, index, columns),
                IC::RemoveIndex(table, index) => T::drop_index(table, schema, index),
            })
            .collect();

        (columns, indeces)
    }
}

/// Some metadata about a table that was just created
#[derive(Debug, Clone)]
pub struct TableMeta {
    pub name: String,
    pub encoding: String,
}

impl TableMeta {
    /// Create a new tablemeta with default values
    pub fn new(name: String) -> Self {
        Self {
            name,
            encoding: "utf-8".to_owned(),
        }
    }

    /// Get a clone of the table name
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Specify an encoding for this table which might vary from the main encoding
    /// of your database
    pub fn encoding<S: Into<String>>(&mut self, enc: S) -> &mut TableMeta {
        self.encoding = enc.into();
        self
    }
}
