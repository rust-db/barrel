//! Other simple table/ column migrations

#![allow(unused_imports)]

use crate::backend::{SqlGenerator, Sqlite};

#[test]
fn create_table() {
    let sql = Sqlite::create_table("table_to_create");
    assert_eq!(String::from("CREATE TABLE \"table_to_create\""), sql);
}

#[test]
fn create_table_if_not_exists() {
    let sql = Sqlite::create_table_if_not_exists("table_to_create");
    assert_eq!(
        String::from("CREATE TABLE IF NOT EXISTS \"table_to_create\""),
        sql
    );
}

#[test]
fn drop_table() {
    let sql = Sqlite::drop_table("table_to_drop");
    assert_eq!(String::from("DROP TABLE \"table_to_drop\""), sql);
}

#[test]
fn drop_table_if_exists() {
    let sql = Sqlite::drop_table_if_exists("table_to_drop");
    assert_eq!(String::from("DROP TABLE IF EXISTS \"table_to_drop\""), sql);
}

#[test]
fn rename_table() {
    let sql = Sqlite::rename_table("old_table", "new_table");
    assert_eq!(
        String::from("ALTER TABLE \"old_table\" RENAME TO \"new_table\""),
        sql
    );
}

#[test]
fn alter_table() {
    let sql = Sqlite::alter_table("table_to_alter");
    assert_eq!(String::from("ALTER TABLE \"table_to_alter\""), sql);
}
