use stash_core::ids::{ItemId, WarehouseId};
use stash_core::stock::{StockLevel, StockMovement};

fn fixture(qty: u32) -> StockLevel {
    StockLevel { item_id: ItemId::new(), warehouse_id: WarehouseId::new(), quantity: qty }
}

#[test]
fn inbound_increases_quantity() {
    let stock = fixture(10);
    let movement = StockMovement::Inbound { qty: 5, reason: "restock".to_string() };
    let result = stock.apply(&movement).unwrap();
    assert_eq!(result.quantity, 15);
}

#[test]
fn outbound_decreases_quantity() {
    let stock = fixture(20);
    let movement = StockMovement::Outbound { qty: 9, reason: "sale".to_string() };
    let result = stock.apply(&movement).unwrap();
    assert_eq!(result.quantity, 11);
}

#[test]
fn outbound_beyond_stock_errors() {
    let stock = fixture(3);
    let movement = StockMovement::Outbound { qty: 19, reason: "sale".to_string() };
    let result = stock.apply(&movement);
    assert!(result.is_err());
}

#[test]
fn adjustment_negative_within_bounds() {
    let stock = fixture(30);
    let movement = StockMovement::Adjustment { delta: -3, reason: "damage".to_string() };
    let result = stock.apply(&movement).unwrap();
    assert_eq!(result.quantity, 27);
}

#[test]
fn adjustment_below_zero_errors() {
    let stock = fixture(2);
    let movement = StockMovement::Adjustment { delta: -3, reason: "damage".to_string() };
    let result = stock.apply(&movement);
    assert!(result.is_err());
}

#[test]
fn is_below_threshold_works() {
    let stock = fixture(5);
    assert!(stock.is_below_threshold(10));
    assert!(!stock.is_below_threshold(3));
}
