//! Schema builder API
//!

use table::Table;

use generators::{DatabaseGenerator, TableGenerator};
use generators::postgres::PGSQL;

/// Represents an action done on a schema
enum ChangeType {
    CreateTable,
    CreateTableIfNotExists,
    RenameTable,
    DropTable,
    DropTableIfExists,
    AlterTable,
    Raw,
}
use self::ChangeType::*;

// 

/// A schema migration generator
pub struct Schema<T: DatabaseGenerator + TableGenerator + Default>(T, Vec<(ChangeType, Table<T>, Box<Fn(&mut Table<T>)>)>);
impl<T: DatabaseGenerator + TableGenerator + Default> Schema<T> {

    /// Create a new Schema with a database backend type
    /// 
    /// Example
    /// 
    /// ```
    /// Schema::<PGSQL>::new();
    /// ```
    pub fn new() -> Self {
        return Schema(Default::default(), Vec::new());
    }

    /// Add a table to the schema with a callback
    ///
    /// The callback is provided with a mutable table that fields
    /// can be worked on.
    pub fn create_table<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table<T>),
    {
        // let t = Table {
        //     name: String::from(name),
        //     items: Vec::new(),
        // };
        // self.1.push((CreateTable, t, Box::new(cb)));
    }

    /// Only create a new table if one with the same name doesn't exist
    ///
    /// Provide a callback to manipulate the table. The callback
    /// is lazy and will only be invoked when calling [[exec]]
    pub fn create_table_if_not_exists<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table<T>),
    {
        // create table if not exists
    }

    /// Rename a table into another
    pub fn rename_table(&mut self, old_name: &str, new_name: &str) {
        // alter table "users" rename to "old_users"
    }

    /// Drop a table
    pub fn drop_table(&mut self, name: &str) {
        // drop table "users"
    }

    /// Only drop a table if it exists
    pub fn drop_table_if_exists(&mut self, name: &str) {
        // drop table if exists "users"
    }

    /// use this function to manupulate a table
    pub fn table<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table<T>),
    {
        // alter table "users" add column "first_name" varchar(255), add column "last_name" varchar(255);
        // alter table "users" drop column "name"
    }


    /// Executes all hooks and does magic
    ///
    /// Needs to be mutable because it morphs the hooks
    pub fn exec(&mut self) -> String {
        let mut s = String::new();

        for pair in &mut self.1 {
            let (mut table, hook) = (&mut pair.1, &pair.2);
            hook(&mut table);
            let table_name: &String = table.get_name();

            s.push_str(&format!("create table {}", table_name));

            s.push(' ');
            s.push('(');
            for cmd in table.get_items() {
                s.push_str(cmd);
            }
            s.push(')');
        }

        return s;
    }
}
