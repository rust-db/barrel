//! Builder API's module

use super::impls::BaseType;
use types::Type;

/// A standard primary numeric key type
///
/// It's 64-bit wide, can't be null or non-unique
/// and auto-increments on inserts.
/// Maps to `primary` on `Pg` and manually enforces
/// this behaviour for other Sql variants.
pub fn primary() -> Type<u64> {
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
pub fn uuid() -> Type<String> {
    Type::new(BaseType::UUID)
        .nullable(false)
        .unique(true)
        .indexed(true)
}

/// Create a basic integer type
pub fn integer() -> Type<i64> {
    Type::new(BaseType::Integer)
}

///
pub fn float() -> Type<f32> {
    Type::new(BaseType::Float)
}

///
pub fn double() -> Type<f64> {
    Type::new(BaseType::Double)
}

///
pub fn boolean() -> Type<bool> {
    Type::new(BaseType::Boolean)
}

///
pub fn varchar<'inner>() -> Type<&'inner str> {
    Type::new(BaseType::Varchar)
}

///
pub fn text() -> Type<String> {
    Type::new(BaseType::Text)
}

///
pub fn json() -> Type<()> {
    unimplemented!()
}

///
pub fn binary<'inner>() -> Type<&'inner [u8]> {
    Type::new(BaseType::Binary)
}

///
pub fn foreign<'inner, I>(inner: &Type<I>) -> Type<&'inner Type<I>> {
    Type::new(BaseType::Foreign(Box::new(inner.get_inner())))
}

pub fn date<'inner, I>() -> Type<&'inner Type<I>> {
    Type::new(BaseType::Date)
}

/// Create an array of inner types
pub fn array<I>(inner: &Type<I>) -> Type<Type<I>> {
    Type::new(BaseType::Array(Box::new(inner.get_inner())))
}
