//! Simple schema representation for migration state

trait Schemas {
    
}


/// Describe the current state of a database to apply a migration to
struct Schema {
    db_name: String,
    columns: Vec<String>,
}


impl Schema {

}


src/
  foo.rs
  foo/
    bar.rs