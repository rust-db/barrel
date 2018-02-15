//! Other simple table/ column migrations

use backend::{SqlGenerator, Pg};
use Type::*;


#[test]
fn create_table() {
    let sql = Pg::create_table("table_to_create");
    assert_eq!(String::from("CREATE TABLE \"table_to_create\""), sql);
}

#[test]
fn create_table_if_not_exists() {
    let sql = Pg::create_table_if_not_exists("table_to_create");
    assert_eq!(String::from("CREATE TABLE \"table_to_create\" IF NOT EXISTS"), sql);
}

#[test]
fn drop_table() {
    let sql = Pg::drop_table("table_to_drop");
    assert_eq!(String::from("DROP TABLE \"table_to_drop\""), sql);
}

#[test]
fn drop_table_if_exists() {
    let sql = Pg::drop_table_if_exists("table_to_drop");
    assert_eq!(String::from("DROP TABLE \"table_to_drop\" IF EXISTS"), sql);
}

#[test]
fn rename_table() {
    let sql = Pg::rename_table("old_table", "new_table");
    assert_eq!(String::from("ALTER TABLE \"old_table\" RENAME TO \"new_table\""), sql);
}

#[test]
fn alter_table() {
    let sql = Pg::alter_table("table_to_alter");
    assert_eq!(String::from("ALTER TABLE \"table_to_alter\""), sql);
}

#[test]
fn drop_column() {
    let sql = Pg::drop_column("column_to_drop");
    assert_eq!(String::from("DROP COLUMN \"column_to_drop\""), sql);
}

#[test]
fn rename_column() {
    let sql = Pg::rename_column("old_column", "new_column");
    assert_eq!(String::from("ALTER COLUMN \"old_column\" RENAME TO \"new_column\""), sql);
}
