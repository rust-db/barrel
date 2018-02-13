//! Postgres implementation of a generator

use super::{SqlGenerator, Type};

pub struct Pg;
impl SqlGenerator for Pg {
    fn create_table(name: &str) -> String {
        return format!("CREATE TABLE {}", name);
    }

    fn create_table_if_not_exists(name: &str) -> String {
        return format!("CREATE TABLE {} IF NOT EXISTS", name);
    }

    fn drop_table(name: &str) -> String {
        return format!("DROP TABLE {}", name);
    }

    fn drop_table_if_exists(name: &str) -> String {
        return format!("DROP TABLE {} IF EXISTS", name);
    }

    fn rename_table(old: &str, new: &str) -> String {
        return format!("ALTER TABLE {} RENAME TO {}", old, new);
    }

    fn alter_table(name: &str) -> String {
        return format!("ALTER TABLE {}", name);
    }

    fn add_column(name: &str, _type: Type) -> String {
        use Type::*;
        return match _type {
            Text => format!("TEXT"),
            _ => unreachable!()
        };
    }

    fn drop_column(name: &str) -> String {
        return format!("DROP COLUMN {}", name);
    }

    fn rename_column(old: &str, new: &str) -> String {
        return format!("ALTER COLUMN {} RENAME TO {}", old, new);
    }
}
