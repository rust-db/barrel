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

impl TableGenerator for PGSQL {

    fn dropColumn(&mut self, name: &str) -> TableGenerated {
        unimplemented!();
    }

    fn renameColumn(&mut self, old: &str, new: &str) -> TableGenerated {
        unimplemented!();
    }

    fn increments(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn integer(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn bigInteger(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn text(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn string(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn float(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn decimal(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn boolean(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn date(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn dateTime(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn time(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn timestamp(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn timestamps(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn dropTimestamps(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn binary(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn enumerable(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn json(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn jsonb(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn uuid(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn comment(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn engine(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn charset(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn collate(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn inherits(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn specificType(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn index(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn dropIndex(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn unique(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn foreign(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn dropForeign(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn dropUnique(&mut self) -> TableGenerated {
        unimplemented!();
    }

    fn dropPrimary(&mut self) -> TableGenerated {
        unimplemented!();
    }


}