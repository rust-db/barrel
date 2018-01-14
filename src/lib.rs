//! Powerful schema builder API in Rust, using Diesel in the backend.
//! 
//! Barrel has two primary models, the schema and the table. A schema is built
//! with a variety of hooks that can be executed on tables, using static callbacks.
//! 
//! ```
//! let mut s = Schema::name("public").create_table("users", |t| {
//!     t.increments();
//! });
//! ```
//! 
//! The above code, for example, will create a new table in the "public" schema, called "users"
//! and then execute the table hook on it when invoking `schema.exec()`. The hook creates an
//! auto-incrementing primary intex. By default the name "id" is assumed.
//! 
//! Barrel is designed to give you ease of use as well as power over how you write your 
//! migrations and SQL schemas. See the `examples` folder for more details 🌈

mod table;
pub use table::*;

mod schema;
pub use schema::*;

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
