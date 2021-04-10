//! Sqlite3 implementation of a generator

use super::SqlGenerator;
use crate::{
    functions::AutogenFunction,
    types::{BaseType, ReferentialAction, Type, WrappedDefault},
};

/// A simple macro that will generate a schema prefix if it exists
macro_rules! prefix {
    ($schema:expr) => {
        $schema
            .map(|s| format!("\"{}\".", s))
            .unwrap_or_else(|| String::new())
    };
}

/// We call this struct Sqlite instead of Sqlite3 because we hope not
/// to have to break the API further down the road
pub struct Sqlite;
impl SqlGenerator for Sqlite {
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
        format!("ALTER TABLE {}\"{}\" RENAME TO \"{}\"", schema, old, new)
    }

    fn alter_table(name: &str, schema: Option<&str>) -> String {
        format!("ALTER TABLE {}\"{}\"", prefix!(schema), name)
    }

    fn add_column(ex: bool, _: Option<&str>, name: &str, tt: &Type) -> String {
        let bt: BaseType = tt.get_inner();
        let btc = bt.clone();
        use self::BaseType::*;
        let primary_definition = match tt.primary {
            true => " PRIMARY KEY",
            false => "",
        };
        let default_definition = match (&tt.default).as_ref() {
            Some(ref m) => match m {
                WrappedDefault::Function(ref fun) => match fun {
                    AutogenFunction::CurrentTimestamp => format!(" DEFAULT CURRENT_TIMESTAMP"),
                },
                WrappedDefault::Null => format!(" DEFAULT NULL"),
                WrappedDefault::AnyText(ref val) => format!(" DEFAULT '{}'", val),
                WrappedDefault::UUID(ref val) => format!(" DEFAULT '{}'", val),
                WrappedDefault::Date(ref val) => format!(" DEFAULT '{:?}'", val),
                WrappedDefault::Boolean(val) => format!(" DEFAULT {}", if *val { 1 } else { 0 }),
                WrappedDefault::Custom(ref val) => format!(" DEFAULT '{}'", val),
                _ => format!(" DEFAULT {}", m),
            },
            _ => format!(""),
        };
        let nullable_definition = match tt.nullable {
            true => "",
            false => " NOT NULL",
        };
        let unique_definition = match tt.unique {
            true => " UNIQUE",
            false => "",
        };
        #[cfg_attr(rustfmt, rustfmt_skip)] /* This shouldn't be formatted. It's too long */
        let base_type_definition = match bt {
            Text => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Varchar(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Char(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Primary => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Integer => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Serial => panic!("SQLite has no serials for non-primary key columns"),
            Float => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Double => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            UUID => panic!("`UUID` not supported by Sqlite3. Use `Text` instead!"),
            Json => panic!("`Json` not supported by Sqlite3. Use `Text` instead!"),
            Boolean => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Date => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Time => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            DateTime => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Binary => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Foreign(_, _, _, _, _) => format!("{}\"{}\" INTEGER{} REFERENCES {}", Sqlite::prefix(ex), name, nullable_definition, Sqlite::print_type(bt)),
            Custom(_) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(bt)),
            Array(it) => format!("{}\"{}\" {}", Sqlite::prefix(ex), name, Sqlite::print_type(Array(Box::new(*it)))),
            Index(_) => unreachable!("Indices are handled via custom builder"),
            Constraint(_, _) => unreachable!("Constraints are handled via custom builder"),
        };

        match btc {
            Foreign(_, _, _, _, _) => {
                format!(
                    "{}{}{}{}",
                    base_type_definition, primary_definition, default_definition, unique_definition,
                )
            }
            _ => {
                format!(
                    "{}{}{}{}{}",
                    base_type_definition,
                    primary_definition,
                    default_definition,
                    nullable_definition,
                    unique_definition,
                )
            }
        }
        //#[cfg_attr(rustfmt, rustfmt_skip)] /* This shouldn't be formatted. It's too long */
        //format!(
        //    // SQL base - default - nullable - unique
        //    "{}{}{}{}{}",
        //    base_type_definition,
        //    primary_definition,
        //    default_definition,
        //    nullable_definition,
        //    unique_definition
        //)
    }

    /// Create a multi-column index
    fn create_index(table: &str, schema: Option<&str>, name: &str, _type: &Type) -> String {
        format!(
            "CREATE {} INDEX {}\"{}\" ON \"{}\" ({})",
            match _type.unique {
                true => "UNIQUE",
                false => "",
            },
            prefix!(schema),
            name,
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

    fn create_constraint(name: &str, _type: &Type) -> String {
        let (r#type, columns) = match _type.inner {
            BaseType::Constraint(ref r#type, ref columns) => (
                r#type.clone(),
                columns
                    .iter()
                    .map(|col| format!("\"{}\"", col))
                    .collect::<Vec<_>>(),
            ),
            _ => unreachable!(),
        };

        format!(
            "CONSTRAINT \"{}\" {} ({})",
            name,
            r#type,
            columns.join(", "),
        )
    }

    /// Drop a multi-column index
    fn drop_index(name: &str) -> String {
        format!("DROP INDEX \"{}\"", name)
    }

    fn drop_column(_: &str) -> String {
        panic!("Sqlite does not support dropping columns!")
    }

    fn rename_column(_: &str, _: &str) -> String {
        panic!("Sqlite does not support renaming columns!")
    }

    fn add_foreign_key(
        columns: &[String],
        table: &str,
        relation_columns: &[String],
        _: Option<&str>,
    ) -> String {
        let columns: Vec<_> = columns.into_iter().map(|c| format!("\"{}\"", c)).collect();

        let relation_columns: Vec<_> = relation_columns
            .into_iter()
            .map(|c| format!("\"{}\"", c))
            .collect();

        format!(
            "FOREIGN KEY({}) REFERENCES \"{}\"({})",
            columns.join(","),
            table,
            relation_columns.join(","),
        )
    }

    fn add_primary_key(columns: &[String]) -> String {
        let columns: Vec<_> = columns.into_iter().map(|c| format!("\"{}\"", c)).collect();
        format!("PRIMARY KEY ({})", columns.join(","))
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
            Char(l) => format!("CHAR({})", l),
            Primary => format!("INTEGER NOT NULL PRIMARY KEY"),
            Serial => panic!("SQLite has no serials for non-primary key columns"),
            Integer => format!("INTEGER"),
            Float => format!("REAL"),
            Double => format!("DOUBLE"),
            UUID => unimplemented!(),
            Boolean => format!("BOOLEAN"),
            Date => format!("DATE"),
            Time => format!("TIME"),
            DateTime => format!("DATETIME"),
            Json => panic!("Json is not supported by Sqlite3"),
            Binary => format!("BINARY"),
            Foreign(_, t, refs, on_update, on_delete) => {
                let d = match on_delete {
                    ReferentialAction::Unset => String::from(""),
                    _ => format!(" {}", on_delete.on_delete()),
                };
                let u = match on_update {
                    ReferentialAction::Unset => String::from(""),
                    _ => format!(" {}", on_update.on_update()),
                };
                format!("{}({}){}{}", t, refs.0.join(","), u, d)
            }
            Custom(t) => format!("{}", t),
            Array(meh) => format!("{}[]", Sqlite::print_type(*meh)),
            Index(_) => unimplemented!(),
            Constraint(_, _) => unreachable!("Constraints are handled via custom builder"),
        }
    }
}
