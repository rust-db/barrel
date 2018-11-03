//! Builder API's module

use super::impls::BaseType;
use types::Type;

/// A standard primary numeric key type
///
/// It's 64-bit wide, can't be null or non-unique
/// and auto-increments on inserts.
/// Maps to `primary` on `Pg` and manually enforces
/// this behaviour for other Sql variants.
pub fn primary() -> Type {
    Type::new(BaseType::Primary)
        .nullable(false)
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

///
pub fn float() -> Type {
    Type::new(BaseType::Float)
}

///
pub fn double() -> Type {
    Type::new(BaseType::Double)
}

///
pub fn boolean() -> Type {
    Type::new(BaseType::Boolean)
}

///
pub fn varchar(len: usize) -> Type {
    Type::new(BaseType::Varchar(len))
}

///
pub fn text() -> Type {
    Type::new(BaseType::Text)
}

///
pub fn json() -> Type {
    unimplemented!()
}

///
pub fn binary<'inner>() -> Type {
    Type::new(BaseType::Binary)
}

///
pub fn foreign(inner: &Type) -> Type {
    Type::new(BaseType::Foreign(Box::new(inner.get_inner())))
}

pub fn date() -> Type {
    Type::new(BaseType::Date)
}

/// Create an array of inner types
pub fn array(inner: &Type) -> Type {
    Type::new(BaseType::Array(Box::new(inner.get_inner())))
}
