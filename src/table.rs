//! A module that handles migration state for Tables and Columns
//!
//!

use super::{TableChange, Type};

#[derive(Clone)]
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

    pub fn add_column<S: Into<String>>(&mut self, name: S, _type: Type) -> &mut Column {
        self.changes.push(TableChange::AddColumn(
            name.into(),
            Column {
                nullable: false,
                increments: false,
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
#[derive(Clone)]
pub struct TableMeta {
    name: String,
    has_id: bool,
    encoding: String,
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

#[derive(Clone)]
pub struct Column {
    nullable: bool,
    increments: bool,
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

    /// Setup this column to automatically increment (such as integers)
    /// 
    /// Throws an error if the column type *can't* increment (like booleans)
    pub fn increments(&mut self) -> &mut Column {
        self.increments = true;
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

            // FIXME: This comparison is now broken
            _ => {}
        }
        panic!("Mismatched data type for `default` value!");
    }
}

#[derive(Clone)]
pub enum ColumnDefault {
    Text(String),
    Varchar(usize),
    Integer(i64),
    Float(f64), // FIXME: Or just use 32-bit floats?
    Boolean(bool),

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
