//! Some unit tests that create create tables
#![allow(unused_imports)]

use crate::backend::{Pg, SqlGenerator};
use crate::{types, Migration, Table};

#[test]
fn simple_table() {
    let mut m = Migration::new();
    m.create_table("users", |_: &mut Table| {});
    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" ();")
    );
}

#[test]
fn create_table_if_not_exists_doesnt_hit_unreachable() {
    let mut m = Migration::new();
    m.create_table_if_not_exists("artist", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::text().nullable(true));
        t.add_column("description", types::text().nullable(true));
        t.add_column("pic", types::text().nullable(true));
        t.add_column("mbid", types::text().nullable(true));
    });
    assert_eq!(m.make::<Pg>(), String::from("CREATE TABLE \"artist\" IF NOT EXISTS (\"id\" SERIAL PRIMARY KEY NOT NULL, \"name\" TEXT, \"description\" TEXT, \"pic\" TEXT, \"mbid\" TEXT);"));
}

#[test]
fn basic_fields() {
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {
        t.add_column("id", types::primary());
        t.add_column("name", types::varchar(255));
        t.add_column("age", types::integer());
        t.add_column("plushy_sharks_owned", types::boolean());
    });

    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY NOT NULL, \"name\" VARCHAR(255) NOT NULL, \"age\" INTEGER NOT NULL, \"plushy_sharks_owned\" BOOLEAN NOT NULL);")
    );
}

// #[test]
// fn basic_fields_with_defaults() {
//     let mut m = Migration::new();
//     m.create_table("users", |t: &mut Table| {
//         t.add_column("name", types::varchar(255));
//         t.add_column("age", types::integer());
//         t.add_column("plushy_sharks_owned", types::boolean()); // nobody is allowed plushy sharks
//     });

//     assert_eq!(
//         m.make::<Pg>(),
//         String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY NOT NULL, \"name\" VARCHAR(255) DEFAULT 'Anonymous' NOT NULL, \"age\" INTEGER DEFAULT '100' NOT NULL, \"plushy_sharks_owned\" BOOLEAN DEFAULT 'f' NOT NULL);")
//     );
// }

#[test]
fn basic_fields_nullable() {
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {
        t.add_column("id", types::primary());
        t.add_column("name", types::varchar(255).nullable(true));
        t.add_column("age", types::integer().nullable(true));
        t.add_column("plushy_sharks_owned", types::boolean().nullable(true));
    });

    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY NOT NULL, \"name\" VARCHAR(255), \"age\" INTEGER, \"plushy_sharks_owned\" BOOLEAN);")
    );
}

// #[test]// fn simple_foreign_fields() {
//     let mut m = Migration::new();
//     m.create_table("users", |t: &mut Table| {
        // t.add_column("id", types::primary());
//         t.add_column("posts", types::foreign("poststypes::"));
//         ()
//     });

//     assert_eq!(
//         m.make::<Pg>(),
//         String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY NOT NULL, \"posts\" INTEGER REFERENCES posts NOT NULL);")
//     );
// }

#[test]
fn create_multiple_tables() {
    let mut m = Migration::new();
    m.create_table("artist", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::text());
        t.add_column("description", types::text());
        t.add_column("pic", types::text());
        t.add_column("mbid", types::text());
    });
    m.create_table("album", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::text());
        t.add_column("pic", types::text());
        t.add_column("mbid", types::text());
    });
    assert_eq!(m.make::<Pg>(), String::from("CREATE TABLE \"artist\" (\"id\" SERIAL PRIMARY KEY NOT NULL, \"name\" TEXT NOT NULL, \"description\" TEXT NOT NULL, \"pic\" TEXT NOT NULL, \"mbid\" TEXT NOT NULL);CREATE TABLE \"album\" (\"id\" SERIAL PRIMARY KEY NOT NULL, \"name\" TEXT NOT NULL, \"pic\" TEXT NOT NULL, \"mbid\" TEXT NOT NULL);"));
}

#[test]
fn drop_table() {
    let mut m = Migration::new();
    m.drop_table("users");

    assert_eq!(m.make::<Pg>(), String::from("DROP TABLE \"users\";"));
}

#[test]
fn drop_table_if_exists() {
    let mut m = Migration::new();
    m.drop_table_if_exists("users");

    assert_eq!(
        m.make::<Pg>(),
        String::from("DROP TABLE IF EXISTS \"users\";")
    );
}

#[test]
fn rename_table() {
    let mut m = Migration::new();
    m.rename_table("users", "cool_users");
    assert_eq!(
        m.make::<Pg>(),
        String::from("ALTER TABLE \"users\" RENAME TO \"cool_users\";")
    );
}
