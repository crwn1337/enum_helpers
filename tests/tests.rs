use enum_helpers::{EnumAs, EnumAsMut, EnumIs};

#[allow(dead_code)]
pub struct SmallStruct {
    a: u32,
    b: u32,
}

#[allow(dead_code)]
#[derive(EnumIs, EnumAs, EnumAsMut)]
pub enum Test {
    None,
    Unnamed(u32),
    Named { a: u32 },
    Multiple(u32, u32),
    MultipleNamed { a: u32, b: u32 },
    Small(SmallStruct),
}

#[test]
fn test() {
    is_tests();
    as_tests();

    // too much copy pasting, so a simple as_mut test:
    let mut test = Test::Small(SmallStruct { a: 123, b: 456 });
    match test.as_mut_small() {
        Some(small) => small.a += 1,
        None => panic!(),
    }
    assert_eq!(test.as_mut_small().unwrap().a, 124);
}

fn as_tests() {
    // None skipped since it is a unit, so it has no fields

    assert_eq!(Test::Unnamed(123).as_unnamed(), Some(&123));
    assert_eq!(Test::Named { a: 123 }.as_unnamed(), None);
    assert_eq!(Test::Multiple(123, 456).as_unnamed(), None);
    assert_eq!(Test::MultipleNamed { a: 123, b: 456 }.as_unnamed(), None);

    assert_eq!(Test::Unnamed(123).as_named(), None);
    assert_eq!(Test::Named { a: 123 }.as_named(), Some(&123));
    assert_eq!(Test::Multiple(123, 456).as_named(), None);
    assert_eq!(Test::MultipleNamed { a: 123, b: 456 }.as_named(), None);

    assert_eq!(Test::Unnamed(123).as_multiple(), None);
    assert_eq!(Test::Named { a: 123 }.as_multiple(), None);
    assert_eq!(Test::Multiple(123, 456).as_multiple(), Some((&123, &456)));
    assert_eq!(Test::MultipleNamed { a: 123, b: 456 }.as_multiple(), None);

    assert_eq!(Test::Unnamed(123).as_multiple_named(), None);
    assert_eq!(Test::Named { a: 123 }.as_multiple_named(), None);
    assert_eq!(Test::Multiple(123, 456).as_multiple_named(), None);
    assert_eq!(
        Test::MultipleNamed { a: 123, b: 456 }.as_multiple_named(),
        Some((&123, &456))
    );
}

fn is_tests() {
    assert!(Test::None.is_none());
    assert!(Test::Unnamed(123).is_unnamed());
    assert!(Test::Named { a: 123 }.is_named());
    assert!(Test::Multiple(123, 456).is_multiple());
    assert!(Test::MultipleNamed { a: 123, b: 456 }.is_multiple_named());

    assert!(!Test::None.is_unnamed());
    assert!(!Test::None.is_named());
    assert!(!Test::None.is_multiple());
    assert!(!Test::None.is_multiple_named());

    assert!(!Test::Unnamed(123).is_none());
    assert!(!Test::Unnamed(123).is_named());
    assert!(!Test::Unnamed(123).is_multiple());
    assert!(!Test::Unnamed(123).is_multiple_named());

    assert!(!Test::Named { a: 123 }.is_none());
    assert!(!Test::Named { a: 123 }.is_unnamed());
    assert!(!Test::Named { a: 123 }.is_multiple());
    assert!(!Test::Named { a: 123 }.is_multiple_named());

    assert!(!Test::Multiple(123, 456).is_none());
    assert!(!Test::Multiple(123, 456).is_unnamed());
    assert!(!Test::Multiple(123, 456).is_named());
    assert!(!Test::Multiple(123, 456).is_multiple_named());

    assert!(!Test::MultipleNamed { a: 123, b: 456 }.is_none());
    assert!(!Test::MultipleNamed { a: 123, b: 456 }.is_unnamed());
    assert!(!Test::MultipleNamed { a: 123, b: 456 }.is_named());
    assert!(!Test::MultipleNamed { a: 123, b: 456 }.is_multiple());
}