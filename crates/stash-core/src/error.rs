/// All errors that can originate from domain/business-rule violations.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid SKU: {0}")]
    InvalidSku(String),

    #[error("insufficient stock: requested {requested}, available {available}")]
    InsufficientStock { requested: u32, available: u32 },
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
