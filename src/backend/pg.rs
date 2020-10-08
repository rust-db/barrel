//! Postgres implementation of a generator
//!
//! This module generates strings that are specific to Postgres
//! databases. They should be thoroughly tested via unit testing

use super::SqlGenerator;
use crate::types::{BaseType, Type};

/// A simple macro that will generate a schema prefix if it exists
macro_rules! prefix {
    ($schema:expr) => {
        $schema
            .map(|s| format!("\"{}\".", s))
            .unwrap_or_else(|| String::new())
    };
}

/// Postgres SQL generator backend
pub struct Pg;
impl SqlGenerator for Pg {
    fn create_table(name: &str, schema: Option<&str>) -> String {
        format!("CREATE TABLE {}\"{}\"", prefix!(schema), name)
    }

    fn create_table_if_not_exists(name: &str, schema: Option<&str>) -> String {
        format!("CREATE TABLE IF NOT EXISTS {}\"{}\"", prefix!(schema), name)
    }

    fn drop_table(name: &str, schema: Option<&str>) -> String {
        format!("DROP TABLE {}\"{}\"", prefix!(schema), name)
    }

    fn drop_table_if_exists(name: &str, schema: Option<&str>) -> String {
        format!("DROP TABLE IF EXISTS {}\"{}\"", prefix!(schema), name)
    }

    fn rename_table(old: &str, new: &str, schema: Option<&str>) -> String {
        let schema = prefix!(schema);
        format!(
            "ALTER TABLE {}\"{}\" RENAME TO {}\"{}\"",
            schema, old, schema, new
        )
    }

    fn alter_table(name: &str, schema: Option<&str>) -> String {
        format!("ALTER TABLE {}\"{}\"", prefix!(schema), name)
    }

    fn add_column(ex: bool, schema: Option<&str>, name: &str, tt: &Type) -> String {
        let bt: BaseType = tt.get_inner();
        use self::BaseType::*;

        #[cfg_attr(rustfmt, rustfmt_skip)] /* This shouldn't be formatted. It's too long */
        format!(
            "{}{}{}{}{}",
            match bt {
                Text => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Varchar(_) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Primary => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Integer => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Float => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Double => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                UUID => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Json => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Boolean => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Date => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Time => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                DateTime => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Binary => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Foreign(_, _, _) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Custom(_) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(bt, schema)),
                Array(it) => format!("{}\"{}\" {}", Pg::prefix(ex), name, Pg::print_type(Array(Box::new(*it)), schema)),
                Index(_) => unreachable!("Indices are handled via custom builder"),
            },
            match tt.primary {
                true => " PRIMARY KEY",
                false => "",
            },
            match (&tt.default).as_ref() {
                Some(ref m) => format!(" DEFAULT '{}'", m),
                _ => format!(""),
            },
            match tt.nullable {
                true => "",
                false => " NOT NULL",
            },
            match tt.unique {
                true => " UNIQUE",
                false => "",
            },
        )
    }

    fn drop_column(name: &str) -> String {
        format!("DROP COLUMN \"{}\"", name)
    }

    fn rename_column(old: &str, new: &str) -> String {
        format!("ALTER COLUMN \"{}\" RENAME TO \"{}\"", old, new)
    }

    fn create_index(table: &str, schema: Option<&str>, name: &str, _type: &Type) -> String {
        // FIXME: Implement PG specific index builder here
        format!(
            "CREATE {} INDEX \"{}\" ON {}\"{}\" ({})",
            match _type.unique {
                true => "UNIQUE",
                false => "",
            },
            name,
            prefix!(schema),
            table,
            match _type.inner {
                BaseType::Index(ref cols) => cols
                    .iter()
                    .map(|col| format!("\"{}\"", col))
                    .collect::<Vec<_>>()
                    .join(", "),
                _ => unreachable!(),
            }
        )
    }

    fn drop_index(name: &str) -> String {
        format!("DROP INDEX \"{}\"", name)
    }

    fn add_foreign_key(
        columns: &[String],
        table: &str,
        relation_columns: &[String],
        schema: Option<&str>,
    ) -> String {
        let columns: Vec<_> = columns.into_iter().map(|c| format!("\"{}\"", c)).collect();

        let relation_columns: Vec<_> = relation_columns
            .into_iter()
            .map(|c| format!("\"{}\"", c))
            .collect();

        format!(
            "FOREIGN KEY({}) REFERENCES {}\"{}\"({})",
            columns.join(","),
            prefix!(schema),
            table,
            relation_columns.join(","),
        )
    }
}

impl Pg {
    fn prefix(ex: bool) -> String {
        match ex {
            true => format!("ADD COLUMN "),
            false => format!(""),
        }
    }

    fn print_type(t: BaseType, schema: Option<&str>) -> String {
        use self::BaseType::*;
        match t {
            Text => format!("TEXT"),
            Varchar(l) => match l {
                0 => format!("VARCHAR"), // For "0" remove the limit
                _ => format!("VARCHAR({})", l),
            },
            /* "NOT NULL" is added here because normally primary keys are implicitly not-null */
            Primary => format!("SERIAL PRIMARY KEY NOT NULL"),
            Integer => format!("INTEGER"),
            Float => format!("FLOAT"),
            Double => format!("DOUBLE PRECISION"),
            UUID => format!("UUID"),
            Boolean => format!("BOOLEAN"),
            Date => format!("DATE"),
            Time => format!("TIME"),
            DateTime => format!("DATETIME"),
            Json => format!("JSON"),
            Binary => format!("BYTEA"),
            Foreign(s, t, refs) => format!(
                "INTEGER REFERENCES {}\"{}\"({})",
                prefix!(s.or(schema.map(|s| s.into()))),
                t,
                refs.0.join(",")
            ),
            Custom(t) => format!("{}", t),
            Array(meh) => format!("{}[]", Pg::print_type(*meh, schema)),
            Index(_) => unimplemented!("Indices are handled via custom builder"),
        }
    }
}
