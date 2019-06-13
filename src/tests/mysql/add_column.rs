//! All add_column combinations for mysql
#![allow(unused_imports)]

use crate::backend::{MySql, SqlGenerator};
use crate::types;

#[test]
fn text() {
    let sql = MySql::add_column(true, "Text", &types::text());
    assert_eq!(String::from("ADD COLUMN Text TEXT NOT NULL"), sql);
}

#[test]
fn varchar() {
    let sql = MySql::add_column(true, "Varchar", &types::varchar(255));
    assert_eq!(
        String::from("ADD COLUMN Varchar VARCHAR(255) NOT NULL"),
        sql
    );
}

#[test]
fn integer() {
    let sql = MySql::add_column(true, "Integer", &types::integer());
    assert_eq!(String::from("ADD COLUMN Integer INTEGER NOT NULL"), sql);
}

#[test]
fn float() {
    let sql = MySql::add_column(true, "Float", &types::float());
    assert_eq!(String::from("ADD COLUMN Float FLOAT NOT NULL"), sql);
}

#[test]
fn double() {
    let sql = MySql::add_column(true, "Double", &types::double());
    assert_eq!(String::from("ADD COLUMN Double DOUBLE NOT NULL"), sql);
}

#[test]
fn boolean() {
    let sql = MySql::add_column(true, "Boolean", &types::boolean());
    assert_eq!(String::from("ADD COLUMN Boolean BOOLEAN NOT NULL"), sql);
}

#[test]
fn binary() {
    let sql = MySql::add_column(true, "Binary", &types::binary());
    assert_eq!(String::from("ADD COLUMN Binary BYTEA NOT NULL"), sql);
}

#[test]
fn date() {
    let sql = MySql::add_column(true, "Date", &types::date());
    assert_eq!(String::from("ADD COLUMN Date DATE NOT NULL"), sql);
}

#[test]
fn foreign() {
    let sql = MySql::add_column(true, "Foreign", &types::foreign("posts", "id"));
    assert_eq!(
        String::from("ADD COLUMN Foreign INTEGER REFERENCES posts(id) NOT NULL"),
        sql
    );
}
