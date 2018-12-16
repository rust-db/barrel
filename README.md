![](assets/logo.svg)

[![](https://travis-ci.org/spacekookie/barrel.svg?branch=master)](https://travis-ci.org/spacekookie/barrel)
[![](https://ci.appveyor.com/api/projects/status/7e00r2e1xatxk3bj?svg=true)](https://ci.appveyor.com/project/spacekookie/barrel)
[![](https://coveralls.io/repos/github/spacekookie/barrel/badge.svg?branch=master&service=github)](https://coveralls.io/github/spacekookie/barrel?branch=master)
[![](https://docs.rs/barrel/badge.svg)](https://docs.rs/barrel/)
[![](https://img.shields.io/crates/v/barrel.svg)](https://crates.io/crates/barrel)
[![](https://img.shields.io/crates/d/barrel.svg)](https://crates.io/crates/barrel)

A powerful schema migration builder, that let's you write your SQL migrations in Rust.

`barrel` makes writing migrations for different databases as easy as possible. It provides you with a common API over SQL, with certain features only provided for database specific implementations.
This way you can focus on your Rust code, and stop worrying about SQL.

While `barrel` works on stable rust with most features, please note that some experimental features might require the nightly compiler.

## Example

The following example will help you get started

```rust
extern crate barrel;

use barrel::*;
use barrel::backend::Pg;

fn main() {
    let mut m = Migration::new();
    m.create_table("users", |t| {
        t.add_column("name", Type::Text);
        t.add_column("age", Type::Integer);
        t.add_column("owns_plushy_sharks", Type::Boolean);
    });

    println!("{}", m.make::<Pg>());
}
```

If you have feedback [regarding the API](https://github.com/spacekookie/barrel/issues/1) or things you would want barrel to do, please open an issue. And documentation PR's are always welcome 💚

## Using Diesel

Since `diesel 1.2.0` it's possible to now use `barrel` for migrations with `diesel`. A guide with some more information on how to get started can be found [here](https://github.com/spacekookie/barrel/blob/master/guides/diesel-setup.md)


## Unstable features

Starting with `v0.2.4` `barrel` now has an `unstable` feature flag which will hide features and breaking changes that are in-development at the time of a minor or patch release. You can use these features if you so desire, but be aware that their usage will change more rapidely between versions (even patches) and their usage will be badly documented.

## License

`barrel` is free software: you can redistribute it and/or modify it under the terms of the MIT Public License.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the MIT Public License for more details.


## Conduct

In the interest of fostering an open and welcoming environment, the `barrel` project pledges to making participation a harassment-free experience for everyone. See [Code of Conduct](code_of_conduct.md) for details. In case of violations, e-mail [kookie@spacekookie.de](mailto:kookie@spacekookie.de).
