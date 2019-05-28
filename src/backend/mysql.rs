//! MySQL implementation of a generator
//!
//! This module generates strings that are specific to MySQL
//! databases. They should be thoroughly tested via unit testing

use super::SqlGenerator;
use crate::types::{BaseType, Type};

/// A simple macro that will generate a schema prefix if it exists
macro_rules! prefix {
    ($schema:expr) => {
        $schema
            .map(|s| format!("{}.", s))
            .unwrap_or_else(|| String::new())
    };
}

/// MySQL generator backend
pub struct MySql;
impl SqlGenerator for MySql {
    fn create_table(name: &str, schema: Option<&str>) -> String {
        format!("CREATE TABLE {}{}", prefix!(schema), name)
    }

    fn create_table_if_not_exists(name: &str, schema: Option<&str>) -> String {
        format!("CREATE TABLE {}{} IF NOT EXISTS", prefix!(schema), name)
    }

    fn drop_table(name: &str, schema: Option<&str>) -> String {
        format!("DROP TABLE {}{}", prefix!(schema), name)
    }

    fn drop_table_if_exists(name: &str, schema: Option<&str>) -> String {
        format!("DROP TABLE {}{} IF EXISTS", prefix!(schema), name)
    }

    fn rename_table(old: &str, new: &str, schema: Option<&str>) -> String {
        let schema = prefix!(schema);
        format!("RENAME TABLE `{}{}` TO `{}{}`", schema, old, schema, new)
    }

    fn alter_table(name: &str, schema: Option<&str>) -> String {
        format!("ALTER TABLE `{}{}`", prefix!(schema), name)
    }

    fn add_column(ex: bool, name: &str, tt: &Type) -> String {
        let bt: BaseType = tt.get_inner();
        use self::BaseType::*;

        #[cfg_attr(rustfmt, rustfmt_skip)] /* This shouldn't be formatted. It's too long */
        format!(
            "{}{}{}{}{}",
            match bt {
                Text => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Varchar(_) => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Primary => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Integer => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Float => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Double => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                UUID => unimplemented!(),
                Json => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Boolean => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Date => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Binary => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Foreign(_) => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Custom(_) => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(bt)),
                Array(it) => format!("{}{} {}", MySql::prefix(ex), name, MySql::print_type(Array(Box::new(*it)))),
                Index(_) => unreachable!(),
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
        format!("CHANGE COLUMN \"{}\"  \"{}\"", old, new)
    }

    fn create_index(table: &str, schema: Option<&str>, name: &str, _type: &Type) -> String {
        // FIXME: Implement Mysql specific index builder here
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
}

impl MySql {
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
            /* "NOT NULL" is added here because normally primary keys are implicitly not-null */
            Primary => format!("INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY"),
            Integer => format!("INTEGER"),
            Float => format!("FLOAT"),
            Double => format!("DOUBLE"),
            UUID => format!("CHAR(36)"),
            Boolean => format!("BOOLEAN"),
            Date => format!("DATE"),
            Json => format!("JSON"),
            Binary => format!("BYTEA"),
            Foreign(t) => format!("INTEGER REFERENCES {}", t),
            Custom(t) => format!("{}", t),
            Array(meh) => format!("{}[]", MySql::print_type(*meh)),
            Index(_) => unreachable!(),
        }
    }
}
