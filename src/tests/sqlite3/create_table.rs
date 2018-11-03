//! Some unit tests that create create tables
#![allow(unused_imports)]

use backend::{SqlGenerator, Sqlite};
use {types, Migration, Table};

#[test]
fn create_multiple_tables() {
    let mut m = Migration::new();
    m.create_table("artist", |t| {
        t.add_column("name", types::text());
        t.add_column("description", types::text());
        t.add_column("pic", types::text());
        t.add_column("mbid", types::text());
    });
    m.create_table("album", |t| {
        t.add_column("name", types::text());
        t.add_column("pic", types::text());
        t.add_column("mbid", types::text());
    });
    assert_eq!(m.make::<Sqlite>(), String::from("CREATE TABLE \"artist\" (\"id\" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT  , \"name\" TEXT  , \"description\" TEXT  , \"pic\" TEXT  , \"mbid\" TEXT  );CREATE TABLE \"album\" (\"id\" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT  , \"name\" TEXT  , \"pic\" TEXT  , \"mbid\" TEXT  );"));
}
