use crate::StorageError;
use stash_core::ids::WarehouseId;
use stash_core::warehouse::Warehouse;

#[async_trait::async_trait]
pub trait WarehouseRepository: Send + Sync {
    async fn create(&self, input: &CreateWarehouseInput) -> Result<Warehouse, StorageError>;
    async fn get(&self, id: WarehouseId) -> Result<Option<Warehouse>, StorageError>;
    async fn list(&self) -> Result<Vec<Warehouse>, StorageError>;
    async fn update(&self, input: &UpdateWarehouseInput) -> Result<Warehouse, StorageError>;
    async fn delete(&self, id: WarehouseId) -> Result<(), StorageError>;
}

#[derive(Debug, Clone)]
pub struct CreateWarehouseInput {
    pub id: WarehouseId,
    pub name: stash_core::warehouse::WarehouseName,
    pub location: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateWarehouseInput {
    pub id: WarehouseId,
    pub name: Option<stash_core::warehouse::WarehouseName>,
    pub location: Option<String>,
}
