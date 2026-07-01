use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Category {
    pub id: crate::ids::CategoryId,
    pub name: CategoryName,
    pub description: Option<String>,
    pub created_at: time::OffsetDateTime,
}
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Category {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let description: Option<String> = row.try_get("description")?;
        let created_at: time::OffsetDateTime = row.try_get("created_at")?;

        Ok(Self {
            id: crate::ids::CategoryId::try_from(id).map_err(decode_sqlx_err)?,
            name: CategoryName(name),
            description,
            created_at,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CategoryName(pub String);
impl CategoryName {
    /// Parses a raw string into a validated category name object.
    ///
    /// # Errors
    /// Returns `ValidationError::InvalidCategoryNameLength` if the name is not
    /// between 3 and 200 characters inclusive.
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

fn decode_sqlx_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> sqlx::Error {
    sqlx::Error::Decode(Box::new(e))
}
