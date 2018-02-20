//! 

use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::prelude::*;
use std::fs::*;
use std::fs;


pub fn migration_from(path: &Path) -> Option<(String, String)> {
    let file = path.join("mod.rs");

    let containing = Path::new(path.to_str().unwrap());
    generate_cargo_proj(&containing, &file);

    let output = if cfg!(target_os = "windows") {
        Command::new("cargo")
                .current_dir(path.join("tmp"))
                .arg("run")
                .output()
                .expect("failed to execute cargo!")
    } else {
        Command::new("sh")
                .current_dir(path.join("tmp"))
                .arg("-c")
                .arg("cargo run")
                .output()
                .expect("failed to execute cargo!")
    };

    let output = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", output);
    let migrations: Vec<&str> = output.split("\n").collect();

    // fs::remove_dir_all(path.join("tmp")).unwrap();

    let up = String::from(migrations[0]);
    let down = String::from(migrations[0]);
    return Some((up, down));
}


fn generate_cargo_proj(path: &Path, migration: &Path) {
    fs::create_dir_all(&path.join("tmp").join("src")).unwrap();

    let ct = path.join("tmp").join("Cargo.toml");
    println!("{:?}", ct);
    let mut cargo_toml = File::create(&ct).unwrap();
    cargo_toml.write_all(b"
[package]
name = \"tmp-generator\"
description = \"Doing nasty things with cargo\"
version = \"0.0.0\"
authors = [\"Katharina Fey <kookie@spacekookie.de>\"]

# TODO: Use same `barrel` dependency as crate
[dependencies]
barrel = { git = \"https://github.com/spacekookie/barrel\" }").unwrap();

    let src_folder = &path.join("tmp").join("src").join("main.rs");
    fs::copy(migration, src_folder).unwrap();
}