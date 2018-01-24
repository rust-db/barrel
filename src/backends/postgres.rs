//! The postgresql SQL generator backend

use traits::*;

/// An SQL generator for PGSQL flavoured SQL
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