//! Implementation specifics for the type system

use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

/// Core type enum, describing the basic type
#[derive(PartialEq, Debug, Clone)]
pub enum BaseType {
    /// Strings
    Text,
    /// Like a String but worse
    Varchar(usize),
    /// Primary key (utility for incrementing integer – postgres supports this, we just mirror it)
    Primary,
    /// Simple integer
    Integer,
    /// Floating point number
    Float,
    /// Like Float but `~ ~ d o u b l e    p r e c i s i o n ~ ~`
    Double,
    /// A unique identifier type
    UUID,
    /// True or False
    Boolean,
    /// Json encoded data
    Json,
    /// Date And Time
    Date,
    /// <inconceivable jibberish>
    Binary,
    /// Foreign key to other table
    Foreign(&'static str),
    /// I have no idea what you are – but I *like* it
    Custom(&'static str),
    /// Any of the above, but **many** of them
    Array(Box<BaseType>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum WrappedDefault<'outer> {
    /// Strings
    Text(String),
    /// Like a String but worse
    Varchar(&'outer str),
    /// Simple integer
    Integer(i64),
    /// Floating point number
    Float(f32),
    /// Like Float but `~ ~ d o u b l e    p r e c i s i o n ~ ~`
    Double(f64),
    /// A unique identifier type
    UUID(String), // TODO: Change to UUID type
    /// True or False
    Boolean(bool),
    /// Date And Time
    Date(SystemTime),
    /// <inconceivable jibberish>
    Binary(&'outer [u8]),
    // /// Foreign key to other table
    // Foreign(Box<Type>),
    // I have no idea what you are – but I *like* it
    // Custom(&'static str),
    /// Any of the above, but **many** of them
    Array(Vec<Type>),
}

impl<'outer> Display for WrappedDefault<'outer> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::WrappedDefault::*;
        write!(
            f,
            "{}",
            &match *self {
                Text(ref val) => format!("{}", val),
                Varchar(ref val) => format!("{}", val),
                Integer(ref val) => format!("{}", val),
                Float(ref val) => format!("{}", val),
                Double(ref val) => format!("{}", val),
                UUID(ref val) => format!("{}", val),
                Boolean(ref val) => format!("{}", val),
                Date(ref val) => format!("{:?}", val),
                Binary(ref val) => format!("{:?}", val),
                Array(ref val) => format!("{:?}", val),
            }
        )
    }
}

/// A database column type and all the metadata attached to it
///
/// Using this struct directly is not recommended. Instead, you should be
/// using the constructor APIs in the `types` module.
///
/// A `Type` is an enum provided to other `barrel` APIs in order
/// to generate SQL datatypes. Working with them directly is possible
/// but not recommended.
///
/// Instead, you can use these helper functions to construct `Type` enums of
/// different...types and constraints. Field metadata is added via chainable
/// factory pattern functions.
///
/// ## Default values
///
/// If no additional arguments are provided, some assumptions will be made
/// about the metadata of a column type.
///
/// - `nullable`: `false`
/// - `indexed`: `false`
/// - `unique`: `false`
/// - `default`:  `None`
/// - `size`: `None` (which will error if size is important)
///
/// ## Examples
///
/// ```rust,norun
/// extern crate barrel;
/// use barrel::types::*;
///
/// // Make your own Primary key :)
/// let col = integer().increments(true).unique(true);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub nullable: bool,
    pub unique: bool,
    pub increments: bool,
    pub indexed: bool,
    pub default: Option<WrappedDefault<'static>>,
    pub size: Option<usize>,
    pub inner: BaseType,
}

/// This is a public API, be considered about breaking thigns
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Type {
    pub(crate) fn new(inner: BaseType) -> Self {
        Self {
            nullable: false,
            unique: false,
            increments: false,
            indexed: false,
            default: None,
            size: None,
            inner,
        }
    }

    /// Function used to hide the inner type to outside users (sneaky, I know)
    pub(crate) fn get_inner(&self) -> BaseType {
        self.inner.clone()
    }
    
    /// Set the nullability of this type
    pub fn nullable(self, arg: bool) -> Self {
        Self { nullable: arg, ..self }
    }
    
    /// Set the uniqueness of this type
    pub fn unique(self, arg: bool) -> Self {
        Self { unique: arg, ..self }
    }
    
    /// Specify if this type should auto-increment
    pub fn increments(self, arg: bool) -> Self {
        Self { increments: arg, ..self }
    }
    
    /// Specify if this type should be indexed by your SQL implementation
    pub fn indexed(self, arg: bool) -> Self {
        Self { indexed: arg, ..self }
    }
    
    /// Provide a default value for a type column
    pub fn default(self, arg: impl Into<WrappedDefault<'static>>) -> Self {
        Self { default: Some(arg.into()), ..self }
    }
    
    /// Specify a size limit (important or varchar & similar)
    pub fn size(self, arg: usize) -> Self {
        Self { size: Some(arg), ..self }
    }
}
