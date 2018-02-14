//! Test Postgresql string generation with barrel

use backend::{SqlGenerator, Pg};
use Type::*;

#[test]
fn text() {
    let sql = Pg::add_column("Text", Text);
    assert_eq!(String::from("\"Text\" TEXT"), sql);
}
#[test]
fn varchar() {
    let sql = Pg::add_column("Varchar", Varchar(255));
    assert_eq!(String::from("\"Varchar\" VARCHAR(255)"), sql);
}
#[test]
fn integer() {
    let sql = Pg::add_column("Integer", Integer);
    assert_eq!(String::from("\"Integer\" INTEGER"), sql);
}
#[test]
fn float() {
    let sql = Pg::add_column("Float", Float);
    assert_eq!(String::from("\"Float\" FLOAT"), sql);
}
#[test]
fn double() {
    let sql = Pg::add_column("Double", Double);
    assert_eq!(String::from("\"Double\" DOUBLE"), sql);
}
#[test]
fn boolean() {
    let sql = Pg::add_column("Boolean", Boolean);
    assert_eq!(String::from("\"Boolean\" BOOLEAN"), sql);
}
#[test]
fn binary() {
    let sql = Pg::add_column("Binary", Binary);
    assert_eq!(String::from("\"Binary\" BINARY"), sql);
}
#[test]
fn foreign() {
    let sql = Pg::add_column("Foreign", Foreign("posts"));
    assert_eq!(String::from("\"Foreign\" INTEGER REFERENCES posts"), sql);
}
#[test]
fn array_text() {
    let sql = Pg::add_column("Array of Text", Array(box Text));
    assert_eq!(String::from("\"Array of Text\" TEXT[]"), sql);
}
#[test]
fn array_varchar() {
    let sql = Pg::add_column("Array of Varchar", Array(box Varchar(255)));
    assert_eq!(String::from("\"Array of Varchar\" VARCHAR(255)[]"), sql);
}
#[test]
fn array_integer() {
    let sql = Pg::add_column("Array of Integer", Array(box Integer));
    assert_eq!(String::from("\"Array of Integer\" INTEGER[]"), sql);
}
#[test]
fn array_float() {
    let sql = Pg::add_column("Array of Float", Array(box Float));
    assert_eq!(String::from("\"Array of Float\" FLOAT[]"), sql);
}
#[test]
fn array_double() {
    let sql = Pg::add_column("Array of Double", Array(box Double));
    assert_eq!(String::from("\"Array of Double\" DOUBLE[]"), sql);
}
#[test]
fn array_boolean() {
    let sql = Pg::add_column("Array of Boolean", Array(box Boolean));
    assert_eq!(String::from("\"Array of Boolean\" BOOLEAN[]"), sql);
}
#[test]
fn array_binary() {
    let sql = Pg::add_column("Array of Binary", Array(box Binary));
    assert_eq!(String::from("\"Array of Binary\" BINARY[]"), sql);
}
#[test]
fn array_foreign() {
    let sql = Pg::add_column("Array of Foreign", Array(box Foreign("posts")));
    assert_eq!(String::from("\"Array of Foreign\" INTEGER[] REFERENCES posts"), sql);
}
#[test]
fn array_array_integer() {
    let sql = Pg::add_column("Array of Array of Integer", Array(box Array(box Integer)));
    assert_eq!(String::from("\"Array of Array of Integer\" INTEGER[][]"), sql);
}
