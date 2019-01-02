//! Other simple table/ column migrations

#![allow(unused_imports)]

use backend::{SqlGenerator, MySql};

#[test]
fn create_table() {
    let sql = MySql::create_table("table_to_create");
    assert_eq!(String::from("CREATE TABLE table_to_create"), sql);
}

#[test]
fn create_table_if_not_exists() {
    let sql = MySql::create_table_if_not_exists("table_to_create");
    assert_eq!(
        String::from("CREATE TABLE table_to_create IF NOT EXISTS"),
        sql
    );
}

#[test]
fn drop_table() {
    let sql = MySql::drop_table("table_to_drop");
    assert_eq!(String::from("DROP TABLE table_to_drop"), sql);
}

#[test]
fn drop_table_if_exists() {
    let sql = MySql::drop_table_if_exists("table_to_drop");
    assert_eq!(String::from("DROP TABLE table_to_drop IF EXISTS"), sql);
}

#[test]
fn rename_table() {
    let sql = MySql::rename_table("old_table", "new_table");
    assert_eq!(
        String::from("RENAME TABLE \"old_table\" TO \"new_table\""),
        sql
    );
}

#[test]
fn alter_table() {
    let sql = MySql::alter_table("table_to_alter");
    assert_eq!(String::from("ALTER TABLE \"table_to_alter\""), sql);
}
