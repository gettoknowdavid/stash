use stash_core::item::ItemWithStock;

pub async fn spawn_storage_task(
    repo: std::sync::Arc<dyn stash_storage::repository::ItemRepository>,
    mut cmd_rx: tokio::sync::mpsc::UnboundedReceiver<crate::app::Command>,
    msg_tx: tokio::sync::mpsc::UnboundedSender<crate::app::Message>,
) {
    while let Some(cmd) = cmd_rx.recv().await {
        let result_msg = match cmd {
            crate::app::Command::FetchItems(filter) => match repo.list(filter).await {
                Ok(items) => crate::app::Message::ItemsLoaded(items),
                Err(e) => crate::app::Message::Error(e.to_string()),
            },
            crate::app::Command::SaveItem(input) => match repo.create(&input).await {
                Ok(item) => crate::app::Message::ItemSaved(ItemWithStock::from_item(item)),
                Err(e) => crate::app::Message::Error(e.to_string()),
            },
            crate::app::Command::DeleteItem(id) => match repo.delete(id).await {
                Ok(()) => crate::app::Message::ItemsLoaded(vec![]),
                Err(e) => crate::app::Message::Error(e.to_string()),
            },
            crate::app::Command::None => continue,
        };
        let _ = msg_tx.send(result_msg);
    }
}
