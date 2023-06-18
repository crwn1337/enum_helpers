use enum_helpers::EnumInto;

#[derive(EnumInto)]
pub enum IntoTest {
    A(u32),
}

#[test]
fn test_into() {
    let a = IntoTest::A(1);
    assert_eq!(a.into_a(), Some(1));

    // TODO: test for compilation errors somehow
    // assert_eq!(a.into_a(), ...);
}
