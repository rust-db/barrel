//! Other simple table/ column migrations
#![allow(unused_imports)]

use crate::backend::{MsSql, SqlGenerator};

#[test]
fn create_table() {
    let sql = MsSql::create_table("table_to_create", None);
    assert_eq!(String::from("CREATE TABLE [table_to_create]"), sql);
}

#[test]
fn create_table_with_schema() {
    let sql = MsSql::create_table("table_to_create", Some("my_schema"));
    assert_eq!(
        String::from("CREATE TABLE [my_schema].[table_to_create]"),
        sql
    );
}

#[test]
fn create_table_if_not_exists() {
    let sql = MsSql::create_table_if_not_exists("table_to_create", None);
    assert_eq!(String::from("IF NOT EXISTS (SELECT * FROM sys.tables WHERE name=\'table_to_create\') CREATE TABLE [table_to_create]"), sql);
}

#[test]
fn drop_table() {
    let sql = MsSql::drop_table("table_to_drop", None);
    assert_eq!(String::from("DROP TABLE [table_to_drop]"), sql);
}

#[test]
fn drop_table_if_exists() {
    let sql = MsSql::drop_table_if_exists("table_to_drop", None);
    assert_eq!(String::from("DROP TABLE IF EXISTS [table_to_drop]"), sql);
}

#[test]
fn rename_table() {
    let sql = MsSql::rename_table("old_table", "new_table", None);
    assert_eq!(String::from("EXEC sp_rename 'old_table', 'new_table'"), sql);
}

#[test]
fn alter_table() {
    let sql = MsSql::alter_table("table_to_alter", None);
    assert_eq!(String::from("ALTER TABLE [table_to_alter]"), sql);
}

#[test]
fn drop_column() {
    let sql = MsSql::drop_column("column_to_drop");
    assert_eq!(String::from("DROP COLUMN [column_to_drop]"), sql);
}

#[test]
fn rename_column() {
    let sql = MsSql::rename_column("table.old_column", "table.new_column");
    assert_eq!(
        String::from("EXEC sp_rename 'table.old_column', 'table.new_column'"),
        sql
    );
}
