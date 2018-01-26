//! Table builder API module
//! 
//! A table in this module represents directly a table in an SQL database. Each
//! table is initialised (for you) with a backend which corresponds to the one
//! selected for your schema in general.
//! 
//! You also don't explicitly have to call `exec` which is (again) done for you
//! by the `schema::exec()` function.
//! 
//! 

use generators::{DatabaseGenerator, TableGenerator};

#[derive(Clone, PartialEq, Eq)]
pub struct Table<T: DatabaseGenerator + TableGenerator + Default>(T, String, Vec<String>);

impl<T: DatabaseGenerator + TableGenerator + Default> Table<T> {
    
    pub fn new(name: &str) -> Self {
        return Table(Default::default(), String::from(name), Vec::new());
    }

    pub fn get_name(&self) -> &String {
        return &self.1;
    }

    pub fn get_items(&self) -> &Vec<String> {
        return &self.2;
    }

    pub fn exec(&self) -> String {
        let l = self.2.len();
        if l == 1 {
            return self.2[0].clone();
        }

        /* Otherwise, build multi-instruction table */
        let mut cmd = String::from("(");
        let mut ctr = 0;

        for item in &self.2 {
            cmd.push_str(item);
            ctr += 1;
            if ctr < l {
                cmd.push_str(", ");
            }
        }

        cmd.push(')');
        return cmd;
    }

    /// Drop an existing column from the table
    pub fn drop_column(&mut self, name: &str) {

    }

    /// Rename an existing column
    pub fn rename_column(&mut self, old: &str, new: &str) {

    }

    /// Adds a primary key called id, that auto increments
    pub fn increments(&mut self) {
        self.2.push("\"id\" serial primary key".to_owned());
    }

    /// Add an integer column
    pub fn integer(&mut self, name: &str) {

    }
    
    /// Add a text column
    pub fn text(&mut self, name: &str) {

    }
    
    /// Add a string column
    pub fn string(&mut self, name: &str) {

    }
    
    /// Add a timestamp column
    pub fn timestamp(&mut self, name: &str) {

    }

}