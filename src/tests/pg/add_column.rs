//! All add_column combinations for pgsql
#![allow(unused_imports)]

use crate::backend::{Pg, SqlGenerator};
use crate::types;

#[test]
fn text() {
    let sql = Pg::add_column(true, None, "Text", &types::text());
    assert_eq!(String::from("ADD COLUMN \"Text\" TEXT NOT NULL"), sql);
}

#[test]
fn varchar() {
    let sql = Pg::add_column(true, None, "Varchar", &types::varchar(255));
    assert_eq!(
        String::from("ADD COLUMN \"Varchar\" VARCHAR(255) NOT NULL"),
        sql
    );
}

#[test]
fn integer() {
    let sql = Pg::add_column(true, None, "Integer", &types::integer());
    assert_eq!(String::from("ADD COLUMN \"Integer\" INTEGER NOT NULL"), sql);
}

#[test]
fn float() {
    let sql = Pg::add_column(true, None, "Float", &types::float());
    assert_eq!(String::from("ADD COLUMN \"Float\" FLOAT NOT NULL"), sql);
}

#[test]
fn double() {
    let sql = Pg::add_column(true, None, "Double", &types::double());
    assert_eq!(
        String::from("ADD COLUMN \"Double\" DOUBLE PRECISION NOT NULL"),
        sql
    );
}

#[test]
fn boolean() {
    let sql = Pg::add_column(true, None, "Boolean", &types::boolean());
    assert_eq!(String::from("ADD COLUMN \"Boolean\" BOOLEAN NOT NULL"), sql);
}

#[test]
fn binary() {
    let sql = Pg::add_column(true, None, "Binary", &types::binary());
    assert_eq!(String::from("ADD COLUMN \"Binary\" BYTEA NOT NULL"), sql);
}

#[test]
fn date() {
    let sql = Pg::add_column(true, None, "Date", &types::date());
    assert_eq!(String::from("ADD COLUMN \"Date\" DATE NOT NULL"), sql);
}

#[test]
fn foreign() {
    let sql = Pg::add_column(true, None, "Foreign", &types::foreign("posts", "id"));
    assert_eq!(
        String::from("ADD COLUMN \"Foreign\" INTEGER REFERENCES \"posts\"(id) NOT NULL"),
        sql
    );
}

#[test]
fn custom() {
    let sql = Pg::add_column(true, None, "Point", &types::custom("POINT"));
    assert_eq!(String::from("ADD COLUMN \"Point\" POINT NOT NULL"), sql);
}

#[test]
fn array_text() {
    let sql = Pg::add_column(true, None, "Array of Text", &types::array(&types::text()));
    assert_eq!(
        String::from("ADD COLUMN \"Array of Text\" TEXT[] NOT NULL"),
        sql
    );
}

#[test]
fn array_varchar() {
    let sql = Pg::add_column(
        true,
        None,
        "Array of Varchar",
        &types::array(&types::varchar(255)),
    );
    assert_eq!(
        String::from("ADD COLUMN \"Array of Varchar\" VARCHAR(255)[] NOT NULL"),
        sql
    );
}

#[test]
fn array_integer() {
    let sql = Pg::add_column(
        true,
        None,
        "Array of Integer",
        &types::array(&types::integer()),
    );
    assert_eq!(
        String::from("ADD COLUMN \"Array of Integer\" INTEGER[] NOT NULL"),
        sql
    );
}

#[test]
fn array_float() {
    let sql = Pg::add_column(true, None, "Array of Float", &types::array(&types::float()));
    assert_eq!(
        String::from("ADD COLUMN \"Array of Float\" FLOAT[] NOT NULL"),
        sql
    );
}

#[test]
fn array_double() {
    let sql = Pg::add_column(
        true,
        None,
        "Array of Double",
        &types::array(&types::double()),
    );
    assert_eq!(
        String::from("ADD COLUMN \"Array of Double\" DOUBLE PRECISION[] NOT NULL"),
        sql
    );
}

#[test]
fn array_boolean() {
    let sql = Pg::add_column(
        true,
        None,
        "Array of Boolean",
        &types::array(&types::boolean()),
    );
    assert_eq!(
        String::from("ADD COLUMN \"Array of Boolean\" BOOLEAN[] NOT NULL"),
        sql
    );
}

#[test]
fn array_binary() {
    let sql = Pg::add_column(
        true,
        None,
        "Array of Binary",
        &types::array(&types::binary()),
    );
    assert_eq!(
        String::from("ADD COLUMN \"Array of Binary\" BYTEA[] NOT NULL"),
        sql
    );
}

// #[test]
// fn array_custom() {
//     let sql = Pg::add_column(true, "Array of Point", &types::array(&types::custom("POINT")));
//     assert_eq!(
//         String::from("ADD COLUMN \"Array of Point\" POINT[] NOT NULL"),
//         sql
//     );
// }

#[test]
fn array_array_integer() {
    let sql = Pg::add_column(
        true,
        None,
        "Array of Array of Integer",
        &types::array(&types::array(&types::integer())),
    );
    assert_eq!(
        String::from("ADD COLUMN \"Array of Array of Integer\" INTEGER[][] NOT NULL"),
        sql
    );
}
