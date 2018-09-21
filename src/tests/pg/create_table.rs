//! Some unit tests that create create tables
#![allow(unused_imports)]

use backend::{Pg, SqlGenerator};
use {Migration, Table};

#[test]
fn simple_table() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("users", |_: &mut Table| {});
    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY);")
    );
}

#[test]
fn basic_fields() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {
        t.add_column("name", Varchar(255));
        t.add_column("age", Integer);
        t.add_column("plushy_sharks_owned", Boolean);
    });

    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY, \"name\" VARCHAR(255), \"age\" INTEGER, \"plushy_sharks_owned\" BOOLEAN);")
    );
}

#[test]
fn basic_fields_with_defaults() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {
        t.add_column("name", Varchar(255)).default("Anonymous");
        t.add_column("age", Integer).default(100);
        t.add_column("plushy_sharks_owned", Boolean).default(false); // nobody is allowed plushy sharks
    });

    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY, \"name\" VARCHAR(255) DEFAULT 'Anonymous', \"age\" INTEGER DEFAULT '100', \"plushy_sharks_owned\" BOOLEAN DEFAULT 'f');")
    );
}

#[test]
fn simple_foreign_fields() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {
        t.add_column("posts", Foreign("posts"));
    });

    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY, \"posts\" INTEGER REFERENCES posts);")
    );
}

#[test]
fn create_multiple_tables() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("artist", |t| {
        t.add_column("name", Text);
        t.add_column("description", Text);
        t.add_column("pic", Text);
        t.add_column("mbid", Text);
    });
    m.create_table("album", |t| {
        t.add_column("name", Text);
        t.add_column("pic", Text);
        t.add_column("mbid", Text);
    });
    assert_eq!(m.make::<Pg>(), String::from("CREATE TABLE \"artist\" (\"id\" SERIAL PRIMARY KEY, \"name\" TEXT, \"description\" TEXT, \"pic\" TEXT, \"mbid\" TEXT);CREATE TABLE \"album\" (\"id\" SERIAL PRIMARY KEY, \"name\" TEXT, \"pic\" TEXT, \"mbid\" TEXT);"));
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

    assert_eq!(m.make::<Pg>(), String::from("DROP TABLE \"users\" IF EXISTS;"));
}

#[test]
fn rename_table() {
    let mut m = Migration::new();
    m.rename_table("users", "cool_users");
    assert_eq!(m.make::<Pg>(), String::from("ALTER TABLE \"users\" RENAME TO \"cool_users\";"));
}

// m.change_table("users", |t| {

// });
