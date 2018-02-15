//! 

use diesel_migrations::Migration;
use std::path::{Path, PathBuf};

pub fn migration_from(path: &Path) -> Option<Box<Migration>> {
    return None;
}