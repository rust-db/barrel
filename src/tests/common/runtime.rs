//! These tests check any kind of runtime-check behaviour
//!
//! They depend on all backends mostly for simplicity.

use crate::{Migration, types, SqlVariant};

/// This test mostly exists to see if we panic
#[test]
fn generate_from() {
    let mut m = Migration::new();
    m.create_table("testing", |table| {
        table.add_column("id", types::primary());
        table.add_column("name", types::varchar(64));
    });

    let _ = m.make_from(SqlVariant::Pg);
    let _ = m.make_from(SqlVariant::Mysql);
    let _ = m.make_from(SqlVariant::Sqlite);
}