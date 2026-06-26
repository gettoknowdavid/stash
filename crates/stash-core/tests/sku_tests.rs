use stash_core::sku::Sku;

#[test]
fn accepts_valid_sku_and_uppercases() {
    let sku = Sku::parse("abc-123").unwrap();
    assert_eq!(sku.0, "ABC-123")
}

#[test]
fn rejects_too_short() {
    let sku = Sku::parse("ab");
    assert!(sku.is_err())
}

#[test]
fn rejects_too_long() {
    let str = "A".repeat(21);
    let sku = Sku::parse(&str);
    assert!(sku.is_err())
}

#[test]
fn rejects_invalid_characters() {
    assert!(Sku::parse("ABC_123").is_err());
    assert!(Sku::parse("ABC 123").is_err());
    assert!(Sku::parse("ABC-123?").is_err());
}

#[test]
fn boundary_lengths_are_accepted() {
    assert!(Sku::parse("ABC").is_ok());
    assert!(Sku::parse(&"A".repeat(20)).is_ok());
}
