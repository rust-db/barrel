//! Schema builder API
//!

use table::*;


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

pub struct Schema {
    schema: Option<String>,
    jobs: Vec<(ChangeType, Table, Box<Fn(&mut Table)>)>,
}

#[derive(Debug)]
/// Simple status codes to communicate if something went wrong
/// while executing a schema migration
pub enum SchemaError {
    TableAlreadyExists,
    TableDoesNotExists,
}

impl Schema {
    /// Create a new schema
    pub fn new() -> Schema {
        return Schema {
            schema: None,
            jobs: Vec::new(),
        };
    }

    /// Specify a schema for a SQL command
    pub fn with_schema(mut self, s: &str) -> Schema {
        self.schema = Some(s.to_owned());
        return self;
    }

    /// Add a table to the schema with a callback
    ///
    /// The callback is provided with a mutable table that fields
    /// can be worked on.
    pub fn create_table<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table),
    {
        let t = Table {
            name: String::from(name),
            items: Vec::new(),
        };
        self.jobs.push((CreateTable, t, Box::new(cb)));
    }

    /// Only create a new table if one with the same name doesn't exist
    ///
    /// Provide a callback to manipulate the table. The callback
    /// is lazy and will only be invoked when calling [[exec]]
    pub fn create_table_if_not_exists<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table),
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
        F: Fn(&mut Table),
    {
        // alter table "users" add column "first_name" varchar(255), add column "last_name" varchar(255);
        // alter table "users" drop column "name"
    }

    /// Provide raw SQL that will be executed
    ///
    /// **Experimental** and **Undocumented**
    ///
    /// So please be careful
    pub fn raw<F: 'static>(&mut self, sql: &str) {}

    /// Executes all hooks and does magic
    ///
    /// Needs to be mutable because it morphs the hooks
    pub fn exec(&mut self) -> String {
        let mut s = String::new();

        for pair in &mut self.jobs {
            let (mut table, hook) = (&mut pair.1, &pair.2);
            let schema: &String = self.schema.as_ref().unwrap();
            hook(&mut table);
            let table_name: &String = &table.name;

            s.push_str("create table ");
            s.push_str(&format!("\"{}\".\"{}\"", schema, table_name));

            s.push(' ');
            s.push('(');
            for cmd in &table.items {
                s.push_str(cmd);
            }
            s.push(')');
        }

        return s;
    }
}

/*
# withSchema
knex.schema.withSchema('public').createTable('users', function (table) {
  table.increments();
})
Outputs:
create table "public"."users" ("id" serial primary key)
    
# createTable

knex.schema.createTable('users', function (table) {
  table.increments();
  table.string('name');
  table.timestamps();
})
Outputs:
create table "users" ("id" serial primary key, "name" varchar(255), 
"created_at" timestamptz, "updated_at" timestamptz)


# createTableIfNotExists

knex.schema.createTableIfNotExists('users', function (table) {
  table.increments();
  table.string('name');
  table.timestamps();
})
Outputs:
create table if not exists "users" ("id" serial primary key, "name" varchar(255), 
"created_at" timestamptz, "updated_at" timestamptz)


# renameTable

knex.schema.renameTable('users', 'old_users')
Outputs:
alter table "users" rename to "old_users"


# dropTable

knex.schema.dropTable('users')
Outputs:
drop table "users"


# TODO: hasColumn
# TODO: hasTable


# dropTableIfExists

knex.schema.dropTableIfExists('users')
Outputs:
drop table if exists "users"


# table

knex.schema.table('users', function (table) {
  table.dropColumn('name');
  table.string('first_name');
  table.string('last_name');
})
Outputs:
alter table "users" add column "first_name" varchar(255), 
add column "last_name" varchar(255);
alter table "users" drop column "name"

# raw
*/
