//! Testing table manipulation functions

use table::*;

#[test]
fn increment() {
    let mut t = Table::new("users");
    t.increments();
    assert_eq!("\"id\" serial primary key", &t.exec());
}