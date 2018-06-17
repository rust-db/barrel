//! Simple schema representation for migration state

trait Schemas {
    /// Returns the name of a database
    fn name(&self) -> String;

    /// Returns a list of all tables in a database
    fn tables(&self) -> Vec<String>;

    /// Returns a list of all column names and types
    fn columns(&self, table: &str) -> Vec<(String, Column)>;
}

trait Column {
    /// Get the type of column in SQL specific terms
    fn type(&self) -> String;
}

// Describe the current state of a database to apply a migration to
struct Schema {
    db_name: String,
    columns: Vec<String>,
}


impl Schema {

}

