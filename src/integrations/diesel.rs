//! 

use std::path::{Path, PathBuf};
use std::fs;
use std::fs::*;
use std::io::prelude::*;


pub fn migration_from(path: &Path) -> Option<String> {
    // TODO: Check if `.rs` file

    let containing = Path::new(path.to_str().unwrap());
    generate_cargo_proj(&containing, path);

    // ... TODO: Run migration ...

    return None;
}


fn generate_cargo_proj(path: &Path, migration: &Path) {
    path.join("tmp").join("src");
    fs::create_dir_all(&path).unwrap();

    let ct = path.parent().unwrap();
    ct.join("Cargo.toml");
    let mut cargo_toml = File::create(&ct).unwrap();
    cargo_toml.write_all(b"
[package]
name = \"tmp-generator\"
description = \"Pretends to be the barrel_diesel integrator\"
version = \"0.0.0\"
authors = [\"Katharina Fey <kookie@spacekookie.de>\"]

# TODO: Use same `barrel` dependency as crate
[dependencies]
barrel = { git = \"https://github.com/spacekookie/barrel\" }").unwrap();

    let src_folder = ct.parent().unwrap();
    src_folder.join("migration.rs");
    fs::copy(migration, src_folder).unwrap();
}