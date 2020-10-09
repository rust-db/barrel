//! Microsoft SQL Server implementation of a generator
//!
//! This module generates strings that are specific to SQL Server
//! databases. They should be thoroughly tested via unit testing

use super::SqlGenerator;
use crate::{
    functions::AutogenFunction,
    types::{BaseType, Type, WrappedDefault},
};

/// A simple macro that will generate a quoted schema prefix if it exists
macro_rules! quoted_prefix {
    ($schema:expr) => {
        $schema
            .map(|s| format!("[{}].", s))
            .unwrap_or_else(|| String::new())
    };
}

/// A simple macro that will generate a schema prefix if it exists
macro_rules! prefix {
    ($schema:expr) => {
        $schema
            .map(|s| format!("{}.", s))
            .unwrap_or_else(|| String::new())
    };
}

/// SQL Server generator backend
pub struct MsSql;

impl MsSql {
    fn default(default: &WrappedDefault<'static>) -> String {
        match default {
            WrappedDefault::Function(ref fun) => match fun {
                AutogenFunction::CurrentTimestamp => format!(" DEFAULT CURRENT_TIMESTAMP"),
            },
            WrappedDefault::Null => format!(" DEFAULT NULL"),
            WrappedDefault::AnyText(ref val) => format!(" DEFAULT '{}'", val),
            WrappedDefault::UUID(ref val) => format!(" DEFAULT '{}'", val),
            WrappedDefault::Date(ref val) => format!(" DEFAULT '{:?}'", val),
            WrappedDefault::Boolean(val) => format!(" DEFAULT {}", if *val { 1 } else { 0 }),
            WrappedDefault::Custom(ref val) => format!(" DEFAULT '{}'", val),
            _ => format!(" DEFAULT {}", default),
        }
    }
}

impl SqlGenerator for MsSql {
    fn create_table(name: &str, schema: Option<&str>) -> String {
        format!("CREATE TABLE {}[{}]", quoted_prefix!(schema), name)
    }

    fn create_table_if_not_exists(name: &str, schema: Option<&str>) -> String {
        let table = format!("{}[{}]", quoted_prefix!(schema), name);

        match schema {
            None => {
                format!(
                    "IF NOT EXISTS (SELECT * FROM sys.tables WHERE name='{table}') CREATE TABLE {quoted}",
                    table = name,
                    quoted = table,
                )
            }
            Some(schema) => {
                format!(
                    "IF NOT EXISTS (SELECT * FROM sys.tables WHERE name='{table}' AND SCHEMA_NAME(schema_id) = '{schema}') CREATE TABLE {quoted}",
                    table = name,
                    quoted = table,
                    schema = schema,
                )
            }
        }
    }

    fn drop_table(name: &str, schema: Option<&str>) -> String {
        format!("DROP TABLE {}[{}]", quoted_prefix!(schema), name)
    }

    fn drop_table_if_exists(name: &str, schema: Option<&str>) -> String {
        format!("DROP TABLE IF EXISTS {}[{}]", quoted_prefix!(schema), name)
    }

    fn rename_table(old: &str, new: &str, schema: Option<&str>) -> String {
        let prefix = prefix!(schema);
        format!(r#"EXEC sp_rename '{}{}', '{}'"#, prefix, old, new)
    }

    fn alter_table(name: &str, schema: Option<&str>) -> String {
        format!("ALTER TABLE {}[{}]", quoted_prefix!(schema), name)
    }

    fn add_column(ex: bool, schema: Option<&str>, name: &str, tt: &Type) -> String {
        let bt: BaseType = tt.get_inner();

        let name_and_type = match bt {
            BaseType::Array(_) => panic!("Arrays are not supported with SQL Server"),
            BaseType::Index(_) => unreachable!("Indices are handled via custom builder"),
            _ => format!(
                "{}[{}] {}",
                MsSql::prefix(ex),
                name,
                MsSql::print_type(bt, schema)
            ),
        };

        let primary_key_indicator = match tt.primary {
            true => " PRIMARY KEY",
            false => "",
        };

        let default_indicator = match (&tt.default).as_ref() {
            Some(ref m) => Self::default(m),
            _ => format!(""),
        };

        let nullable_indicator = match tt.nullable {
            true => "",
            false => " NOT NULL",
        };

        let unique_indicator = match tt.unique {
            true => " UNIQUE",
            false => "",
        };

        format!(
            "{}{}{}{}{}",
            // `ADD name VARCHAR(max)` or `name VARCHAR(max)`
            name_and_type,
            // `PRIMARY KEY` or nothing
            primary_key_indicator,
            // `DEFAULT 'foo'` or nothing
            default_indicator,
            // `NOT NULL` or nothing
            nullable_indicator,
            // `UNIQUE or nothing`
            unique_indicator
        )
    }

    fn drop_column(name: &str) -> String {
        format!("DROP COLUMN [{}]", name)
    }

    fn rename_column(old: &str, new: &str) -> String {
        format!(r#"EXEC sp_rename '{}', '{}'"#, old, new)
    }

    fn create_index(table: &str, schema: Option<&str>, name: &str, _type: &Type) -> String {
        // FIXME: Implement PG specific index builder here
        format!(
            "CREATE {} INDEX [{}] ON {}[{}] ({})",
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
                    .map(|col| format!("[{}]", col))
                    .collect::<Vec<_>>()
                    .join(", "),
                _ => unreachable!(),
            }
        )
    }

    fn drop_index(name: &str) -> String {
        format!("DROP INDEX [{}]", name)
    }

    fn add_foreign_key(
        columns: &[String],
        table: &str,
        relation_columns: &[String],
        schema: Option<&str>,
    ) -> String {
        let columns: Vec<_> = columns.into_iter().map(|c| format!("[{}]", c)).collect();
        let relation_columns: Vec<_> = relation_columns
            .into_iter()
            .map(|c| format!("[{}]", c))
            .collect();

        format!(
            "FOREIGN KEY({}) REFERENCES {}[{}]({})",
            columns.join(","),
            prefix!(schema),
            table,
            relation_columns.join(","),
        )
    }

    fn add_primary_key(columns: &[String]) -> String {
        let columns: Vec<_> = columns.into_iter().map(|c| format!("[{}]", c)).collect();
        format!("PRIMARY KEY ({})", columns.join(","))
    }
}

impl MsSql {
    fn prefix(ex: bool) -> String {
        match ex {
            true => format!("ADD "),
            false => format!(""),
        }
    }

    fn print_type(t: BaseType, schema: Option<&str>) -> String {
        use self::BaseType::*;
        match t {
            Text => format!("TEXT"),
            Varchar(l) => match l {
                0 => format!("VARCHAR(MAX)"), // For "0" remove the limit
                _ => format!("VARCHAR({})", l),
            },
            Char(l) => format!("CHAR({})", l),
            /* "NOT NULL" is added here because normally primary keys are implicitly not-null */
            Primary => format!("INT IDENTITY(1,1) PRIMARY KEY NOT NULL"),
            Integer => format!("INT"),
            Serial => format!("INT IDENTITY(1,1)"),
            Float => format!("FLOAT(24)"),
            Double => format!("FLOAT(53)"),
            UUID => format!("UNIQUEIDENTIFIER"),
            Boolean => format!("BIT"),
            Date => format!("DATE"),
            Time => format!("TIME"),
            DateTime => format!("DATETIME2"),
            Json => format!("JSON"),
            Binary => format!("VARBINARY(MAX)"),
            Foreign(s, t, refs) => format!(
                "INT REFERENCES {}[{}]({})",
                quoted_prefix!(s.or(schema.map(|s| s.into()))),
                t,
                refs.0
                    .iter()
                    .map(|r| format!("[{}]", r))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Custom(t) => format!("{}", t),
            Array(meh) => format!("{}[]", MsSql::print_type(*meh, schema)),
            Index(_) => unreachable!("Indices are handled via custom builder"),
        }
    }
}
