//! Postgres implementation of a generator
//!
//! This module generates strings that are specific to Postgres
//! databases. They should be thoroughly tested via unit testing

use super::{Column, SqlGenerator, Type};

pub struct Pg;
impl SqlGenerator for Pg {
    fn create_table(name: &str) -> String {
        return format!("CREATE TABLE \"{}\"", name);
    }

    fn create_table_if_not_exists(name: &str) -> String {
        return format!("CREATE TABLE \"{}\" IF NOT EXISTS", name);
    }

    fn drop_table(name: &str) -> String {
        return format!("DROP TABLE \"{}\"", name);
    }

    fn drop_table_if_exists(name: &str) -> String {
        return format!("DROP TABLE \"{}\" IF EXISTS", name);
    }

    fn rename_table(old: &str, new: &str) -> String {
        return format!("ALTER TABLE \"{}\" RENAME TO \"{}\"", old, new);
    }

    fn alter_table(name: &str) -> String {
        return format!("ALTER TABLE \"{}\"", name);
    }

    fn add_column(ex: bool, name: &str, column: &Column) -> String {
        use Type::*;
        let t: Type = column._type.clone();

        #[rustfmt_skip] /* This shouldn't be formatted. It's too long */
        let mut s = match t {
            Primary => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Text => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Varchar(_) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Integer => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Float => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Double => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Boolean => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Binary => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Foreign(_) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Custom(_) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type( t )),
            Array(it) => format!("{}\"{}\" {}",Pg::prefix(ex),name,Pg::print_type( Array(box *it) )),
        };

        /* Add default value if it exists */
        s.push_str(&match (&column.def).as_ref() {
            Some(ref m) => format!(" DEFAULT '{}'", m),
            _ => format!(""),
        });

        /* Include `not null` attributes  */
        s.push_str(&match column.nullable {
            true => format!(" NOT NULL"),
            false => format!(""),
        });

        return s;
    }

    fn drop_column(name: &str) -> String {
        return format!("DROP COLUMN \"{}\"", name);
    }

    fn rename_column(old: &str, new: &str) -> String {
        return format!("ALTER COLUMN \"{}\" RENAME TO \"{}\"", old, new);
    }
}

impl Pg {
    fn prefix(ex: bool) -> String {
        return match ex {
            true => format!("ADD COLUMN "),
            false => format!(""),
        };
    }

    fn print_type(t: Type) -> String {
        use Type::*;
        return match t {
            Primary => format!("SERIAL PRIMARY KEY"),
            Text => format!("TEXT"),
            Varchar(l) => match l {
                0 => format!("VARCHAR"), // For "0" remove the limit
                _ => format!("VARCHAR({})", l),
            },
            Integer => format!("INTEGER"),
            Float => format!("FLOAT"),
            Double => format!("DOUBLE"),
            Boolean => format!("BOOLEAN"),
            Binary => format!("BINARY"),
            Custom(t) => format!("{}", t),
            Foreign(t) => format!("INTEGER REFERENCES {}", t),
            Array(meh) => format!("{}[]", Pg::print_type(*meh)),
        };
    }
}
