use stash_core::ids::ItemId;

#[test]
fn round_trip_string_conversion() {
    let id = ItemId::new();
    let s: String = id.into();
    let parsed = ItemId::try_from(s).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn invalid_string_errors() {
    assert!(ItemId::try_from("not-a-uuid".to_string()).is_err());
}
