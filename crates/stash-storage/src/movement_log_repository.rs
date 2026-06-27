use crate::StorageError;
use stash_core::ids::{ItemId, WarehouseId};
use stash_core::stock::{StockMovement, StockMovementRecord};

#[async_trait::async_trait]
pub trait MovementLogRepository: Send + Sync {
    async fn record(
        &self,
        item_id: ItemId,
        warehouse_id: WarehouseId,
        movement: &StockMovement,
    ) -> Result<StockMovementRecord, StorageError>;
    async fn list_for_item(
        &self,
        item_id: ItemId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<StockMovementRecord>, StorageError>;
    async fn list_recent(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<StockMovementRecord>, StorageError>;
}
