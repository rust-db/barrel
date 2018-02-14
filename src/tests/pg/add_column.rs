//! All add_column combinations for pgsql

use backend::{SqlGenerator, Pg};
use Type::*;


#[test]
fn text() {
    let sql = Pg::add_column(true, "Text", &Text);
    assert_eq!(String::from("ADD COLUMN \"Text\" TEXT"), sql);
}

#[test]
fn varchar() {
    let sql = Pg::add_column(true, "Varchar", &Varchar(255));
    assert_eq!(String::from("ADD COLUMN \"Varchar\" VARCHAR(255)"), sql);
}

#[test]
fn integer() {
    let sql = Pg::add_column(true, "Integer", &Integer);
    assert_eq!(String::from("ADD COLUMN \"Integer\" INTEGER"), sql);
}

#[test]
fn float() {
    let sql = Pg::add_column(true, "Float", &Float);
    assert_eq!(String::from("ADD COLUMN \"Float\" FLOAT"), sql);
}

#[test]
fn double() {
    let sql = Pg::add_column(true, "Double", &Double);
    assert_eq!(String::from("ADD COLUMN \"Double\" DOUBLE"), sql);
}

#[test]
fn boolean() {
    let sql = Pg::add_column(true, "Boolean", &Boolean);
    assert_eq!(String::from("ADD COLUMN \"Boolean\" BOOLEAN"), sql);
}

#[test]
fn binary() {
    let sql = Pg::add_column(true, "Binary", &Binary);
    assert_eq!(String::from("ADD COLUMN \"Binary\" BINARY"), sql);
}

#[test]
fn foreign() {
    let sql = Pg::add_column(true, "Foreign", &Foreign("posts"));
    assert_eq!(String::from("ADD COLUMN \"Foreign\" INTEGER REFERENCES posts"), sql);
}

#[test]
fn array_text() {
    let sql = Pg::add_column(true, "Array of Text", &Array(box Text));
    assert_eq!(String::from("ADD COLUMN \"Array of Text\" TEXT[]"), sql);
}

#[test]
fn array_varchar() {
    let sql = Pg::add_column(true, "Array of Varchar", &Array(box Varchar(255)));
    assert_eq!(String::from("ADD COLUMN \"Array of Varchar\" VARCHAR(255)[]"), sql);
}

#[test]
fn array_integer() {
    let sql = Pg::add_column(true, "Array of Integer", &Array(box Integer));
    assert_eq!(String::from("ADD COLUMN \"Array of Integer\" INTEGER[]"), sql);
}

#[test]
fn array_float() {
    let sql = Pg::add_column(true, "Array of Float", &Array(box Float));
    assert_eq!(String::from("ADD COLUMN \"Array of Float\" FLOAT[]"), sql);
}

#[test]
fn array_double() {
    let sql = Pg::add_column(true, "Array of Double", &Array(box Double));
    assert_eq!(String::from("ADD COLUMN \"Array of Double\" DOUBLE[]"), sql);
}

#[test]
fn array_boolean() {
    let sql = Pg::add_column(true, "Array of Boolean", &Array(box Boolean));
    assert_eq!(String::from("ADD COLUMN \"Array of Boolean\" BOOLEAN[]"), sql);
}

#[test]
fn array_binary() {
    let sql = Pg::add_column(true, "Array of Binary", &Array(box Binary));
    assert_eq!(String::from("ADD COLUMN \"Array of Binary\" BINARY[]"), sql);
}

#[test]
fn array_array_integer() {
    let sql = Pg::add_column(true, "Array of Array of Integer", &Array(box Array(box Integer)));
    assert_eq!(String::from("ADD COLUMN \"Array of Array of Integer\" INTEGER[][]"), sql);
}
