use Migration;

#[test]
fn create_multiple_tables() {
    let mut migr = Migration::new();
    migr.create_table("foo", |_| {});
    migr.create_table("bar", |_| {});

    assert!(migr.changes.len() == 2);
}
