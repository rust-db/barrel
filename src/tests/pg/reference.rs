#![allow(unused_imports)]

use crate::backend::{Pg, SqlGenerator};
use crate::{types, Migration, Table};


#[test]
fn in_schema() {
    let sql = Pg::add_column(false, Some("schema"), "author", &types::foreign("users", "id"));

    assert_eq!(sql, "\"author\" INTEGER REFERENCES \"schema\".\"users\"(id) NOT NULL");
}

#[test]
fn ext_schema() {
    let sql = Pg::add_column(false, Some("schema"), "author", &types::foreign_schema("other_schema", "users", "id"));

    assert_eq!(sql, "\"author\" INTEGER REFERENCES \"other_schema\".\"users\"(id) NOT NULL");
}