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
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use types::Type;

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

    // pub fn add_primary<S: Into<String>>(&mut self, name: S) -> &mut Type {
    //     self.changes.push(TableChange::AddColumn(
    //         name.into(),
    //         Column {
    //             indexed: true,
    //             unique: true,
    //             nullable: false,
    //             increments: true,
    //             _type: Type::Integer,
    //             def: None,
    //         },
    //     ));

    //     return match self.changes.last_mut().unwrap() {
    //         &mut TableChange::AddColumn(_, ref mut c) => c,
    //         _ => unreachable!(),
    //     };
    // }

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
            println!("in make's change loop change: {:?}", change.clone());
            s.push(match change {
                &mut AddColumn(ref name, ref col) => {
                    let mut s = T::add_column(ex, name, &col);
                    s
                }
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
    pub has_id: bool,
    pub encoding: String,
}

impl TableMeta {
    /// Create a new tablemeta with default values
    pub fn new(name: String) -> TableMeta {
        return TableMeta {
            name: name,
            has_id: true,
            encoding: "utf-8".to_owned(),
        };
    }

    /// Return a clone of the table name
    pub fn name(&self) -> String {
        return self.name.clone();
    }

    /// Disable the auto-key feature
    ///
    /// A table is by default created with an auto-incrementing primary
    /// key called "id". You can disable this feature here. If you do and still
    /// want a priamry key, you will have to specify it yourself in the table
    /// init closure
    pub fn without_id(&mut self) -> &mut TableMeta {
        self.has_id = false;
        return self;
    }

    /// Specify an encoding for this table which might vary from the main encoding
    /// of your database
    pub fn encoding<S: Into<String>>(&mut self, enc: S) -> &mut TableMeta {
        self.encoding = enc.into();
        return self;
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    /// Is this a unique key
    pub unique: bool,

    /// Should the database create an index
    pub indexed: bool,

    /// Can this column be NULL
    pub nullable: bool,

    /// Does it auto-increment
    pub increments: bool,

    /// What's the column type
    pub _type: Type,

    /// What's default value records in this column
    pub def: Option<ColumnDefault>,
}

impl Column {
    /// Lazy constructor mostly used in unit tests
    pub fn new(t: Type) -> Column {
        return Column {
            indexed: false,
            unique: false,
            nullable: false,
            increments: false,
            _type: t,
            def: None,
        };
    }

    /// Set a default value for this column
    pub fn default<T: Into<ColumnDefault>>(&mut self, data: T) -> &mut Column {
        self.def = Some(data.into());
        return self;
    }

    /// Set a column to allow being null
    pub fn nullable(&mut self) -> &mut Column {
        self.nullable = true;
        return self;
    }

    /// Setup this column to automatically increment (such as integers)
    ///
    /// Throws an error if the column type *can't* increment (like booleans)
    pub fn increments(&mut self) -> &mut Column {
        self.increments = true;
        return self;
    }
}

#[derive(Debug, Clone)]
pub enum ColumnDefault {
    Text(String),
    Varchar(usize),
    Integer(i64),
    Float(f64), // FIXME: Or just use 32-bit floats?
    Boolean(bool),
    Date(String),
    /// A foreign key has a table and id it points to
    Foreign(String, u64),
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

impl From<i32> for ColumnDefault {
    fn from(data: i32) -> Self {
        return ColumnDefault::Integer(data as i64);
    }
}

impl From<usize> for ColumnDefault {
    fn from(data: usize) -> Self {
        return ColumnDefault::Integer(data as i64);
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

impl Display for ColumnDefault {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::ColumnDefault::*;
        return write!(
            f,
            "{}",
            &match *self {
                Text(ref val) => format!("{}", val),
                Varchar(ref val) => format!("{}", val),
                Integer(ref val) => format!("{}", val),
                Float(ref val) => format!("{}", val),
                Date(ref val) => format!("{}", val),
                Boolean(ref val) => match val {
                    &true => format!("t"),
                    &false => format!("f"),
                },
                Foreign(ref val, _) => format!("{}", val),
            }
        );
    }
}
