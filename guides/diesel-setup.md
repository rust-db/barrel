# Diesel setup

**Disclaimer:** the barrel crate is still in an early state of development and should not be considered "stable". Use at your own risk!

Using rust migrations (via `barrel`) with `diesel` is really simple. First, you need to add the `rust-migrations` to the features list for your `diesel` dependency. Note that currently `barrel` only supports postgres databases.


```toml
[dependencies]
diesel = { version = "1.2", features = ["rust-migrations", "postgres"] }
# ...
```

Also make sure that you installed the `diesel_cli` with the `rust-migrations` feature flag as well

```bash
~ cargo install diesel_cli --features="rust-migrations"
```

