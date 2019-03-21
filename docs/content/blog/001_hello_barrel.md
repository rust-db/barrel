Title: Releasing 0.5.0
Category: Blog
Date: 2019-03-21 21:30

So `barrel.rs` has a blog now 🎉!

Today we are also happy to announce the release of version `0.5.0`.
A lot of work as gone into this release and breaks the API in a few ways.
This post will quickly highlight new features and also changes to the API.

### New type system

This release adds `barrel::types` which replaces the old `Column` types.
While types are still enums with different type variants, the creation
is now much more streamlined with a builder-style API.

```rust
types::varchar(255)
    .nullable(true)
    .default("Alice")
```

This change allows us to more easily support database specific types
in the future and makes it easier for you to create your own custom
type builders.
Check out the `barrel::types` documentation for details.

### Explicit IDs for tables

Since `0.2.0` a new table had an implicit `PRIMARY` key called `id`.
It was brought up several times that `barrel` should not have implicit behaviour like this
and and as such we have no reverted this!

Now you will need to create a new `id` field explicitly

```rust
table.add_column("id", types::primary())
```

We're still evaluating the option to add a `table.id()` function
(or similar) to make this process easier.

### Various changes

The `DatabaseExecutor` was renamed to `SqlRunner`,
there were several bug-fixes around `diesel` integration as well as
the table API.

Most importantly, you will need a very recent `diesel` version
which then depends on `0.5.0`.
If you're using barrel currently, nothing will change until you update.
Be aware that your existing migrations will need adjustment if you plan
on re-running them!

---

All in all, this is a pretty big step forward.
We're also tracking some things to implement on a roadmap
for an upcoming `1.0.0` release later this year (maybe!)
