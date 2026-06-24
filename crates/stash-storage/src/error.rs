/// Errors surfaced by repository implementations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("record not found")]
    NotFound,

    #[error("database error: {0}")]
    Database(String),

    #[error(transparent)]
    Core(#[from] stash_core::CoreError),
}
