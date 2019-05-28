
use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

use super::Type;

#[derive(PartialEq, Debug, Clone)]
pub enum WrappedDefault<'outer> {
    /// Any text information
    AnyText(&'outer str),
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
    /// Foreign key to other table
    Foreign(Box<Type>),
    // I have no idea what you are – but I *like* it
    Custom(&'static str),
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
                AnyText(ref val) => format!("{}", val),
                Integer(ref val) => format!("{}", val),
                Float(ref val) => format!("{}", val),
                Double(ref val) => format!("{}", val),
                UUID(ref val) => format!("{}", val),
                Boolean(ref val) => format!("{}", val),
                Date(ref val) => format!("{:?}", val),
                Binary(ref val) => format!("{:?}", val),
                Foreign(ref val) => format!("{:?}", val),
                Custom(ref val) => format!("{}", val),
                Array(ref val) => format!("{:?}", val),
            }
        )
    }
}

impl From<&'static str> for WrappedDefault<'static> {
    fn from(s: &'static str) -> Self {
        WrappedDefault::AnyText(s)
    }
}

impl From<i64> for WrappedDefault<'static> {
    fn from(s: i64) -> Self {
        WrappedDefault::Integer(s)
    }
}

impl From<f32> for WrappedDefault<'static> {
    fn from(s: f32) -> Self {
        WrappedDefault::Float(s)
    }
}

impl From<f64> for WrappedDefault<'static> {
    fn from(s: f64) -> Self {
        WrappedDefault::Double(s)
    }
}

impl From<bool> for WrappedDefault<'static> {
    fn from(s: bool) -> Self {
        WrappedDefault::Boolean(s)
    }
}

impl From<SystemTime> for WrappedDefault<'static> {
    fn from(s: SystemTime) -> Self {
        WrappedDefault::Date(s)
    }
}