//! Builder API's module

use types::Type;
use super::impls::BaseType;

/// Creates an auto-incrementing primary key type
pub fn primary() -> Type<u64> {
    Type::new(BaseType::Primary)
}

/// Create a UUID primary key
pub fn uuid() -> Type<String> {
    Type::new(BaseType::UUID)
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
    unimplemented!()
}

/// Create an array of inner types
pub fn array<I>(inner: &Type<I>) -> Type<Type<I>> {
    Type::new(BaseType::Array(Box::new(inner.get_inner())))
}
