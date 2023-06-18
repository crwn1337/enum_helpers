use enum_helpers::{EnumAs, EnumAsMut};

#[derive(EnumAs)]
pub enum AsTest {
    A(u32), // Unnamed
    B { a: u32 }, // Named
    C { a: u32, b: u32 }, // Multiple
}

#[test]
fn test_as() {
    let a = AsTest::A(1);
    assert_eq!(a.as_b(), None);

    let b = AsTest::B { a: 1 };
    assert_eq!(b.as_b(), Some(&1));

    let c = AsTest::C { a: 1, b: 2 };
    assert_eq!(c.as_c(), Some((&1, &2)));
}

#[derive(EnumAsMut)]
pub enum AsMutTest {
    A(u32), // Unnamed
    B { a: u32 }, // Named
    C { a: u32, b: u32 }, // Multiple
}

#[test]
fn test_as_mut() {
    let mut a = AsMutTest::A(1);
    assert_eq!(a.as_mut_b(), None);

    let mut b = AsMutTest::B { a: 1 };
    *b.as_mut_b().unwrap() += 1;
    assert_eq!(b.as_mut_b(), Some(&mut 2));

    let mut c = AsMutTest::C { a: 1, b: 2 };
    *c.as_mut_c().unwrap().0 += 1;
    *c.as_mut_c().unwrap().1 += 1;
    assert_eq!(c.as_mut_c(), Some((&mut 2, &mut 3)));
}