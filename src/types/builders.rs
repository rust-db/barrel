//! Builder API's module

use types::Type;

/// Creates an auto-incrementing primary key type
pub fn primary() -> Type<u64> {
    unimplemented!()
}

/// Create a basic integer type
pub fn integer() -> Type<i64> {
    unimplemented!()
}

///
pub fn float() -> Type<f32> {
    unimplemented!()
}

///
pub fn double() -> Type<f64> {
    unimplemented!()
}

///
pub fn boolean() -> Type<bool> {
    unimplemented!()
}

///
pub fn varchar<'inner>() -> Type<&'inner str> {
    unimplemented!()
}

///
pub fn text() -> Type<String> {
    unimplemented!()
}

///
pub fn json() -> Type<()> {
    unimplemented!()
}

///
pub fn binary<'inner>() -> Type<&'inner [u8]> {
    unimplemented!()
}

///
pub fn foreign<'inner, I>() -> Type<&'inner Type<I>> {
    unimplemented!()
}

/// Create an array of inner types
pub fn array<I>(inner: Type<I>) -> Type<Type<I>> {
    unimplemented!()
}
