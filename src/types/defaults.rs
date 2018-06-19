//! Gracefully handle defaults (to the outside)

use super::impls::{BaseType, Type};

pub(crate) struct TypeDefault {
    inner: BaseType,
    sql_string: String,
}

trait Defaultable<T> {
    fn validate(&self, t: Type) -> bool;
    fn get(&self) -> T;
}

macro_rules! validate_type {
    ($x:expr, $pattern:pat) => {
        match $x.get_inner() {
            $pattern => true,
            _ => false,
        }
    };
}

struct BoolDefault(bool);
impl Defaultable<bool> for BoolDefault {
    fn validate(&self, t: Type) -> bool {
        validate_type!(t, BaseType::Boolean)
    }

    fn get(&self) -> bool {
        self.0
    }
}

struct TextDefault(String);
impl Defaultable<String> for TextDefault {
    fn validate(&self, t: Type) -> bool {
        let a = validate_type!(t, BaseType::Text) || validate_type!(t, BaseType::Varchar);
        match t.size {
            Some(s) => if self.0.len() <= s {
                true && a
            } else {
                false
            },
            _ => true && a,
        }
    }

    fn get(&self) -> String {
        self.0
    }
}

struct IntegerDefault(i64);
impl Defaultable<i64> for IntegerDefault {
    fn validate(&self, t: Type) -> bool {
        validate_type!(t, BaseType::Integer)
    }

    fn get(&self) -> i64 {
        self.0
    }
}

macro_rules! check_type {
    ($into:expr, $_type:pat) => {
        match $into {
            $_type => {}
            _ => panic!("Can't turn {:?} into <Unknown> like that", $into),
        }
    };
}

impl From<(bool, Type)> for TypeDefault {
    fn from((b, bt): (bool, Type)) -> Self {
        check_type!(bt.get_inner(), BaseType::Boolean);

        Self {
            inner: bt,
            sql_string: match b {
                true => "TRUE",
                false => "FALSE",
            }.into(),
        }
    }
}

impl From<(String, Type)> for TypeDefault {
    fn from((b, bt): (String, Type)) -> Self {
        match bt.get_inner() {
            BaseType::Text | BaseType::Varchar => {}
            _ => panic!("Can't turn {:?} into Text like that", bt),
        }

        Self {
            inner: bt.get_inner(),
            sql_string: match bt.get_inner() {
                BaseType::Text => b.into(),
                BaseType::Varchar => if b.len() < bt.size.unwrap() {
                    b.into()
                } else {
                    panic!(
                        "String is too long to fit into the size constraint: {}",
                        b.len()
                    );
                },
            },
        }
    }
}
