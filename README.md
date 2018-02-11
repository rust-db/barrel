![](assets/logo_wide.svg)

[![](https://travis-ci.org/spacekookie/barrel.svg?branch=master)](https://travis-ci.org/spacekookie/barrel) [![](https://coveralls.io/repos/github/spacekookie/barrel/badge.svg?branch=master)](https://coveralls.io/github/spacekookie/barrel?branch=master) [![](https://img.shields.io/crates/v/barrel.svg)](https://crates.io/crates/barrel) [![](https://docs.rs/barrel/badge.svg)](https://docs.rs/barrel/)

A schema migration builder for Rust. Write complicated SQL schema migrations in Rust and easily switch databases.

## Example

The API was recently completely changed (**again**). The current iteration of the API can be seen below. Some of the functions might not fully work yet or need tweaking. In fact, a lot of the functions haven't been properly hooked up yet 😅

```rust
extern crate barrel;

use barrel::*;

fn main() {
    let mut m = Migration::new();
    m.create_table("users", |t| {
        t.add_column("name", Type::Text);
        t.add_column("age", Type::Integer);
        t.add_column("owns_plushy_sharks", Type::Boolean);
    });

    // I like plushy sharks
    m.rename_table("sharks", "plushies");
}

```

If you have feedback regarding the API or things you would want barrel to do, please open an issue. And documentation PR's are always welcome 💚


## License

`barrel` is free software: you can redistribute it and/or modify it under the terms of the MIT Public License.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the MIT Public License for more details.


## Conduct

In the interest of fostering an open and welcoming environment, the `barrel` project pledges to making participation a harassment-free experience for everyone. See [Code of Conduct](code_of_conduct.md) for details. In case of violations, e-mail [kookie@spacekookie.de](mailto:kookie@spacekookie.de).
