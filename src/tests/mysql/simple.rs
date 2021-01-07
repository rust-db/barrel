//! Other simple table/ column migrations

#![allow(unused_imports)]

use crate::backend::{MySql, SqlGenerator};

#[test]
fn create_table() {
    let sql = MySql::create_table("table_to_create", None);
    assert_eq!(String::from("CREATE TABLE `table_to_create`"), sql);
}

#[test]
fn create_table_with_schema() {
    let sql = MySql::create_table("table_to_create", Some("my_schema"));
    assert_eq!(String::from("CREATE TABLE `my_schema`.`table_to_create`"), sql);
}

#[test]
fn create_table_if_not_exists() {
    let sql = MySql::create_table_if_not_exists("table_to_create", None);
    assert_eq!(
        String::from("CREATE TABLE IF NOT EXISTS `table_to_create`"),
        sql
    );
}

#[test]
fn drop_table() {
    let sql = MySql::drop_table("table_to_drop", None);
    assert_eq!(String::from("DROP TABLE `table_to_drop`"), sql);
}

#[test]
fn drop_table_if_exists() {
    let sql = MySql::drop_table_if_exists("table_to_drop", None);
    assert_eq!(String::from("DROP TABLE IF EXISTS `table_to_drop`"), sql);
}

#[test]
fn rename_table() {
    let sql = MySql::rename_table("old_table", "new_table", None);
    assert_eq!(String::from("RENAME TABLE `old_table` TO `new_table`"), sql);
}

#[test]
fn alter_table() {
    let sql = MySql::alter_table("table_to_alter", None);
    assert_eq!(String::from("ALTER TABLE `table_to_alter`"), sql);
}
