//! A module that represents tables and columns
//!
//! A table is a collection of columns and some metadata. Creating
//! a table gives you access to the metadata fields that can only
//! be set when creating the table.
//!
//! You can also change existing tables with a closure that can
//! then access individual columns in that table.

use super::{backend::SqlGenerator, ForeignKeyChange, IndexChange, PrimaryKeyChange, TableChange};
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

impl Debug for ForeignKeyChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("ForeignKeyChange")
    }
}

impl Debug for PrimaryKeyChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("PrimaryKeyChange")
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub meta: TableMeta,
    columns: Vec<TableChange>,
    indices: Vec<IndexChange>,
    foreign_keys: Vec<ForeignKeyChange>,
    primary_key: Option<PrimaryKeyChange>,
}

#[derive(Debug, Clone)]
pub struct SqlChanges {
    pub(crate) columns: Vec<String>,
    pub(crate) indices: Vec<String>,
    pub(crate) foreign_keys: Vec<String>,
    pub(crate) primary_key: Option<String>,
}

impl Table {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            meta: TableMeta::new(name.into()),
            columns: vec![],
            indices: vec![],
            foreign_keys: vec![],
            primary_key: None,
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

    /// Inject a line of custom SQL into the table block
    ///
    /// This is a bypass to the barrel typesystem, in case there is
    /// something your database supports that barrel doesn't, or if
    /// there is an issue with the way that barrel represents types.
    /// It does however mean that the SQL provided needs to be
    /// specific for one database, meaning that future migrations
    /// might become cumbersome.
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

    pub fn add_partial_index<S: Into<String>>(&mut self, name: S, columns: Type, conditions: S) {
        match columns.inner {
            crate::types::BaseType::Index(_) => {}
            _ => panic!("Calling `add_index` with a non-`Index` type is not allowed!"),
        }

        self.indices.push(IndexChange::AddPartialIndex {
            table: self.meta.name.clone(),
            index: name.into(),
            columns,
            conditions: conditions.into(),
        });
    }

    /// Drop an index on this table
    pub fn drop_index<S: Into<String>>(&mut self, name: S) {
        self.indices.push(IndexChange::RemoveIndex(
            self.meta.name.clone(),
            name.into(),
        ));
    }

    pub fn add_foreign_key(
        &mut self,
        columns_on_this_side: &[&str],
        related_table: &str,
        columns_on_that_side: &[&str],
    ) {
        let table = related_table.into();

        let columns = columns_on_this_side
            .into_iter()
            .map(|c| String::from(*c))
            .collect();

        let relation_columns = columns_on_that_side
            .into_iter()
            .map(|c| String::from(*c))
            .collect();

        self.foreign_keys.push(ForeignKeyChange::AddForeignKey {
            table,
            columns,
            relation_columns,
        })
    }

    pub fn set_primary_key(&mut self, columns: &[&str]) {
        let primary_key =
            PrimaryKeyChange::AddPrimaryKey(columns.into_iter().map(|s| s.to_string()).collect());

        self.primary_key = Some(primary_key);
    }

    /// Generate Sql for this table.
    pub fn make<T: SqlGenerator>(&mut self, ex: bool, schema: Option<&str>) -> SqlChanges {
        use ForeignKeyChange as KFC;
        use IndexChange as IC;
        use PrimaryKeyChange as PKC;
        use TableChange as TC;

        let columns = self
            .columns
            .iter_mut()
            .map(|change| match change {
                &mut TC::AddColumn(ref name, ref col) => T::add_column(ex, schema, name, &col),
                &mut TC::DropColumn(ref name) => T::drop_column(name),
                &mut TC::RenameColumn(ref old, ref new) => T::rename_column(old, new),
                &mut TC::ChangeColumn(ref mut name, _, _) => T::alter_table(name, schema),
                &mut TC::CustomLine(ref sql) => sql.clone(),
            })
            .collect();

        let indices = self
            .indices
            .iter()
            .map(|change| match change {
                IC::AddIndex {
                    index,
                    table,
                    columns,
                } => T::create_index(table, schema, index, columns),
                IC::AddPartialIndex {
                    index,
                    table,
                    columns,
                    conditions,
                } => T::create_partial_index(table, schema, index, columns, conditions),
                IC::RemoveIndex(_, index) => T::drop_index(index),
            })
            .collect();

        let primary_key = self.primary_key.as_ref().map(|pk| match pk {
            PKC::AddPrimaryKey(ref cols) => T::add_primary_key(cols),
        });

        let foreign_keys = self
            .foreign_keys
            .iter()
            .map(|change| match change {
                KFC::AddForeignKey {
                    columns,
                    table,
                    relation_columns,
                } => T::add_foreign_key(
                    columns.as_slice(),
                    table,
                    relation_columns.as_slice(),
                    schema,
                ),
            })
            .collect();

        SqlChanges {
            columns,
            indices,
            foreign_keys,
            primary_key,
        }
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
