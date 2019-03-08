use crate::{
    types::{self, Type},
    Migration,
};
use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum DataTypes {
    Bool,
    F64,
    I64,
    String,
}

impl DataTypes {
    #[allow(unused)]
    pub fn string(&self) -> &str {
        match *self {
            DataTypes::Bool => "bool",
            DataTypes::F64 => "f64",
            DataTypes::I64 => "i64",
            DataTypes::String => "String",
        }
    }

    pub fn to_database_type(&self) -> Type {
        match *self {
            DataTypes::Bool => types::text(),
            DataTypes::F64 => types::double(),
            DataTypes::I64 => types::integer(),
            DataTypes::String => types::text(),
        }
    }
}

impl fmt::Debug for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            DataTypes::Bool => "",
            DataTypes::F64 => "f64",
            DataTypes::I64 => "i64",
            DataTypes::String => "string",
        };
        write!(f, "{:#?}", printable)
    }
}

#[derive(Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataTypes,
}

impl ColumnDef {
    pub fn new(name: String, data_type: DataTypes) -> ColumnDef {
        ColumnDef {
            name: name,
            data_type: data_type,
        }
    }
}

impl fmt::Debug for ColumnDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.name, self.data_type)
    }
}

pub fn create_table_if_not_exists(name: &str, columns: &Vec<ColumnDef>) {
    let mut m = Migration::new();
    let cols = columns.clone();
    m.create_table(name, move |t| {
        for cd in &cols {
            let cname: &str = &cd.name;
            t.add_column(cname, cd.data_type.to_database_type());
        }
    })
    .without_id();
}

#[test]
fn barrel_reverse_integration() {
    let cols = vec![
        ColumnDef::new("name".into(), DataTypes::String),
        ColumnDef::new("age".into(), DataTypes::I64),
        ColumnDef::new("coolness".into(), DataTypes::F64),
        ColumnDef::new("plushy_sharks_owned".into(), DataTypes::Bool),
    ];

    // We just call this function and hope it doesn't panic
    create_table_if_not_exists("users", &cols);
}
