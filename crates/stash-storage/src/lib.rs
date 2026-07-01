#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod category_repository;
pub mod error;
pub mod item_repository;
pub mod movement_log_repository;
pub mod sqlite;
pub mod stock_repository;
pub mod warehouse_repository;

pub use error::StorageError;
