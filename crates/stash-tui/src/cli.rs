#[derive(Debug, clap::Parser)]
#[command(name = "stash", version, about = "Terminal Inventory Management System")]
pub struct Cli {
    /// Path to the SQLite database file. Overrides config.toml.
    #[arg(long)]
    pub db: Option<std::path::PathBuf>,

    /// Path to config.toml. Defaults to the OS config directory.
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Render tick rate in milliseconds. Overrides config.toml.
    #[arg(long)]
    pub tick_rate: Option<u64>,

    /// Import items from a CSV file, then exit without launching the TUI.
    #[arg(long)]
    pub import: Option<std::path::PathBuf>,

    /// Print a low-stock report to stdout, then exit without launching the TUI.
    #[arg(long)]
    pub headless_report: bool,
}
