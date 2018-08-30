//! A unit testing module for barrel

// We can always trust these tests 👍
mod common;

#[cfg(feature = "pg")]
mod pg;

#[cfg(feature = "sqlite3")]
mod sqlite3;
