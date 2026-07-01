#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Category {
    pub id: crate::ids::CategoryId,
    pub name: CategoryName,
    pub description: Option<String>,
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct CategoryName(pub String);
impl CategoryName {
    /// Parses a raw string into a validated category name object.
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
    /// - `ValidationError::InvalidCategoryNameLength`: If the input length is not within
    ///   the valid range.
    pub fn parse(raw: &str) -> Result<Self, crate::ValidationError> {
        if raw.len() < 3 || raw.len() > 200 {
            return Err(crate::ValidationError::InvalidCategoryNameLength(raw.len()));
        }

        Ok(Self(raw.to_string()))
    }
}
impl From<CategoryName> for String {
    fn from(raw: CategoryName) -> Self {
        raw.0
    }
}
impl TryFrom<String> for CategoryName {
    type Error = crate::ValidationError;
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Self::parse(&raw)
    }
}
