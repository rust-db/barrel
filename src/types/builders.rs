//! Builder API's module

use super::impls::Constraint;
use super::impls::{BaseType, ReferentialAction, WrapVec};
use crate::types::Type;

/// A standard primary numeric key type
///
/// It's 64-bit wide, can't be null or non-unique
/// and auto-increments on inserts.
/// Maps to `primary` on `Pg` and manually enforces
/// this behaviour for other Sql variants.
pub fn primary() -> Type {
    Type::new(BaseType::Primary)
        .nullable(true) // Primary keys are non-null implicitly
        .increments(true) // This is ignored for now
        .primary(false) // Primary keys are primary implictly
        .unique(false) // Primary keys are unique implicitly
        .indexed(false)
}

/// A (standardised) UUID primary key type
///
/// Similar to `primary()`, but uses a standard
/// layout UUID type, mapping to `uuid` on `Pg`
/// and not supported by all Sql variants.
pub fn uuid() -> Type {
    Type::new(BaseType::UUID)
        .nullable(false)
        .unique(true)
        .indexed(true)
}

/// Create a basic integer type
pub fn integer() -> Type {
    Type::new(BaseType::Integer)
}

/// Create an auto-incrementing integer type
pub fn serial() -> Type {
    Type::new(BaseType::Serial)
}

/// A 32-bit floating point type
pub fn float() -> Type {
    Type::new(BaseType::Float)
}

/// A 64-bit floating point type
pub fn double() -> Type {
    Type::new(BaseType::Double)
}

/// A boolean data type (true, false)
pub fn boolean() -> Type {
    Type::new(BaseType::Boolean)
}

/// A variable-length string type
pub fn varchar(len: usize) -> Type {
    Type::new(BaseType::Varchar(len))
}

/// A fixed-length string type
pub fn r#char(len: usize) -> Type {
    Type::new(BaseType::Char(len))
}

/// A variable-length string type
pub fn text() -> Type {
    Type::new(BaseType::Text)
}

/// A json-type column – not supported by all backends
pub fn json() -> Type {
    Type::new(BaseType::Json)
}

/// Embed binary data
pub fn binary<'inner>() -> Type {
    Type::new(BaseType::Binary)
}

/// Create a column that points to some foreign table
pub fn foreign<S, I>(
    table: S,
    keys: I,
    on_update: ReferentialAction,
    on_delete: ReferentialAction,
) -> Type
where
    S: Into<String>,
    I: Into<WrapVec<String>>,
{
    Type::new(BaseType::Foreign(
        None,
        table.into(),
        keys.into(),
        on_update,
        on_delete,
    ))
}

/// Like `foreign(...)` but letting you provide an external schema
///
/// This function is important when making cross-schema references
pub fn foreign_schema<S, I>(
    schema: S,
    table: S,
    keys: I,
    on_update: ReferentialAction,
    on_delete: ReferentialAction,
) -> Type
where
    S: Into<String>,
    I: Into<WrapVec<String>>,
{
    Type::new(BaseType::Foreign(
        Some(schema.into()),
        table.into(),
        keys.into(),
        on_update,
        on_delete,
    ))
}

/// Any custom SQL type that is embedded into a migration
pub fn custom(sql: &'static str) -> Type {
    Type::new(BaseType::Custom(sql))
}

/// An SQL date type
pub fn date() -> Type {
    Type::new(BaseType::Date)
}

/// An SQL time type
pub fn time() -> Type {
    Type::new(BaseType::Time)
}

/// An SQL datetime type
pub fn datetime() -> Type {
    Type::new(BaseType::DateTime)
}

/// Create an array of inner types
pub fn array(inner: &Type) -> Type {
    Type::new(BaseType::Array(Box::new(inner.get_inner())))
}

/// Create an index over multiple, existing columns of the same type
pub fn index<I, S>(columns: I) -> Type
where
    S: ToString,
    I: IntoIterator<Item = S>,
{
    let vec: Vec<String> = columns.into_iter().map(|s| s.to_string()).collect();
    Type::new(BaseType::Index(vec))
}

/// Create a constraint over multiple, existing columns of the same type
pub fn unique_constraint<I, S>(columns: I) -> Type
where
    S: ToString,
    I: IntoIterator<Item = S>,
{
    let vec: Vec<String> = columns.into_iter().map(|s| s.to_string()).collect();
    Type::new(BaseType::Constraint(Constraint::Unique, vec))
}
