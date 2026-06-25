#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Allow a few pedantic lints that are net-negative for this domain:
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod repository;
pub mod sqlite;

pub use error::StorageError;
