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


// impl TableGenerator for PGSQL {

//     fn dropColumn<T>(&mut self, name: &str) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn renameColumn<T>(&mut self, old: &str, new: &str) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn increments<T>(&mut self) -> TableGenerated<T> {

//         let tg: TableGenerated<String> = TableGenerated {
//             raw: "$name serial primary key".to_owned(),
//             name: "id".to_owned(),
//             val: "".to_owned(),
//         };

//         unimplemented!();
//     }

//     fn integer<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn bigInteger<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn text<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn string<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn float<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn decimal<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn boolean<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn date<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn dateTime<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn time<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn timestamp<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn timestamps<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn dropTimestamps<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn binary<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn enumerable<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn json<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn jsonb<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn uuid<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn comment<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn engine<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn charset<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn collate<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn inherits<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn specificType<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn index<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn dropIndex<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn unique<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn foreign<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn dropForeign<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn dropUnique<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }

//     fn dropPrimary<T>(&mut self) -> TableGenerated<T> {
//         unimplemented!();
//     }
// }
