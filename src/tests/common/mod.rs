/* Include some external tests */
mod cloning;
mod utils;

#[cfg(all(feature = "sqlite3", feature = "pg", feature = "mysql"))]
mod runtime;

use crate::types::{BaseType, Type, WrappedDefault};
use crate::Migration;

#[test]
fn create_multiple_tables() {
    let mut migr = Migration::new();
    migr.create_table("foo", |_| {});
    migr.create_table("bar", |_| {});

    assert!(migr.changes.len() == 2);
}

#[test]
fn create_table_if_not_exists() {
    let mut migr = Migration::new();
    migr.create_table_if_not_exists("foo", |_| {});

    assert!(migr.changes.len() == 1);
}

#[test]
fn pin_public_api() {
    // The best sql type because it's very queer 🏳️‍🌈
    let tt = Type::new(BaseType::Custom("GAY"));

    assert_eq!(tt.nullable, false);
    assert_eq!(tt.indexed, false);
    assert_eq!(tt.unique, false);
    assert_eq!(tt.increments, false);
    assert_eq!(tt.default, None);
    assert_eq!(tt.size, None);
    assert_eq!(tt.inner, BaseType::Custom("GAY"));
}

#[test]
fn pin_struct_layout() {
    // The best sql type because it's very queer 🏳️‍🌈
    let tt = Type {
        nullable: false,
        indexed: false,
        primary: false,
        unique: false,
        increments: false,
        default: None,
        size: None,
        inner: BaseType::Custom("GAY"),
    };

    assert_eq!(tt.nullable, false);
    assert_eq!(tt.indexed, false);
    assert_eq!(tt.primary, false);
    assert_eq!(tt.unique, false);
    assert_eq!(tt.increments, false);
    assert_eq!(tt.default, None);
    assert_eq!(tt.size, None);
    assert_eq!(tt.inner, BaseType::Custom("GAY"));
}

#[test]
fn default_render_anytext() {
    use self::WrappedDefault::*;
    assert_eq!(format!("{}", AnyText("hello".into())), "hello".to_owned());
}

#[test]
fn default_render_integer() {
    use self::WrappedDefault::*;
    assert_eq!(format!("{}", Integer(42)), "42".to_owned());
}

#[test]
fn default_render_float() {
    use self::WrappedDefault::*;
    assert_eq!(format!("{}", Float(42000.0)), "42000".to_owned());
}

#[test]
fn default_render_double() {
    use self::WrappedDefault::*;
    assert_eq!(
        format!("{}", Double(123456789.123456789)),
        "123456789.12345679".to_owned()
    );
}

#[test]
fn default_render_uuid() {
    use self::WrappedDefault::*;
    assert_eq!(
        format!("{}", UUID("b616ab2a-e13c-11e8-9f32-f2801f1b9fd1".into())),
        "b616ab2a-e13c-11e8-9f32-f2801f1b9fd1".to_owned()
    );
}

// #[test]
// fn default_render_date() {
//     use self::WrappedDefault::*;
//     assert_eq!(format!("{}", Date(SystemTime::now())), "".to_owned());
// }

#[test]
fn default_render_binary() {
    use self::WrappedDefault::*;
    assert_eq!(
        format!(
            "{}",
            Binary(&[
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF
            ])
        ),
        "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]".to_owned()
    );
}

// #[test]
// fn default_render_array() {
//     use self::WrappedDefault::*;
//     assert_eq!(
//         format!("{}", Array(vec![Type::new(BaseType::Custom("GAY"))])),
//         "".to_owned()
//     );
// }
