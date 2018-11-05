use types;

#[test]
fn cloning_types() {
    let tt = types::text();
    assert_eq!(tt, tt.clone());
}

#[test]
fn debugging_types() {
    let tt = types::text();
    assert_eq!(format!("{:#?}", tt), "Type {\n    nullable: false,\n    unique: false,\n    increments: false,\n    indexed: false,\n    default: None,\n    size: None,\n    inner: Text\n}".to_owned());
}

#[test]
fn equals_types() {
    let t1 = types::text();
    let t2 = t1.clone();
    let t3 = types::integer();
    assert!(t1 == t2);
    assert!(t1 != t3);
}
