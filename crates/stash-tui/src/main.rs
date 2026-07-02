use crate::app::bridge::{spawn_storage_task, StorageRepositories};
use crate::app::{App, Command, Message};
use crate::cli::Cli;
use crate::config::{Config, ThemeChoice};
use crate::terminal::{init_panic_hook, init_terminal, restore_terminal};
use clap::Parser;
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
pub mod cli;
pub mod config;
pub mod import;
pub mod logging;
pub mod terminal;
pub mod theme;
pub mod ui;

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

/// Epic 7.3-adjacent: a plain-text low-stock report for `--headless-report`.
async fn print_low_stock_report(item_repo: &dyn ItemRepository) -> anyhow::Result<()> {
    let filter = ItemFilter { below_threshold_only: true, ..Default::default() };
    let low_stock = item_repo.list(filter).await?;
    if low_stock.is_empty() {
        println!("No items below their reorder threshold.");
        return Ok(());
    }
    println!("{:<20} {:<30} {:>10} {:>10}", "SKU", "Name", "Qty", "Threshold");
    for entry in low_stock {
        println!(
            "{:<20} {:<30} {:>10} {:>10}",
            entry.item.sku.0, entry.item.name, entry.qty, entry.item.reorder_threshold
        );
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load(cli.config.as_deref())?;

    // If we resolved a config path via the OS default (not --config) and no
    // file exists yet, seed one so the user has something to edit (Ticket 6.1).
    if cli.config.is_none() {
        if let Some(default_path) = Config::default_config_path() {
            Config::ensure_default_file(&default_path)?;
        }
    }

    // CLI flags override config.toml, which overrides built-in defaults.
    if let Some(db) = &cli.db {
        config.db_path = db.to_string_lossy().to_string();
    }
    if let Some(tick_rate) = cli.tick_rate {
        config.tick_rate_ms = tick_rate;
    }

    let runtime = tokio::runtime::Runtime::new()?;
    let db = runtime.block_on(stash_storage::sqlite::connect(&config.db_path))?;

    let item_repo: Arc<dyn ItemRepository> = Arc::new(ItemRepo::new(db.clone()));
    let category_repo: Arc<dyn CategoryRepository> = Arc::new(CategoryRepo::new(db.clone()));
    let warehouse_repo: Arc<dyn WarehouseRepository> = Arc::new(WarehouseRepo::new(db.clone()));

    // --- Headless modes: no terminal, no panic hook, exit before the TUI ever starts. ---
    if let Some(import_path) = &cli.import {
        let summary = runtime.block_on(import::import_csv(
            import_path,
            item_repo.as_ref(),
            category_repo.as_ref(),
        ))?;
        println!("Imported {} item(s).", summary.imported);
        if !summary.skipped.is_empty() {
            println!("Skipped {} row(s):", summary.skipped.len());
            for (row, reason) in &summary.skipped {
                println!("  row {row}: {reason}");
            }
        }
        return Ok(());
    }
    if cli.headless_report {
        runtime.block_on(print_low_stock_report(item_repo.as_ref()))?;
        return Ok(());
    }

    // --- Normal TUI path ---
    let _log_guard = match Config::default_log_dir() {
        Some(dir) => Some(logging::init(&dir)?),
        None => None,
    };
    tracing::info!(db_path = %config.db_path, "starting stash-tui");

    init_panic_hook();
    let mut terminal = init_terminal()?;

    let movement_repo: Arc<dyn MovementLogRepository> = Arc::new(MovementLogRepo::new(db.clone()));
    let stock_repo: Arc<dyn StockRepository> = Arc::new(StockRepo::new(db.clone()));

    runtime.block_on(ensure_defaults(category_repo.as_ref(), warehouse_repo.as_ref()))?;

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let repos = StorageRepositories {
                item: item_repo,
                movement: movement_repo,
                stock: stock_repo,
                category: category_repo,
                warehouse: warehouse_repo,
            };
            spawn_storage_task(repos, cmd_rx, msg_tx).await;
        });
    });

    let mut app = App::new();
    app.theme = match config.theme {
        ThemeChoice::Dark => theme::Theme::dark(),
        ThemeChoice::Light => theme::Theme::light(),
    };
    cmd_tx.send(Command::FetchItems(ItemFilter::default()))?;
    cmd_tx.send(Command::FetchCategories)?;
    cmd_tx.send(Command::FetchWarehouses)?;
    cmd_tx.send(Command::FetchMovements { item_id: None, limit: 20, offset: 0 })?;

    let tick_rate = std::time::Duration::from_millis(config.tick_rate_ms);
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if crossterm::event::poll(tick_rate)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if let Some(cmd) = app.update(Message::KeyPressed(key)) {
                    cmd_tx.send(cmd)?;
                }
            }
        }

        while let Ok(msg) = msg_rx.try_recv() {
            if let Some(cmd) = app.update(msg) {
                cmd_tx.send(cmd)?;
            }
        }

        if app.should_quit {
            break;
        }
    }

    tracing::info!("shutting down");
    restore_terminal()?;
    Ok(())
}
