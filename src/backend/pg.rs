//! Postgres implementation of a generator

use super::{SqlGenerator, Type};

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

    fn add_column(name: &str, _type: Type) -> String {
        use Type::*;
        return match _type {
            Text => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Varchar(_) => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Integer => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Float => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Double => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Boolean => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Binary => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Foreign(_) => format!("\"{}\" {}", name, Pg::print_type(_type)),
            Array(t) => { // FIXME: Doesn't support Array(Array(Foreign))
                return format!(
                    "\"{}\" {}",
                    name,
                    match *t {
                        Foreign(table) => format!("INTEGER[] REFERENCES {}", table),
                        _ => Pg::print_type(Array(Box::new(*t))),
                    }
                );
            }
        };
    }

    fn drop_column(name: &str) -> String {
        return format!("DROP COLUMN \"{}\"", name);
    }

    fn rename_column(old: &str, new: &str) -> String {
        return format!("ALTER COLUMN \"{}\" RENAME TO \"{}\"", old, new);
    }
}

impl Pg {
    fn print_type(t: Type) -> String {
        use Type::*;
        return match t {
            Text => format!("TEXT"),
            Varchar(l) => format!("VARCHAR({})", l),
            Integer => format!("INTEGER"),
            Float => format!("FLOAT"),
            Double => format!("DOUBLE"),
            Boolean => format!("BOOLEAN"),
            Binary => format!("BINARY"),
            Foreign(t) => format!("INTEGER REFERENCES {}", t),
            Array(meh) => format!("{}[]", Pg::print_type(*meh)),
            _ => unreachable!(),
        };
    }
}
