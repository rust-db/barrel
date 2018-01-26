//!

#[test]
fn create_table_pgsql() {

    use schema::Schema;
    use table::Table;
    use generators::postgres::*;

    let mut sql = Schema::<PGSQL>::new();
    sql.create_table("users", |t: &mut Table<PGSQL>| {
        t.increments();
        t.string("username");
        t.integer("plushy_sharks_owned");

        t.timestamp("joined");
        t.timestamp("birthday");
    });

    let migration = sql.exec();
    assert_eq!("create table \"users\" (\"id\" serial primary key, \"username\" varchar(255), \"plushy_sharks_owned\" int, \"joined\" timestamptz, \"birthday\" timestamptz)", migration);
}