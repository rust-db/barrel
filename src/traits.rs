//! A series of traits that generate SQL
//! 
//! Each database backend generates different SQL syntax and abstracts
//! the differences in implementations away from the migration
//! implementation.


/// A module which generates SQL syntax focused around generating 
/// basic SQL database statements
pub trait DatabaseGenerator<T> {

    fn create_table(name: &str) -> TableGenerated<T>;

    fn create_table_if_not_exists(name: &str) -> TableGenerated<T>;

    fn drop_table(name: &str) -> TableGenerated<T>;

    fn drop_table_if_not_exists(name: &str) -> TableGenerated<T>;

    fn rename_table(old: &str, new: &str) -> TableGenerated<T>;

    fn raw_table(name: &str) -> TableGenerated<T>;
}

/// A partially generated snippet of a table manipulation
struct TableGenerated<T> {
    name: String,
    value: T
}


/// A module which generates SQL syntax foused around generating
/// table manipulation statements
trait TableGenerator {
    
    /// Add an auto incrementing primary key
    fn increments();

    

}