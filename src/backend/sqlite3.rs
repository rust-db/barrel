//! Sqlite3 implementation of a generator

use super::SqlGenerator;
use types::{BaseType, Type};

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

    fn add_column(ex: bool, name: &str, tt: &Type) -> String {
        let bt: BaseType = tt.get_inner();
        use self::BaseType::*;

        #[cfg_attr(rustfmt, rustfmt_skip)] /* This shouldn't be formatted. It's too long */
        format!(
            "{}{}{}",
            match bt {
                Text => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Varchar(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Primary => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Integer => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Float => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Double => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                UUID => unimplemented!(),
                Json => unimplemented!(),
                Boolean => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Date => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Binary => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Foreign(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Custom(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
                Array(it) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(Array(Box::new(*it))))
            },
            match (&tt.default).as_ref() {
                Some(ref m) => format!(" DEFAULT '{}'", m),
                _ => format!(""),
            },
            match tt.nullable {
                true => "",
                false => " NOT NULL",
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

    fn print_type(t: BaseType) -> String {
        use self::BaseType::*;
        match t {
            Text => format!("TEXT"),
            Varchar(l) => match l {
                0 => format!("VARCHAR"), // For "0" remove the limit
                _ => format!("VARCHAR({})", l),
            },
            Primary => format!("INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT"),
            Integer => format!("INTEGER"),
            Float => format!("REAL"),
            Double => format!("DOUBLE"),
            UUID => unimplemented!(),
            Boolean => format!("BOOLEAN"),
            Date => format!("DATE"),
            Json => panic!("Json is not supported by Sqlite3"),
            Binary => format!("BINARY"),
            Foreign(t) => format!("INTEGER REFERENCES {}", t),
            Custom(t) => format!("{}", t),
            Array(meh) => format!("{}[]", Sqlite::print_type(*meh)),
        }
    }
}
