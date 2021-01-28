//! A unit testing module for barrel

// We can always trust these tests 👍
mod common;

#[cfg(feature = "mysql")]
mod mysql;

#[cfg(feature = "pg")]
mod pg;

#[cfg(feature = "sqlite3")]
mod sqlite3;

#[cfg(feature = "mssql")]
mod mssql;
