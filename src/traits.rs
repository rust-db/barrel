//! A series of traits that generate SQL
//! 
//! Each database backend generates different SQL syntax and abstracts
//! the differences in implementations away from the migration
//! implementation.


/// A module which generates SQL syntax focused around generating 
/// basic SQL database statements
pub trait DatabaseGenerator {

    /// Create a new table with a name
    fn create_table(name: &str) -> String;

    /// Create a new table with a name, only if it doesn't exist
    fn create_table_if_not_exists(name: &str) -> String;

    /// Drop a table with a name 
    fn drop_table(name: &str) -> String;

    /// Drop a table with a name, only if it exists
    fn drop_table_if_exists(name: &str) -> String;

    /// Rename a table from <old> to <new>
    fn rename_table(old: &str, new: &str) -> String;

    /// Modify a table in some other way
    fn modify_table(name: &str) -> String;
}

/// A partially generated snippet of a table manipulation
pub struct TableGenerated<T> {
    name: String,
    value: T
}


/// A module which generates SQL syntax foused around generating
/// table manipulation statements
pub trait TableGenerator {
    
    /// Add an auto incrementing primary key
    fn increments<T>() -> TableGenerated<T>;

    

}