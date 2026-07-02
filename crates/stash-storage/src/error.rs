/// Errors surfaced by repository implementations.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("item not found: {0:?}")]
    NotFound(stash_core::ids::ItemId),

    #[error("Category is still in use: {0:?}")]
    CategoryInUse(stash_core::ids::CategoryId),

    #[error("Warehouse still has items in it: {0:?}")]
    WarehouseInUse(stash_core::ids::WarehouseId),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("database migration error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Core(#[from] stash_core::CoreError),

    #[error(transparent)]
    Validation(#[from] stash_core::ValidationError),
}
impl StorageError {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::NotFound(_) => {
                "That item does not exist or it may have been deleted.".to_string()
            }
            Self::CategoryInUse(_) => {
                "This category still has items in it. Move or delete those items first.".to_string()
            }
            Self::WarehouseInUse(_) => {
                "This warehouse still has stock in it. Clear its stock first.".to_string()
            }
            Self::Database(sqlx::Error::Database(db_error)) => {
                use sqlx::error::ErrorKind;
                match db_error.kind() {
                    ErrorKind::UniqueViolation => unique_violation_message(db_error.message()),
                    ErrorKind::ForeignKeyViolation => "That references something that no longer \
                        exists (maybe a category or warehouse was deleted). Try refreshing."
                        .to_string(),
                    ErrorKind::CheckViolation => {
                        "That value isn't allowed (e.g. stock can't go negative).".to_string()
                    }
                    _ => "Something went wrong somewhere. Please try again.".to_string(),
                }
            }
            Self::Database(sqlx::Error::PoolTimedOut) => {
                "The database is busy. Please try again in a moment.".to_string()
            }
            Self::Database(_) => "A database error occurred. Please try again.".to_string(),
            Self::MigrateError(_) => {
                "Failed to set up the database. Please restart the app.".to_string()
            }
            Self::Core(e) => e.message(),
            Self::Validation(e) => e.message(),
        }
    }
}

fn unique_violation_message(raw: &str) -> String {
    if raw.contains("items.sku") {
        "That SKU is already in use. Choose a different one.".to_string()
    } else if raw.contains("categories.name") {
        "A category with that name already exists.".to_string()
    } else if raw.contains("warehouses.name") {
        "A warehouse with that name already exists.".to_string()
    } else {
        "That value is already in use.".to_string()
    }
}
