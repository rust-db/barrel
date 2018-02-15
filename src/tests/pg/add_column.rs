//! All add_column combinations for pgsql

use backend::{SqlGenerator, Pg};
use Type::*;
use Column;


#[test]
fn text() {
    let sql = Pg::add_column(true, "Text", &Column::new(Text));
    assert_eq!(String::from("ADD COLUMN \"Text\" TEXT"), sql);
}

#[test]
fn varchar() {
    let sql = Pg::add_column(true, "Varchar", &Column::new(Varchar(255)));
    assert_eq!(String::from("ADD COLUMN \"Varchar\" VARCHAR(255)"), sql);
}

#[test]
fn integer() {
    let sql = Pg::add_column(true, "Integer", &Column::new(Integer));
    assert_eq!(String::from("ADD COLUMN \"Integer\" INTEGER"), sql);
}

#[test]
fn float() {
    let sql = Pg::add_column(true, "Float", &Column::new(Float));
    assert_eq!(String::from("ADD COLUMN \"Float\" FLOAT"), sql);
}

#[test]
fn double() {
    let sql = Pg::add_column(true, "Double", &Column::new(Double));
    assert_eq!(String::from("ADD COLUMN \"Double\" DOUBLE"), sql);
}

#[test]
fn boolean() {
    let sql = Pg::add_column(true, "Boolean", &Column::new(Boolean));
    assert_eq!(String::from("ADD COLUMN \"Boolean\" BOOLEAN"), sql);
}

#[test]
fn binary() {
    let sql = Pg::add_column(true, "Binary", &Column::new(Binary));
    assert_eq!(String::from("ADD COLUMN \"Binary\" BINARY"), sql);
}

#[test]
fn foreign() {
    let sql = Pg::add_column(true, "Foreign", &Column::new(Foreign("posts")));
    assert_eq!(String::from("ADD COLUMN \"Foreign\" INTEGER REFERENCES posts"), sql);
}

#[test]
fn array_text() {
    let sql = Pg::add_column(true, "Array of Text", &Column::new(Array(box Text)));
    assert_eq!(String::from("ADD COLUMN \"Array of Text\" TEXT[]"), sql);
}

#[test]
fn array_varchar() {
    let sql = Pg::add_column(true, "Array of Varchar", &Column::new(Array(box Varchar(255))));
    assert_eq!(String::from("ADD COLUMN \"Array of Varchar\" VARCHAR(255)[]"), sql);
}

#[test]
fn array_integer() {
    let sql = Pg::add_column(true, "Array of Integer", &Column::new(Array(box Integer)));
    assert_eq!(String::from("ADD COLUMN \"Array of Integer\" INTEGER[]"), sql);
}

#[test]
fn array_float() {
    let sql = Pg::add_column(true, "Array of Float", &Column::new(Array(box Float)));
    assert_eq!(String::from("ADD COLUMN \"Array of Float\" FLOAT[]"), sql);
}

#[test]
fn array_double() {
    let sql = Pg::add_column(true, "Array of Double", &Column::new(Array(box Double)));
    assert_eq!(String::from("ADD COLUMN \"Array of Double\" DOUBLE[]"), sql);
}

#[test]
fn array_boolean() {
    let sql = Pg::add_column(true, "Array of Boolean", &Column::new(Array(box Boolean)));
    assert_eq!(String::from("ADD COLUMN \"Array of Boolean\" BOOLEAN[]"), sql);
}

#[test]
fn array_binary() {
    let sql = Pg::add_column(true, "Array of Binary", &Column::new(Array(box Binary)));
    assert_eq!(String::from("ADD COLUMN \"Array of Binary\" BINARY[]"), sql);
}

#[test]
fn array_array_integer() {
    let sql = Pg::add_column(true, "Array of Array of Integer", &Column::new(Array(box Array(box Integer))));
    assert_eq!(String::from("ADD COLUMN \"Array of Array of Integer\" INTEGER[][]"), sql);
}
