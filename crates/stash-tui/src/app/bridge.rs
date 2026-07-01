use crate::app::{Command, Message};
use stash_core::item::ItemWithStock;
use std::sync::Arc;

pub struct StorageRepositories {
    pub item: Arc<dyn stash_storage::item_repository::ItemRepository>,
    pub movement: Arc<dyn stash_storage::movement_log_repository::MovementLogRepository>,
    pub stock: Arc<dyn stash_storage::stock_repository::StockRepository>,
    pub category: Arc<dyn stash_storage::category_repository::CategoryRepository>,
    pub warehouse: Arc<dyn stash_storage::warehouse_repository::WarehouseRepository>,
}

pub async fn spawn_storage_task(
    repos: StorageRepositories,
    mut cmd_rx: tokio::sync::mpsc::UnboundedReceiver<Command>,
    msg_tx: tokio::sync::mpsc::UnboundedSender<Message>,
) {
    while let Some(cmd) = cmd_rx.recv().await {
        let result_msg = match cmd {
            Command::FetchItems(filter) => match repos.item.list(filter).await {
                Ok(items) => Message::ItemsLoaded(items),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::SaveItem(input) => match repos.item.create(&input).await {
                Ok(item) => Message::ItemSaved(ItemWithStock::from_item(item)),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::UpdateItem(input) => match repos.item.update(&input).await {
                Ok(item) => Message::ItemUpdated(ItemWithStock::from_item(item)),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteItem(id) => match repos.item.delete(id).await {
                Ok(()) => Message::ItemDeleted(id),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::FetchMovements { item_id, limit, offset } => {
                let result = match item_id {
                    Some(id) => repos.movement.list_for_item(id, limit, offset).await,
                    None => repos.movement.list_recent(limit, offset).await,
                };
                match result {
                    Ok(records) => Message::MovementsLoaded(records),
                    Err(e) => Message::Error(e.to_string()),
                }
            }
            Command::RecordMovement { item_id, warehouse_id, movement } => {
                match repos.stock.apply_movement(item_id, warehouse_id, &movement).await {
                    Ok(level) => Message::StockUpdated(item_id, level.quantity),
                    Err(e) => Message::Error(e.to_string()),
                }
            }
            Command::FetchCategories => match repos.category.list().await {
                Ok(categories) => Message::CategoriesLoaded(categories),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::SaveCategory(input) => match repos.category.create(&input).await {
                Ok(c) => Message::CategorySaved(c),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::UpdateCategory(input) => match repos.category.update(&input).await {
                Ok(c) => Message::CategoryUpdated(c),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteCategory(id) => match repos.category.delete(id).await {
                Ok(()) => Message::CategoryDeleted(id),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::FetchWarehouses => match repos.warehouse.list().await {
                Ok(warehouses) => Message::WarehousesLoaded(warehouses),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::SaveWarehouse(input) => match repos.warehouse.create(&input).await {
                Ok(w) => Message::WarehouseSaved(w),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::UpdateWarehouse(input) => match repos.warehouse.update(&input).await {
                Ok(w) => Message::WarehouseUpdated(w),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteWarehouse(id) => match repos.warehouse.delete(id).await {
                Ok(()) => Message::WarehouseDeleted(id),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::None => continue,
        };
        let _ = msg_tx.send(result_msg);
    }
}
