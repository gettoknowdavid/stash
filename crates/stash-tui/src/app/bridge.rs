use crate::app::{Command, Message};
use std::sync::Arc;

pub async fn spawn_storage_task(
    item_repo: Arc<dyn stash_storage::item_repository::ItemRepository>,
    category_repo: Arc<dyn stash_storage::category_repository::CategoryRepository>,
    warehouse_repo: Arc<dyn stash_storage::warehouse_repository::WarehouseRepository>,
    movement_repo: Arc<dyn stash_storage::movement_log_repository::MovementLogRepository>,
    mut cmd_rx: tokio::sync::mpsc::UnboundedReceiver<Command>,
    msg_tx: tokio::sync::mpsc::UnboundedSender<Message>,
) {
    while let Some(cmd) = cmd_rx.recv().await {
        let result_msg = match cmd {
            Command::FetchItems(filter) => match item_repo.list(filter).await {
                Ok(items) => Message::ItemsLoaded(items),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::SaveItem(input) => match item_repo.create(&input).await {
                Ok(item) => Message::ItemSaved(stash_core::item::ItemWithStock::from_item(item)),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::UpdateItem(input) => match item_repo.update(&input).await {
                Ok(item) => Message::ItemSaved(stash_core::item::ItemWithStock::from_item(item)),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteItem(id) => match item_repo.delete(id).await {
                Ok(()) => Message::ItemsLoaded(vec![]),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::FetchCategories => match category_repo.list().await {
                Ok(categories) => Message::CategoriesLoaded(categories),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::SaveCategory(input) => match category_repo.create(&input).await {
                Ok(category) => Message::CategorySaved(category),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::UpdateCategory(input) => match category_repo.update(&input).await {
                Ok(category) => Message::CategorySaved(category),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteCategory(id) => match category_repo.delete(id).await {
                Ok(()) => Message::CategoriesLoaded(vec![]),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::FetchWarehouses => match warehouse_repo.list().await {
                Ok(warehouses) => Message::WarehousesLoaded(warehouses),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::SaveWarehouse(input) => match warehouse_repo.create(&input).await {
                Ok(warehouse) => Message::WarehouseSaved(warehouse),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::UpdateWarehouse(input) => match warehouse_repo.update(&input).await {
                Ok(warehouse) => Message::WarehouseSaved(warehouse),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteWarehouse(id) => match warehouse_repo.delete(id).await {
                Ok(()) => Message::WarehousesLoaded(vec![]),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::FetchMovements { item_id, limit, offset } => {
                let result = match item_id {
                    Some(id) => movement_repo.list_for_item(id, limit, offset).await,
                    None => movement_repo.list_recent(limit, offset).await,
                };
                match result {
                    Ok(records) => Message::MovementsLoaded(records),
                    Err(e) => Message::Error(e.to_string()),
                }
            }
            Command::RecordMovement { item_id, warehouse_id, movement } => {
                match movement_repo.record(item_id, warehouse_id, &movement).await {
                    Ok(_) => Message::None,
                    Err(e) => Message::Error(e.to_string()),
                }
            }
            Command::None => continue,
        };
        let _ = msg_tx.send(result_msg);
    }
}
