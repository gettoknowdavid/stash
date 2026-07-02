use tracing_appender::non_blocking::WorkerGuard;

/// Sets up file-based, non-blocking logging. Must run *before* entering raw
/// mode — a TUI app can't log to stdout while the alternate screen is up
/// without corrupting the display.
///
/// The returned `WorkerGuard` must be kept alive for the life of the
/// program; dropping it early stops the flush thread and log lines get lost.
///
/// # Errors
/// Returns an error if the log directory can't be created.
pub fn init(log_dir: &std::path::Path) -> anyhow::Result<WorkerGuard> {
    std::fs::create_dir_all(log_dir)?;
    let appender = tracing_appender::rolling::daily(log_dir, "stash-tui.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    Ok(guard)
}
