use crate::app::bridge::spawn_storage_task;
use crate::app::{App, Command, Message};
use crate::terminal::{init_panic_hook, init_terminal, restore_terminal};
use stash_core::category::CategoryName;
use stash_core::ids::CategoryId;
use stash_core::item::ItemFilter;
use stash_core::warehouse::WarehouseName;
use stash_storage::category_repository::{CategoryRepository, CreateCategoryInput};
use stash_storage::item_repository::ItemRepository;
use stash_storage::movement_log_repository::MovementLogRepository;
use stash_storage::sqlite::category_repo::CategoryRepo;
use stash_storage::sqlite::item_repo::ItemRepo;
use stash_storage::sqlite::movement_log_repo::MovementLogRepo;
use stash_storage::sqlite::stock_repo::StockRepo;
use stash_storage::sqlite::warehouse_repo::WarehouseRepo;
use stash_storage::stock_repository::StockRepository;
use stash_storage::warehouse_repository::{CreateWarehouseInput, WarehouseRepository};
use std::sync::Arc;

pub mod app;
pub mod terminal;
pub mod ui;

/// First-run convenience: an inventory app with zero categories or warehouses can't do
/// anything (item creation needs a category, stock adjustment needs a warehouse). Rather
/// than forcing manual setup before the TUI is usable, seed one of each if none exist.
async fn ensure_defaults(
    category_repo: &dyn CategoryRepository,
    warehouse_repo: &dyn WarehouseRepository,
) -> anyhow::Result<()> {
    if category_repo.list().await?.is_empty() {
        category_repo
            .create(&CreateCategoryInput {
                id: CategoryId::new(),
                name: CategoryName::parse("General")?,
                description: Some("Default category".into()),
            })
            .await?;
    }

    if warehouse_repo.list().await?.is_empty() {
        warehouse_repo
            .create(&CreateWarehouseInput {
                id: stash_core::ids::WarehouseId::new(),
                name: WarehouseName::parse("Main")?,
                location: None,
            })
            .await?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_panic_hook();
    let mut terminal = init_terminal()?;

    let runtime = tokio::runtime::Runtime::new()?;
    let db = runtime.block_on(stash_storage::sqlite::connect("stash.db"))?;

    let item_repo: Arc<dyn ItemRepository> = Arc::new(ItemRepo::new(db.clone()));
    let movement_repo: Arc<dyn MovementLogRepository> = Arc::new(MovementLogRepo::new(db.clone()));
    let stock_repo: Arc<dyn StockRepository> = Arc::new(StockRepo::new(db.clone()));
    let category_repo: Arc<dyn CategoryRepository> = Arc::new(CategoryRepo::new(db.clone()));
    let warehouse_repo: Arc<dyn WarehouseRepository> = Arc::new(WarehouseRepo::new(db.clone()));

    runtime.block_on(ensure_defaults(category_repo.as_ref(), warehouse_repo.as_ref()))?;

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let handle = tokio::spawn(async move {
                spawn_storage_task(
                    item_repo,
                    movement_repo,
                    stock_repo,
                    category_repo,
                    warehouse_repo,
                    cmd_rx,
                    msg_tx,
                )
                .await;
            });
            let _ = handle.await;
        });
    });

    let mut app = App::new();
    cmd_tx.send(Command::FetchItems(ItemFilter::default()))?;
    cmd_tx.send(Command::FetchCategories)?;
    cmd_tx.send(Command::FetchWarehouses)?;
    cmd_tx.send(Command::FetchMovements { item_id: None, limit: 20, offset: 0 })?;

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if let Some(cmd) = app.update(Message::KeyPressed(key)) {
                    cmd_tx.send(cmd)?;
                }
            }
        }

        // drain any pending async results without blocking
        while let Ok(msg) = msg_rx.try_recv() {
            if let Some(cmd) = app.update(msg) {
                cmd_tx.send(cmd)?;
            }
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal()?;
    Ok(())
}
