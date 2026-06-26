pub mod app;
pub mod ui;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    run().context("stash exited with an error")
}

fn run() -> Result<()> {
    // Library errors (CoreError, StorageError) flow up through `?` here.
    // anyhow::Error implements From<E: std::error::Error>, and thiserror-derived
    // errors implement std::error::Error, so no glue code is needed.
    println!("stash: foundation laid");
    Ok(())
}
