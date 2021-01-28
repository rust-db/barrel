//! A backend module which provides a few generic traits
//! to implement SQL generation for different databases.
//!
//! It also re-exports the generators for existing databases
//! so they can be used more conveniently.

#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::MySql;

#[cfg(feature = "pg")]
mod pg;
#[cfg(feature = "pg")]
pub use self::pg::Pg;

#[cfg(feature = "sqlite3")]
mod sqlite3;
#[cfg(feature = "sqlite3")]
pub use self::sqlite3::Sqlite;

#[cfg(feature = "mssql")]
mod mssql;
#[cfg(feature = "mssql")]
pub use self::mssql::MsSql;

#[allow(unused_imports)]
use crate::{types::Type, Migration};

/// An enum describing all supported Sql flavours
#[derive(Copy, Clone, Debug)]
pub enum SqlVariant {
    #[cfg(feature = "sqlite3")]
    Sqlite,
    #[cfg(feature = "pg")]
    Pg,
    #[cfg(feature = "mysql")]
    Mysql,
    #[cfg(feature = "mssql")]
    Mssql,
    #[doc(hidden)]
    __Empty,
}

impl SqlVariant {
    pub(crate) fn run_for(self, _migr: &Migration) -> String {
        match self {
            #[cfg(feature = "sqlite3")]
            SqlVariant::Sqlite => _migr.make::<Sqlite>(),

            #[cfg(feature = "pg")]
            SqlVariant::Pg => _migr.make::<Pg>(),

            #[cfg(feature = "mysql")]
            SqlVariant::Mysql => _migr.make::<MySql>(),

            #[cfg(feature = "mssql")]
            SqlVariant::Mssql => _migr.make::<MsSql>(),

            _ => panic!("You need to select an Sql variant!"),
        }
    }
}

/// A generic SQL generator trait
pub trait SqlGenerator {
    /// Create a new table with a name
    fn create_table(name: &str, schema: Option<&str>) -> String;

    /// Create a new table with a name, only if it doesn't exist
    fn create_table_if_not_exists(name: &str, schema: Option<&str>) -> String;

    /// Drop a table with a name
    fn drop_table(name: &str, schema: Option<&str>) -> String;

    /// Drop a table with a name, only if it exists
    fn drop_table_if_exists(name: &str, schema: Option<&str>) -> String;

    /// Rename a table from <old> to <new>
    fn rename_table(old: &str, new: &str, schema: Option<&str>) -> String;

    /// Modify a table in some other way
    fn alter_table(name: &str, schema: Option<&str>) -> String;

    /// Create a new column with a type
    fn add_column(ex: bool, schema: Option<&str>, name: &str, _type: &Type) -> String;

    /// Drop an existing column from the table
    fn drop_column(name: &str) -> String;

    /// Rename an existing column
    fn rename_column(old: &str, new: &str) -> String;

    /// Create a multi-column index
    fn create_index(table: &str, schema: Option<&str>, name: &str, _type: &Type) -> String;

    /// Create a constraint
    fn create_constraint(name: &str, _type: &Type) -> String;

    /// Create a multi-column index
    fn create_partial_index(
        table: &str,
        schema: Option<&str>,
        name: &str,
        _type: &Type,
        conditions: &str,
    ) -> String {
        format!(
            "{} WHERE {}",
            Self::create_index(table, schema, name, _type),
            conditions
        )
    }

    /// Drop a multi-column index
    fn drop_index(name: &str) -> String;

    /// Add a foreign key
    fn add_foreign_key(
        columns: &[String],
        table: &str,
        relation_columns: &[String],
        schema: Option<&str>,
    ) -> String;

    fn add_primary_key(columns: &[String]) -> String;
}
