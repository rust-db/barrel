//! Database agnostic schema builder module
//! 



// /// An SQL type for a column
// pub enum Type {
//     Text,
//     Integer,
//     Float,
//     Boolean,
//     Date,
//     Time,
//     Timestamp,
// }

// /*
// Foreign Key (Customer_SID) REFERENCES CUSTOMER(SID));
// */

// pub fn add_column(name: &str, t: Type) {

// }

// fn test() {

//     add_column("name", Type::Text);
//     // add_column("posts", Type::Foreign).references("blub");

// }



// struct Placeholder;

// struct Metadata<T> {
//     name: String,
//     unique: bool,
//     references: (String, Vec<String>),
//     primary_key: bool,
//     not_null: bool,
//     default: Option<T>,
//     // encoding: Maybe
// }

// struct Column<T> {
//     name: String,
//     nullable: bool,
//     default: Option<T>,
// }

// impl<T> Column<T> {
//     pub fn new(name: &str) -> Column<T> {
//         return Column {
//             name: String::from(name),
//             nullable: false,
//             default: None,
//         };
//     }

//     pub fn default(&mut self, data: T) {
//         self.default = Some(data);
//     }
// }

// #[derive(PartialEq)]
// enum ColType {
//     Text, Number
// }

// #[derive(PartialEq)]
// enum ColDefault {
//     Text(String), Number(i64)
// }

// struct Col {
//     name: String,
//     nullable: bool,
//     t: ColType,
//     default: Option<ColDefault>
// }


// impl From<&'static str> for ColDefault {
//     fn from(error: &'static str) -> Self {
//         return ColDefault::Text(error.into());
//     }
// }

// impl From<i64> for ColDefault {
//     fn from(error: i64) -> Self {
//         return ColDefault::Number(error);
//     }
// }


// impl Col {

//     pub fn new<S: Into<String>>(name: S, t: ColType) -> Col {
//         return Col {
//             name: name.into(),
//             nullable: false,
//             t:t ,
//             default: None,
//         };
//     }

//     pub fn can_be_null(&mut self) {
//         self.nullable = true;
//     }

//     pub fn default<T: Into<ColDefault>>(&mut self, data: T) {
//         let def: ColDefault = data.into();
        
//         self.default = Some(def);
//     }
// }

// struct Tab {
//     name: String,
//     cols: Vec<Col>
// }

// impl Tab {
//     pub fn new<S: Into<String>>(name: S) -> Tab {
//         return Tab { name: name.into(), cols: Vec::new() };
//     }

//     pub fn add_column(&mut self, name: &str, t: ColType) -> &mut Col {
//         self.cols.push(Col::new(name, t));
//         let len = self.cols.len();
//         return &mut self.cols[len - 1];
//     }
// }


// #[allow(unused)]
// fn testt() {
//     use self::ColType::*;

//     let mut t = Tab::new("users");
//     t.add_column("name", Text);
//     t.add_column("gender", Text).default("octopus");
//     t.add_column("age", Number).default(666);
//     t.add_column("plushy_sharks_owned", Number).can_be_null();

// }


// enum Change {
    
//     /* Table changes */
//     AddColumn(String, Type),
//     ChangeColumn(String, Type, ()),
//     RenameColumn(String, String),
//     RemoveColumn(String),

//     /* Database changes */
//     CreateTable(String, Placeholder),
//     RenameTable(String, String),
//     DropTable(String),
// }

// /*

// add_column
// add_foreign_key
// add_index
// add_reference
// add_timestamps
// change_column
// change_column_default (must supply a :from and :to option)
// change_column_null
// create_join_table
// create_table
// disable_extension
// drop_join_table
// drop_table (must supply a block)
// enable_extension
// remove_column (must supply a type)
// remove_columns (must specify at least one column name or more)
// remove_foreign_key (must supply a second table)
// remove_index
// remove_reference
// remove_timestamps
// rename_column
// rename_index
// rename_table

// */

// /// Represents an action done on a schema
// #[derive(Clone)]
// #[allow(unused)]
// enum ChangeType {
//     CreateTable,
//     CreateTableIfNotExists,
//     RenameTable,
//     DropTable,
//     DropTableIfExists,
//     AlterTable,
//     Raw,
// }
// use self::ChangeType::*;

// /// A schema migration generator
// /// 
// /// Takes a generic argument that then is used to select the database backend.
// pub struct Schema<T: DatabaseGenerator + TableGenerator + Default>(T, Vec<(ChangeType, Table<T>, Box<Fn(&mut Table<T>)>)>);
// impl<T: DatabaseGenerator + TableGenerator + Default> Schema<T> {

//     /// Create a new Schema with a database backend type
//     /// 
//     /// Example
//     /// 
//     /// ```notest
//     /// Schema::<PGSQL>::new();
//     /// ```
//     pub fn new() -> Self {
//         return Schema(Default::default(), Vec::new());
//     }

//     /// Add a table to the schema with a callback
//     ///
//     /// The callback is provided with a mutable table that fields
//     /// can be worked on.
//     pub fn create_table<F: 'static>(&mut self, name: &str, cb: F)
//     where
//         F: Fn(&mut Table<T>),
//     {
//         let t = Table::new(name);
//         self.1.push((CreateTable, t, Box::new(cb)));
//     }

//     /// Only create a new table if one with the same name doesn't exist
//     ///
//     /// Provide a callback to manipulate the table. The callback
//     /// is lazy and will only be invoked when calling [[exec]]
//     pub fn create_table_if_not_exists<F: 'static>(&mut self, name: &str, cb: F)
//     where
//         F: Fn(&mut Table<T>),
//     {
//         let t = Table::new(name);
//         self.1.push((CreateTableIfNotExists, t, Box::new(cb)));
//     }

//     pub fn rename_table(&mut self, _: &str, _: &str) {
//         unimplemented!();
//     }

//     pub fn drop_table(&mut self, _: &str) {
//         unimplemented!();
//     }

//     pub fn drop_table_if_exists(&mut self, _: &str) {
//         unimplemented!();
//     }

//     /// use this function to manupulate a table
//     pub fn table<F: 'static>(&mut self, name: &str, cb: F)
//     where
//         F: Fn(&mut Table<T>),
//     {
//         let t = Table::new(name);
//         self.1.push((AlterTable, t, Box::new(cb)));
//     }


//     /// Executes all hooks and does magic
//     ///
//     /// Needs to be mutable because it morphs the hooks
//     pub fn exec(&mut self) -> String {
//         let mut s = String::new();

//         for pair in &mut self.1 {
//             let (mut table, hook) = (&mut pair.1, &pair.2);
//             hook(&mut table);
//             let table_name: &String = table.get_name();
//             let _type = pair.0.clone();

//             let cmd: String = match _type {
//                 CreateTable => T::create_table(table_name),
//                 AlterTable => T::alter_table(table_name),
//                 _ => String::from("COMMAND NOT SUPPORTED 😭"),
//             };

//             /* Add the command, some space, then the table contents */
//             s.push_str(&cmd);
//             s.push(' ');
//             s.push_str(&table.exec());
//         }

//         return s;
//     }
// }
