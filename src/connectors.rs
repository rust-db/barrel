//! A module meant for library developers
//!
//! `barrel` can be used with different migration toolkits or
//! SQL adapters. You can either use it to just generate strings
//! or implemented the provided trait that will then automatically
//! execute the SQL string on your apropriate database backend.
//!
//! You can then simple call `Migration::execute` to run the provided
//! migration.

/// A generic trait that frameworks using barrel can implement
///
/// An object of this trait can be given to a `Migration` object to
/// automatically generate and run the given SQL string for a
/// database connection which is wrapped by it
pub trait SqlRunner {
    /// Execute the migration on a backend
    fn execute<S: Into<String>>(&mut self, sql: S);
}
