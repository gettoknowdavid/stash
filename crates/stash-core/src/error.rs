/// All errors that can originate from domain/business-rule violations.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid SKU: {0}")]
    InvalidSku(String),

    #[error("insufficient stock: requested {requested}, available {available}")]
    InsufficientStock { requested: u32, available: u32 },

    #[error("quantity overflow")]
    QuantityOverflow,

    #[error("invalid movement kind: {0}")]
    InvalidMovementKind(String),
}
impl CoreError {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::InvalidSku(s) => format!("'{s}' isn't a valid SKU."),
            Self::InsufficientStock { requested, available } => format!(
                "Not enough stock: you asked for {requested} but only {available} is available."
            ),
            Self::QuantityOverflow => "That quantity is too large.".to_string(),
            Self::InvalidMovementKind(_) => "Unrecognized stock movement type.".to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("missing required field: {0}")]
    MissingField(&'static str),

    #[error("invalid SKU format: {0}")]
    InvalidSkuFormat(String),

    #[error("invalid SKU length: {0}")]
    InvalidSkuLength(usize),

    #[error("invalid warehouse name length: {0}")]
    InvalidWarehouseNameLength(usize),

    #[error("invalid category name length: {0}")]
    InvalidCategoryNameLength(usize),
}
impl ValidationError {
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::MissingField(f) => format!("{f} is required."),
            Self::InvalidSkuFormat(_) => {
                "SKU can only contain letters, numbers, and hyphens.".to_string()
            }
            Self::InvalidSkuLength(len) => format!("SKU must be 3–20 characters (got {len})."),
            Self::InvalidWarehouseNameLength(len) => {
                format!("Warehouse name must be 3–200 characters (got {len}).")
            }
            Self::InvalidCategoryNameLength(len) => {
                format!("Category name must be 3–200 characters (got {len}).")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insufficient_stock_error_includes_quantities() {
        let err = CoreError::InsufficientStock { requested: 20, available: 2 };
        assert_eq!(err.to_string(), "insufficient stock: requested 20, available 2");
    }
}
