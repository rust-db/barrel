//! Database connection backends

#[cfg(feature = "default")]
mod diesel;

/// An error type that describes what went wrong when connection to a database
/// 
/// TODO: Add more types
pub enum ConnectionError {
    GenericError,
}


/// A very simple connection type that wraps around Diesel connections (and potentially more)
pub trait DbConnection {

    /// Create a new connection to a database
    fn new(route: &str) -> Self;

    /// Execute a bunch of SQL statements
    fn batch_exec(&mut self, sql: &str);
}


pub struct Connector<T: DbConnection>(T);
impl<T: DbConnection> Connector<T> {

    pub fn new(route: &str) -> Connector<T> {
        return Connector(T::new(route));
    }

    pub fn batch_exec(&mut self, sql: &str) {
        self.0.batch_exec(sql);
    }

}