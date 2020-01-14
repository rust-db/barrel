//! Some unit tests that create create tables
#![allow(unused_imports)]

use crate::backend::{MySql, SqlGenerator};
use crate::{types, Migration, Table};

#[test]
fn create_multiple_tables() {
    let mut m = Migration::new();
    m.create_table("artist", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::text().nullable(true));
        t.add_column("description", types::text().nullable(true));
        t.add_column("pic", types::text().nullable(true));
        t.add_column("mbid", types::text().nullable(true));
    });
    m.create_table("album", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::text().nullable(true));
        t.add_column("pic", types::text().nullable(true));
        t.add_column("mbid", types::text().nullable(true));
    });
    assert_eq!(m.make::<MySql>(), String::from("CREATE TABLE `artist` (`id` INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY, `name` TEXT, `description` TEXT, `pic` TEXT, `mbid` TEXT);CREATE TABLE `album` (`id` INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY, `name` TEXT, `pic` TEXT, `mbid` TEXT);"));
}

#[test]
fn create_table_if_not_exists_doesnt_hit_unreachable() {
    let mut m = Migration::new();
    m.create_table_if_not_exists("artist", |t| {
        t.add_column("id", types::primary());
        t.add_column("name", types::text().nullable(true));
        t.add_column("description", types::text().nullable(true));
        t.add_column("pic", types::text().nullable(true));
        t.add_column("mbid", types::text().nullable(true));
    });
    assert_eq!(m.make::<MySql>(), String::from("CREATE TABLE `artist` IF NOT EXISTS (`id` INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY, `name` TEXT, `description` TEXT, `pic` TEXT, `mbid` TEXT);"));
}
