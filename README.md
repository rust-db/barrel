# barrel

[![Build Status](https://travis-ci.org/spacekookie/barrel.svg?branch=master)](https://travis-ci.org/spacekookie/barrel)  [![Coverage Status](https://coveralls.io/repos/github/spacekookie/barrel/badge.svg?branch=master)](https://coveralls.io/github/spacekookie/barrel?branch=master)ß

A schema building API, using Diesel as a backend and integrated query builder. Write SQL schema migrations in Rust!

## Example

```rust
extern crate barrel;
use barrel::*;

fn main() {
    let mut s = Schema::new().with_schema("public").create_table("users", |t: &mut Table| {
        t.increments();
    });

    println!("{}", s.exec());
    // Outputs: create table "public"."users" ("id" serial primary key)
}
```

The API is very early in development and most things that need to work, don't yet 😝. 