//! A series of traits that generate SQL
//! 
//! Each database backend generates different SQL syntax and abstracts
//! the differences in implementations away from the migration
//! implementation.


/// A module which generates SQL syntax focused around generating 
/// basic SQL database statements
pub trait DatabaseGenerator {

    /// Create a new table with a name
    fn create_table(name: &str) -> String;

    /// Create a new table with a name, only if it doesn't exist
    fn create_table_if_not_exists(name: &str) -> String;

    /// Drop a table with a name 
    fn drop_table(name: &str) -> String;

    /// Drop a table with a name, only if it exists
    fn drop_table_if_exists(name: &str) -> String;

    /// Rename a table from <old> to <new>
    fn rename_table(old: &str, new: &str) -> String;

    /// Modify a table in some other way
    fn modify_table(name: &str) -> String;
}

/// A partially generated snippet of a table manipulation
pub struct TableGenerated {
    pub name: String,
    // value: T
}


/// A module which generates SQL syntax foused around generating
/// table manipulation statements
pub trait TableGenerator {

    fn dropColumn(&mut self, name: &str) -> TableGenerated;
    fn renameColumn(&mut self, old: &str, new: &str) -> TableGenerated;
    fn increments(&mut self) -> TableGenerated;
    fn integer(&mut self) -> TableGenerated;
    fn bigInteger(&mut self) -> TableGenerated;
    fn text(&mut self) -> TableGenerated;
    fn string(&mut self) -> TableGenerated;
    fn float(&mut self) -> TableGenerated;
    fn decimal(&mut self) -> TableGenerated;
    fn boolean(&mut self) -> TableGenerated;
    fn date(&mut self) -> TableGenerated;
    fn dateTime(&mut self) -> TableGenerated;
    fn time(&mut self) -> TableGenerated;
    fn timestamp(&mut self) -> TableGenerated;
    fn timestamps(&mut self) -> TableGenerated;
    fn dropTimestamps(&mut self) -> TableGenerated;
    fn binary(&mut self) -> TableGenerated;
    fn enumerable(&mut self) -> TableGenerated;
    fn json(&mut self) -> TableGenerated;
    fn jsonb(&mut self) -> TableGenerated;
    fn uuid(&mut self) -> TableGenerated;
    fn comment(&mut self) -> TableGenerated;
    fn engine(&mut self) -> TableGenerated;
    fn charset(&mut self) -> TableGenerated;
    fn collate(&mut self) -> TableGenerated;
    fn inherits(&mut self) -> TableGenerated;
    fn specificType(&mut self) -> TableGenerated;
    fn index(&mut self) -> TableGenerated;
    fn dropIndex(&mut self) -> TableGenerated;
    fn unique(&mut self) -> TableGenerated;
    fn foreign(&mut self) -> TableGenerated;
    fn dropForeign(&mut self) -> TableGenerated;
    fn dropUnique(&mut self) -> TableGenerated;
    fn dropPrimary(&mut self) -> TableGenerated;

    

}