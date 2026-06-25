/// Errors surfaced by repository implementations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("item not found: {0:?}")]
    NotFound(stash_core::ids::ItemId),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("database migration error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Core(#[from] stash_core::CoreError),

    #[error(transparent)]
    Validation(#[from] stash_core::ValidationError),
}
