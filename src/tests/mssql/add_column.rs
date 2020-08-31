//! All add_column combinations for pgsql
#![allow(unused_imports)]

use crate::backend::{MsSql, SqlGenerator};
use crate::types;

#[test]
fn text() {
    let sql = MsSql::add_column(true, None, "Text", &types::text());
    assert_eq!(String::from("ADD [Text] TEXT NOT NULL"), sql);
}

#[test]
fn varchar() {
    let sql = MsSql::add_column(true, None, "Varchar", &types::varchar(255));
    assert_eq!(String::from("ADD [Varchar] VARCHAR(255) NOT NULL"), sql);
}

#[test]
fn integer() {
    let sql = MsSql::add_column(true, None, "Integer", &types::integer());
    assert_eq!(String::from("ADD [Integer] INT NOT NULL"), sql);
}

#[test]
fn float() {
    let sql = MsSql::add_column(true, None, "Float", &types::float());
    assert_eq!(String::from("ADD [Float] FLOAT(24) NOT NULL"), sql);
}

#[test]
fn double() {
    let sql = MsSql::add_column(true, None, "Double", &types::double());
    assert_eq!(String::from("ADD [Double] FLOAT(53) NOT NULL"), sql);
}

#[test]
fn boolean() {
    let sql = MsSql::add_column(true, None, "Boolean", &types::boolean());
    assert_eq!(String::from("ADD [Boolean] BIT NOT NULL"), sql);
}

#[test]
fn binary() {
    let sql = MsSql::add_column(true, None, "Binary", &types::binary());
    assert_eq!(String::from("ADD [Binary] VARBINARY(MAX) NOT NULL"), sql);
}

#[test]
fn date() {
    let sql = MsSql::add_column(true, None, "Date", &types::date());
    assert_eq!(String::from("ADD [Date] DATE NOT NULL"), sql);
}

#[test]
fn foreign() {
    let sql = MsSql::add_column(true, None, "Foreign", &types::foreign("posts", "id"));
    assert_eq!(
        String::from("ADD [Foreign] INT REFERENCES [posts]([id]) NOT NULL"),
        sql
    );
}

#[test]
fn custom() {
    let sql = MsSql::add_column(true, None, "Xml", &types::custom("XML"));
    assert_eq!(String::from("ADD [Xml] XML NOT NULL"), sql);
}
