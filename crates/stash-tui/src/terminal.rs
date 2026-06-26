use crossterm::{cursor, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::io::Stdout;

pub fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub fn restore_terminal() -> anyhow::Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}

pub fn init_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        original(info);
    }));
}
