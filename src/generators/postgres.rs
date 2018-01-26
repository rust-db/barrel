//! The postgresql SQL generator backend


use generators::{DatabaseGenerator, TableGenerator};

/// An SQL generator for PGSQL flavoured SQL
#[derive(Default)]
pub struct PGSQL {}

impl PGSQL {
    pub fn new() -> Self {
        return PGSQL {};
    }
}


/// This block implements 
impl DatabaseGenerator for PGSQL {
    
    fn create_table(name: &str) -> String {
        return format!("create table {}", name);
    }
    
    fn create_table_if_not_exists(name: &str) -> String {
        return format!("create table if not exists {}", name);
    }
    
    fn drop_table(name: &str) -> String {
        return format!("drop table {}", name);
    }
    
    fn drop_table_if_exists(name: &str) -> String {
        return format!("drop table if exists {}", name);
    }
    
    fn rename_table(old: &str, new: &str) -> String {
        return format!("alter table {} rename to {}", old, new);
    }
    
    fn modify_table(name: &str) -> String {
        return format!("alter table {}", name);
    }
}


impl TableGenerator for PGSQL {

    /// Drop an existing column from the table
    fn drop_column(&mut self, name: &str) -> String {
        return format!("drop column \"{}\"", name);
    }

    /// Rename an existing column
    fn rename_column(&mut self, old: &str, new: &str) -> String {
        return format!("rename column \"{}\" to \"{}\"", old, new);
    }

    /// Add an auto-incrementing primary key
    fn increments(&mut self) -> String {
        return format!("\"id\" serial primary key");
    }
    
    /// Add an integer column
    fn integer(&mut self, name: &str) -> String {
        return format!("\"{}\" int", name);
    }
    
    /// Add a text column
    fn text(&mut self, name: &str) -> String {
        return format!("\"{}\" text", name);
    }
    
    /// Add a string column
    fn string(&mut self, name: &str) -> String {
        let limit = 255; // FIXME: make this easily configurable
        return format!("\"{}\" varchar({})", name, limit);
    }
    
    /// Add a timestamp column
    fn timestamp(&mut self, name: &str) -> String {
        return format!("\"{}\" timestamptz", name);
    }
}

