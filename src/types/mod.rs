//! Types constructor module
//!
//! A `Type` is an enum provided to other `barrel` APIs in order
//! to generate SQL datatypes. Working with them directly is possible
//! but not recommended.
//!
//! Instead, you can use these helper functions to construct `Type` enums of
//! different...types and constraints. Field metadata is added via chainable
//! factory pattern functions.
//!
//! ## Default values
//!
//! If no additional arguments are provided, some assumptions will be made
//! about the metadata of a column type.
//!
//! - `nullable`: `false`
//! - `indexed`: `false`
//! - `unique`: `false`
//! - `default`:  `None`
//! - `size`: `None` (which will error if size is important)
//!
//! ## Examples
//!
//! ```norun
//! extern crate barrel;
//! use barrel::types::*;
//!
//! // Make your own Primary key :)
//! let col = integer().increments(true).unique(true);
//! ```

mod builders;
mod impls;

/// Export all builder functions
pub use self::builders::*;

/// Export only the Type struct
pub use self::impls::{BaseType, Type, WrappedDefault};
