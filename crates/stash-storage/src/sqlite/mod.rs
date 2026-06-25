use crate::StorageError;
use std::str::FromStr;

pub mod item_repo;

///
/// Asynchronously establishes a connection to a `SQLite` database and applies any
/// pending migrations.
///
/// # Parameters
/// - `db_path`: A string slice representing the file path to the `SQLite` database.
///   If the file does not exist, it will be created automatically.
///
/// # Returns
/// - `Result<sqlx::sqlite::SqlitePool, StorageError>`:
///   - On success: An instance of `SqlitePool`, representing the connection pool to
///     the `SQLite` database.
///   - On error: A `StorageError` describing the failure.
///
/// # Behavior
/// - Configures the `SQLite` connection with the following settings:
///   - Creates the database file if it doesn't exist.
///   - Sets the `journal_mode` to "WAL" for better concurrency.
///   - Sets the `synchronous` pragma to "NORMAL" for faster write performance with
///     durability trade-offs.
///   - Enables support for enforcing foreign key constraints.
/// - Initializes a connection pool with a maximum of 5 connections.
/// - Runs pending database migrations located in the `./migrations` directory.
///
/// # Errors
/// - Returns a `StorageError` if:
///   - The database connection cannot be established.
///   - Migrating the database fails.
///   - The specified `db_path` is invalid or cannot be parsed.
///
/// # Notes
/// - Ensure the `./migrations` directory contains valid migration files that sqlx can process.
/// - The WAL (write-ahead logging) journal mode improves performance by allowing multiple
///   readers and a single writer simultaneously.
pub async fn connect(db_path: &str) -> Result<sqlx::sqlite::SqlitePool, StorageError> {
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(db_path)?
        .create_if_missing(true)
        .pragma("journal_mode", "WAL")
        .pragma("synchronous", "NORMAL")
        .pragma("foreign_keys", "ON");

    let pool =
        sqlx::sqlite::SqlitePoolOptions::new().max_connections(5).connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
