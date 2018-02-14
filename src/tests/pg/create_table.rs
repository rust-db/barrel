//! Some unit tests that create create tables

use backend::{Pg, SqlGenerator};
use {Migration, Table};

#[test]
fn simple_table() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("users", |t: &mut Table| {});
    assert_eq!(
        m.make::<Pg>(),
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY)")
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
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY, \"name\" VARCHAR(255), \"age\" INTEGER, \"plushy_sharks_owned\" BOOLEAN)")
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
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY, \"name\" VARCHAR(255) DEFAULT 'Anonymous', \"age\" INTEGER DEFAULT '100', \"plushy_sharks_owned\" BOOLEAN DEFAULT 'f')")
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
        String::from("CREATE TABLE \"users\" (\"id\" SERIAL PRIMARY KEY, \"posts\" INTEGER REFERENCES posts)")
    );
}

