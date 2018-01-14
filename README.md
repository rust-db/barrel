# barrel

A schema builder for Rust, using Diesel as it's backend (for now)

## How to use

(Eventually)

```rust
extern crate barrel;
use barrel::*;

let s = Schema::name("public").create_table("users", |t: &mut Table| {
    t.increments();
});

println("{}", s.exec());
// Outputs:
//          create table "public"."users" ("id" serial primary key)
```

This is heavily WIP and more will follow 😜