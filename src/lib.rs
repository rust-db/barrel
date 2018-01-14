// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub mod table;
pub use table::*;
use table::Table;

pub mod schema;
pub use schema::*;
use schema::Schema;

pub fn test() {

    let mut s = Schema::name("public").create_table("users", |t: &mut Table| {
        t.increments();
    });

    println!("{}", s.exec());
}


//
// – with
// – withSchema
// – createTable
// – createTableIfNotExists
// – renameTable
// – dropTable
// – hasColumn
// – hasTable
// – dropTableIfExists
// – table
// – raw
//
//
//
//
//
//
// – dropColumn
// – dropColumns
// – renameColumn
// – increments
// – integer
// – bigInteger
// – text
// – string
// – float
// – decimal
// – boolean
// – date
// – dateTime
// – time
// – timestamp
// – timestamps
// – dropTimestamps
// – binary
// – enum / enu
// – json
// – jsonb
// – uuid
// – comment
// – engine
// – charset
// – collate
// – inherits
// – specificType
// – index
// – dropIndex
// – unique
// – foreign
// – dropForeign
// – dropUnique
// – dropPrimary
