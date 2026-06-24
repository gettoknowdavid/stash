use proptest::prelude::*;
use stash_core::{ids, stock};

pub fn test_fixture() -> stock::StockLevel {
    stock::StockLevel {
        item_id: ids::ItemId::new(),
        warehouse_id: ids::WarehouseId::new(),
        quantity: 100,
    }
}

proptest! {
    #[test]
    fn quantity_never_goes_negative(start in 0u32..10_000, outbound in 0u32..20_000) {
        let stock = stock::StockLevel {quantity: start, ..test_fixture()};
        let movement = stock::StockMovement::Outbound {qty: outbound, reason: "test".into()};
        match stock.apply(&movement) {
            Ok(new_stock) => assert!(new_stock.quantity <= start),
            Err(stash_core::CoreError::InsufficientStock { .. }) => {},
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }
}
