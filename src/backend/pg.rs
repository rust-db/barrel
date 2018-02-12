//!

use super::SqlGenerator;

pub struct Pg;
impl SqlGenerator for Pg {
    fn create_table(name: &str) -> String {
        return String::new();
    }

    fn create_table_if_not_exists(name: &str) -> String {
        return String::new();
    }

    fn drop_table(name: &str) -> String {
        return String::new();
    }

    fn drop_table_if_exists(name: &str) -> String {
        return String::new();
    }

    fn rename_table(old: &str, new: &str) -> String {
        return String::new();
    }

    fn alter_table(name: &str) -> String {
        return String::new();
    }

    fn drop_column(name: &str) -> String {
        return String::new();
    }

    fn rename_column(old: &str, new: &str) -> String {
        return String::new();
    }

    fn increments() -> String {
        return String::new();
    }

    fn integer(name: &str) -> String {
        return String::new();
    }

    fn text(name: &str) -> String {
        return String::new();
    }

    fn string(name: &str) -> String {
        return String::new();
    }

    fn timestamp(name: &str) -> String {
        return String::new();
    }
}
