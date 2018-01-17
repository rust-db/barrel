//! Schema builder API
//! 

#[derive(Clone, PartialEq, Eq)]
pub struct Table {
    pub name: String,
    pub items: Vec<String>,
}


impl Table {
    
    pub fn new(name: &str) -> Table {
        return Table {
            name: String::from(name),
            items: Vec::new(),
        };
    }

    pub fn exec(&self) -> String {
        let l = self.items.len();
        if l == 1 {
            return self.items[0].clone();
        }

        /* Otherwise, build multi-instruction table */
        let mut cmd = String::from("(");
        let mut ctr = 0;

        for item in &self.items {
            cmd.push_str(item);
            ctr += 1;
            if ctr < l {
                cmd.push_str(", ");
            }
        }

        cmd.push(')');
        return cmd;
    }

    /// Adds a primary key called id, that auto increments
    pub fn increments(&mut self) {
        self.items.push("\"id\" serial primary key".to_owned());
    }

    pub fn string(&mut self, s: &str) {

    }

    pub fn timestamps(&mut self) {

    }
}