use enum_helpers::EnumIs;

#[derive(EnumIs)]
pub enum Generic<T> {
    A,
    B(T),
}

#[test]
fn test_generic() {
    let a = Generic::<i32>::A;
    assert!(a.is_a());
    assert!(!a.is_b());

    let b = Generic::<i32>::B(1);
    assert!(!b.is_a());
    assert!(b.is_b());
}