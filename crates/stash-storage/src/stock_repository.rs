use crate::StorageError;
use stash_core::ids::{ItemId, WarehouseId};
use stash_core::stock::{StockLevel, StockMovement};

#[async_trait::async_trait]
pub trait StockRepository: Sync + Send {
    async fn get(
        &self,
        item_id: ItemId,
        warehouse_id: WarehouseId,
    ) -> Result<Option<StockLevel>, StorageError>;

    async fn list_for_item(&self, item_id: ItemId) -> Result<Vec<StockLevel>, StorageError>;

    /// Applies a movement to the stock level for (item, warehouse) and records it in the
    /// audit log, atomically. This is the only path that should ever change `stock_levels` —
    /// it guarantees the level and the log never drift apart.
    async fn apply_movement(
        &self,
        item_id: ItemId,
        warehouse_id: WarehouseId,
        movement: &StockMovement,
    ) -> Result<StockLevel, StorageError>;
}
