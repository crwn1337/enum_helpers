use enum_helpers::EnumAs;

#[derive(Debug, PartialEq)]
pub struct AsStruct {
    pub a: u32,
}

#[derive(EnumAs)]
pub enum AsTest {
    A(u32),
    B(AsStruct),
}

#[test]
fn test_struct() {
    let a = AsTest::A(1);
    assert_eq!(a.as_a(), Some(&1));

    let b = AsTest::B(AsStruct { a: 1 });
    assert_eq!(b.as_b(), Some(&AsStruct { a: 1 }));
}