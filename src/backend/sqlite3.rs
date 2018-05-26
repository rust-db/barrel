//! Sqlite3 implementation of a generator

use super::{Column, SqlGenerator, Type};

/// We call this struct Sqlite instead of Sqlite3 because we hope not 
/// to have to break the API further down the road 
pub struct Sqlite;
impl SqlGenerator for Sqlite {

    fn create_table(name: &str) -> String {
        format!("CREATE TABLE \"{}\"", name)
    }

    fn create_table_if_not_exists(name: &str) -> String {
        format!("CREATE TABLE IF NOT EXISTS \"{}\"", name)
    }

    fn drop_table(name: &str) -> String {
        format!("DROP TABLE \"{}\"", name)
    }

    fn drop_table_if_exists(name: &str) -> String {
        format!("DROP TABLE IF EXISTS \"{}\"", name)
    }

    fn rename_table(old: &str, new: &str) -> String {
        format!("ALTER TABLE \"{}\" RENAME TO \"{}\"", old, new)
    }

    fn alter_table(name: &str) -> String {
        format!("ALTER TABLE \"{}\"", name)
    }

   fn add_column(ex: bool, name: &str, column: &Column) -> String {
        use Type::*;
        let t: Type = column._type.clone();

        #[cfg_attr(rustfmt, rustfmt_skip)] /* This shouldn't be formatted. It's too long */
        format!(
            "{} {} {}",
            match t {
                Primary => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Text => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Varchar(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Integer => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Float => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Double => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Boolean => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Binary => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Foreign(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Custom(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(t)),
                Array(it) => format!("{}\"{}\" {}",Sqlite::prefix(ex),name,Sqlite::print_type(Array(Box::new(*it)))
                ),
            },
            match (&column.def).as_ref() {
                Some(ref m) => format!(" DEFAULT '{}'", m),
                _ => format!(""),
            },
            match column.nullable {
                true => " NOT NULL",
                false => "",
            }
        )
    }

    #[allow(unused_variables)]
    fn drop_column(name: &str) -> String {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn rename_column(old: &str, new: &str) -> String {
        unimplemented!()
    }
}

impl Sqlite {
    fn prefix(ex: bool) -> String {
        match ex {
            true => format!("ADD COLUMN "),
            false => format!(""),
        }
    }

    fn print_type(t: Type) -> String {
        use Type::*;
        match t {
            Primary => format!("INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT"),
            Text => format!("TEXT"),
            Varchar(l) => match l {
                0 => format!("VARCHAR"), // For "0" remove the limit
                _ => format!("VARCHAR({})", l),
            },
            Integer => format!("INTEGER"),
            Float => format!("REAL"),
            Double => format!("DOUBLE"),
            Boolean => format!("BOOLEAN"),
            Binary => format!("BINARY"),
            Custom(t) => format!("{}", t),
            Foreign(t) => format!("INTEGER REFERENCES {}", t),
            Array(meh) => format!("{}[]", Sqlite::print_type(*meh)),
        }
    }
}
