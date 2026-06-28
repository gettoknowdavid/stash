#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Allow a few pedantic lints that are net-negative for this domain:
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod item_repository;
pub mod movement_log_repository;
pub mod sqlite;
pub mod category_repository;
pub mod warehouse_repository;

pub use error::StorageError;
