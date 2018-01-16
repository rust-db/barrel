//! Schema builder API
//! 

#[derive(Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub items: Vec<String>,
}


impl Table {
    

    pub fn increments(&mut self) {
        self.items.push("\"id\" serial primary key".to_owned());
    }

    pub fn string(&mut self, s: &str) {

    }
    pub fn timestamps(&mut self) {

    }
}