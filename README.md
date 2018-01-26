![Barrel Logo](assets/logo_wide.svg)

[![Build Status](https://travis-ci.org/spacekookie/barrel.svg?branch=master)](https://travis-ci.org/spacekookie/barrel)   [![Coverage Status](https://coveralls.io/repos/github/spacekookie/barrel/badge.svg?branch=master)](https://coveralls.io/github/spacekookie/barrel?branch=master) [![Crates.io badge](https://img.shields.io/crates/v/barrel.svg)](https://crates.io/crates/barrel) [![Rust docs](https://docs.rs/barrel/badge.svg)](https://docs.rs/barrel/)

A schema migration building API, using Diesel as a backend and integrated query builder. Write complicated SQL schema migrations in Rust!

## Example

The API was recently completely changed to potentially easily allow different database backends to be used. The current iteration of the API can be seen below. Some of the functions might not fully work yet or need tweaking. In fact, a lot of the functions haven't been properly hooked up yet 😅

```rust
extern crate barrel;

use barrel::{Schema, Table};
use barrel::generators::postgres::*;

fn main() {
    let mut sql = Schema::<PGSQL>::new();
    sql.create_table("users", |t: &mut Table<PGSQL>| {
        t.increments();
        t.string("username");
        t.integer("plushy_sharks_owned");
    });

    println!("{}", sql.exec());
    // create table "users" ("id" serial primary key, "username" varchar(255), "plushy_sharks_owned" int)
}

```

If you have feedback regarding the API or things you would want barrel to do, please open an issue. And documentation PR's are always welcome 💚
