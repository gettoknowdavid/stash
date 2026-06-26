use stash_core::money::Money;

#[test]
fn from_naira_rounds_to_nearest_kobo() {
    assert_eq!(Money::from_naira(10.005).0, 1001);
    assert_eq!(Money::from_naira(10.0).0, 1000);
    assert_eq!(Money::from_naira(5.0).0, 500);
    assert_eq!(Money::from_naira(1.0).0, 100);
}

#[test]
fn checked_add_succeeds_within_range() {
    let a = Money(100);
    let b = Money(50);
    assert_eq!(a.checked_add(&b).unwrap().0, 150)
}

#[test]
fn checked_add_fails_on_overflow() {
    let a = Money(i64::MAX);
    let b = Money(1);
    assert!(a.checked_add(&b).is_none());
}
