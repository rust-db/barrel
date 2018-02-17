![](assets/logo_wide.svg)

[![](https://travis-ci.org/spacekookie/barrel.svg?branch=master)](https://travis-ci.org/spacekookie/barrel)
[![](https://ci.appveyor.com/api/projects/status/7e00r2e1xatxk3bj?svg=true)](https://ci.appveyor.com/project/spacekookie/barrel)
[![](https://coveralls.io/repos/github/spacekookie/barrel/badge.svg?branch=master&service=github)](https://coveralls.io/github/spacekookie/barrel?branch=master)
[![](https://docs.rs/barrel/badge.svg)](https://docs.rs/barrel/)
[![](https://img.shields.io/crates/v/barrel.svg)](https://crates.io/crates/barrel)
[![](https://img.shields.io/crates/d/barrel.svg)](https://crates.io/crates/barrel)


A powerful schema migration builder for Rust. Write complicated SQL schema migrations in Rust and easily switch databases.

`barrel` is meant to make writing migrations for different databases as easy as possible. It creates a simple to use API over SQL which allows you to focus on managing your database, not fighting with SQL.


## Example

`barrel` currently requires the nightly compiler. The following code is a simple example of how to get started

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


## License

`barrel` is free software: you can redistribute it and/or modify it under the terms of the MIT Public License.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the MIT Public License for more details.


## Conduct

In the interest of fostering an open and welcoming environment, the `barrel` project pledges to making participation a harassment-free experience for everyone. See [Code of Conduct](code_of_conduct.md) for details. In case of violations, e-mail [kookie@spacekookie.de](mailto:kookie@spacekookie.de).
