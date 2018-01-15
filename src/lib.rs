// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub mod table;
use table::Table;

pub mod schema;
use schema::Schema;


pub fn test() {

    let s = Schema::name("public").create_table("users", |t: &mut Table| {
        t.increments();
    });

    // create table "public"."users" ("id" serial primary key)

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
