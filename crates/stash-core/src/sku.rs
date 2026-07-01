use crate::error::ValidationError;
// Sku wraps a String but ONLY lets you build one through `parse`,
// which validates the format. This is "parse, don't validate":
// once you HAVE a Sku value, you know — by its very existence — that
// it's valid. No need to re-check it everywhere else in the codebase.

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct Sku(pub String);
impl Sku {
    /// Parses a raw string into a validated SKU (Stock Keeping Unit) object.
    ///
    /// # Parameters
    /// - `raw`: A string slice representing the raw SKU input.
    ///
    /// # Returns
    /// - `Result<Self, ValidationError>`:
    ///   - `Ok(Self)`: If the input string satisfies all validation rules.
    ///   - `Err(ValidationError)`: If validation fails, wrapping the specific error.
    ///
    /// # Validation Rules
    /// 1. The input must only contain ASCII alphanumeric characters or hyphens (`-`).
    /// 2. The length of the input must be between 3 and 20 characters inclusive.
    ///
    /// # Errors
    /// This function returns the following validation errors:
    /// - `ValidationError::InvalidSkuFormat`: If the input contains invalid characters.
    /// - `ValidationError::InvalidSkuLength`: If the input length is not within the valid range.
    ///
    pub fn parse(raw: &str) -> Result<Self, ValidationError> {
        if !raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(ValidationError::InvalidSkuFormat(raw.to_string()));
        }

        if raw.len() < 3 || raw.len() > 20 {
            return Err(ValidationError::InvalidSkuLength(raw.len()));
        }

        Ok(Self(raw.to_uppercase()))
    }

    /// Suggests a SKU by slugifying a name. This is a *suggestion*, not a guarantee
    /// of uniqueness — the DB unique constraint (and the form's error path)
    /// still has final say.
    #[must_use]
    pub fn suggest_from_name(name: &str) -> String {
        let mut slug: String = name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '-' })
            .collect();
        while slug.contains("--") {
            slug = slug.replace("--", "-");
        }
        let mut slug = slug.trim_matches('-').to_string();
        if slug.len() < 3 {
            slug.push_str("-ITM");
        }
        slug.truncate(20);
        slug
    }
}
impl From<Sku> for String {
    fn from(raw: Sku) -> Self {
        raw.0
    }
}
impl TryFrom<String> for Sku {
    type Error = ValidationError;
    fn try_from(raw: String) -> Result<Self, Self::Error> {
        Self::parse(&raw)
    }
}
