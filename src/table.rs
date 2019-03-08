//! A module that represents tables and columns
//!
//! A table is a collection of columns and some metadata. Creating
//! a table gives you access to the metadata fields that can only
//! be set when creating the table.
//!
//! You can also change existing tables with a closure that can
//! then access individual columns in that table.

use super::backend::SqlGenerator;
use super::TableChange;
use crate::types::Type;
use std::fmt::{Debug, Formatter, Result as FmtResult};

impl Debug for TableChange {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("TableChange")
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub meta: TableMeta,
    changes: Vec<TableChange>,
}

impl Table {
    pub fn new<S: Into<String>>(name: S) -> Table {
        return Table {
            meta: TableMeta::new(name.into()),
            changes: Vec::new(),
        };
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
        self.changes
            .push(TableChange::AddColumn(name.into(), _type));

        return match self.changes.last_mut().unwrap() {
            &mut TableChange::AddColumn(_, ref mut c) => c,
            _ => unreachable!(),
        };
    }

    pub fn drop_column<S: Into<String>>(&mut self, name: S) {
        self.changes.push(TableChange::DropColumn(name.into()));
    }

    pub fn rename_column<S: Into<String>>(&mut self, old: S, new: S) {
        self.changes
            .push(TableChange::RenameColumn(old.into(), new.into()));
    }

    pub fn make<T: SqlGenerator>(&mut self, ex: bool) -> Vec<String> {
        use TableChange::*;
        let mut s = Vec::new();

        for change in &mut self.changes {
            s.push(match change {
                &mut AddColumn(ref name, ref col) => T::add_column(ex, name, &col),
                &mut DropColumn(ref name) => T::drop_column(name),
                &mut RenameColumn(ref old, ref new) => T::rename_column(old, new),
                &mut ChangeColumn(ref mut name, _, _) => T::alter_table(name),
            });
        }

        return s;
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
    pub fn new(name: String) -> TableMeta {
        return TableMeta {
            name: name,
            encoding: "utf-8".to_owned(),
        };
    }

    /// Return a clone of the table name
    pub fn name(&self) -> String {
        return self.name.clone();
    }

    /// Specify an encoding for this table which might vary from the main encoding
    /// of your database
    pub fn encoding<S: Into<String>>(&mut self, enc: S) -> &mut TableMeta {
        self.encoding = enc.into();
        return self;
    }
}

