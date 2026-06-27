use crate::app::{Command, Message};
use stash_core::item::ItemWithStock;
use std::sync::Arc;

pub async fn spawn_storage_task(
    item_repo: Arc<dyn stash_storage::item_repository::ItemRepository>,
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
                Ok(item) => Message::ItemSaved(ItemWithStock::from_item(item)),
                Err(e) => Message::Error(e.to_string()),
            },
            Command::DeleteItem(id) => match item_repo.delete(id).await {
                Ok(()) => Message::ItemsLoaded(vec![]),
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
