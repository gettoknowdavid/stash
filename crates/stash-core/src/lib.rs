#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Allow a few pedantic lints that are net-negative for this domain:
#![allow(clippy::module_name_repetitions)]

pub mod category;
mod error;
pub mod ids;
pub mod item;
pub mod money;
pub mod sku;
pub mod stock;
pub mod warehouse;

pub use error::CoreError;
pub use error::ValidationError;
