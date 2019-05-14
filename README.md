![](assets/logo.svg)

[![](https://travis-ci.org/rust-db/barrel.svg?branch=master)](https://travis-ci.org/spacekookie/barrel)
[![](https://ci.appveyor.com/api/projects/status/7e00r2e1xatxk3bj?svg=true)](https://ci.appveyor.com/project/spacekookie/barrel)
[![](https://coveralls.io/repos/github/spacekookie/barrel/badge.svg?branch=master&service=github)](https://coveralls.io/github/spacekookie/barrel?branch=master)
[![](https://docs.rs/barrel/badge.svg)](https://docs.rs/barrel/)
[![](https://img.shields.io/crates/v/barrel.svg)](https://crates.io/crates/barrel)
[![](https://img.shields.io/crates/d/barrel.svg)](https://crates.io/crates/barrel)

A powerful database schema builder, that let's you write your SQL migrations in Rust!

`barrel` offers callback-style builder functions for SQL migrations
and is designed to be flexible, portable and fun to use.
It provides you with a common interface over SQL,
with additional database-specific builders.

This way you can focus on your Rust code, without having to worry about SQL.

## Example

The following example will help you get started

```rust
use barrel::{types, Migration, Pg};

fn main() {
    let mut m = Migration::new();

    m.create_table("users", |t| {
        t.add_column("name", types::varchar(255));
        t.add_column("age", types::integer());
        t.add_column("owns_plushy_sharks", types::boolean());
    });

    println!("{}", m.make::<Pg>());
}
```

## Using Diesel

Since `diesel 1.2.0` it's possible to now use `barrel` for migrations with `diesel`. A guide with some more information on how to get started can be found [here](https://github.com/spacekookie/barrel/blob/master/guides/diesel-setup.md)

### Migration guide

If you've been using `barrel` to write migrations for `diesel` before the `0.5.0` release,
some migration of your migrations will be required.
Since `0.5.0` the way types are constructed changed.
Instead of constructing a type with `Types::VarChar(255)` (an enum variant),
the types are now provided by a module called `types` and builder functions.
The same type would now be `types::varchar(255)` (a function call),
which then returns a `Type` enum.

You can also directly created your own `Type` builders this way.
Check the docs for details!

## License

`barrel` is free software: you can redistribute it and/or modify it
under the terms of the MIT Public License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the MIT Public License for more details.

## Conduct

In the interest of fostering an open and welcoming environment,
the `barrel` project pledges to making participation a harassment-free experience for everyone.
See [Code of Conduct](code_of_conduct.md) for details.
In case of violations, e-mail [kookie@spacekookie.de](mailto:kookie@spacekookie.de).
