#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Warehouse {
    pub id: crate::ids::WarehouseId,
    pub name: WarehouseName,
    pub location: Option<String>,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct WarehouseName(pub String);
impl WarehouseName {
    /// Parses a raw string into a validated warehouse name object.
    ///
    /// # Parameters
    /// - `raw`: A string slice representing the raw input.
    ///
    /// # Returns
    /// - `Result<Self, ValidationError>`:
    ///   - `Ok(Self)`: If the input string satisfies all validation rules.
    ///   - `Err(ValidationError)`: If validation fails, wrapping the specific error.
    ///
    /// # Validation Rules
    /// 1. The length of the input must be between 3 and 200 characters inclusive.
    ///
    /// # Errors
    /// This function returns the following validation errors:
    /// - `ValidationError::InvalidWarehouseNameLength`: If the input length is not within
    ///   the valid range.
    pub fn parse(raw: &str) -> Result<Self, crate::ValidationError> {
        if raw.len() < 3 || raw.len() > 200 {
            return Err(crate::ValidationError::InvalidWarehouseNameLength(raw.len()));
        }

        Ok(Self(raw.to_string()))
    }
}
impl From<WarehouseName> for String {
    fn from(raw: WarehouseName) -> Self {
        raw.0
    }
}
impl TryFrom<String> for WarehouseName {
    type Error = crate::ValidationError;
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Self::parse(&raw)
    }
}
