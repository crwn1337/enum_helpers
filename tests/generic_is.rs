use enum_helpers::EnumIs;

#[derive(EnumIs)]
pub enum GenericIs<T> {
    A,
    B(T),
}

#[test]
fn test_generic_is() {
    let a = GenericIs::<i32>::A;
    assert!(a.is_a());
    assert!(!a.is_b());

    let b = GenericIs::<i32>::B(1);
    assert!(!b.is_a());
    assert!(b.is_b());
}
