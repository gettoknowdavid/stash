use crate::app::bridge::spawn_storage_task;
use crate::app::{App, Command, Message};
use crate::terminal::{init_panic_hook, init_terminal, restore_terminal};
use stash_core::item::ItemFilter;
use stash_storage::item_repository::ItemRepository;
use stash_storage::movement_log_repository::MovementLogRepository;
use stash_storage::sqlite::item_repo::ItemRepo;
use stash_storage::sqlite::movement_log_repo::MovementLogRepo;
use std::sync::Arc;

pub mod app;
pub mod terminal;
pub mod ui;

fn main() -> anyhow::Result<()> {
    init_panic_hook();
    let mut terminal = init_terminal()?;

    let runtime = tokio::runtime::Runtime::new()?;
    let db = runtime.block_on(stash_storage::sqlite::connect("stash.db"))?;
    let item_repo: Arc<dyn ItemRepository> = Arc::new(ItemRepo::new(db.clone()));
    let movement_repo: Arc<dyn MovementLogRepository> = Arc::new(MovementLogRepo::new(db));

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let handle = tokio::spawn(async move {
                spawn_storage_task(item_repo, movement_repo, cmd_rx, msg_tx).await;
            });
            let _ = handle.await;
        });
    });

    let mut app = App::new();
    cmd_tx.send(Command::FetchItems(ItemFilter::default()))?;

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
            app.update(msg);
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal()?;
    Ok(())
}
