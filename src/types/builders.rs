//! Builder API's module

use super::impls::BaseType;
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
        .increments(true)
        .unique(true)
        .indexed(true)
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

/// A fixed-length string type
pub fn varchar(len: usize) -> Type {
    Type::new(BaseType::Varchar(len))
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
pub fn foreign(inner: &'static str) -> Type {
    Type::new(BaseType::Foreign(inner))
}

/// Any custom SQL type that is embedded into a migration
pub fn custom(sql: &'static str) -> Type {
    Type::new(BaseType::Custom(sql))
}

/// An SQL date type
pub fn date() -> Type {
    Type::new(BaseType::Date)
}

/// Create an array of inner types
pub fn array(inner: &Type) -> Type {
    Type::new(BaseType::Array(Box::new(inner.get_inner())))
}
