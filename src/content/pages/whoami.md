Title: barrel.rs
Subtitle: NULL
URL: /
Save_As: index.html
Template: page

![](images/logo.svg)

A powerful schema migration builder, that let's you write your SQL migrations in Rust.

`barrel` makes writing migrations for different databases as easy as possible.
It provides you with a common API over SQL,
with certain features only provided for database specific implementations.
This way you can focus on your Rust code, and stop worrying about SQL.

## Example

The following example will help you get started

```rust
use barrel::{types, Migration, Pg};
use barrel::backend::Pg;

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

## Unstable features

Starting with `v0.2.4` `barrel` now has an `unstable` feature flag which will hide features and breaking changes that are in-development at the time of a minor or patch release. You can use these features if you so desire, but be aware that their usage will change more rapidely between versions (even patches) and their usage will be badly documented.

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

