use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Warehouse {
    pub id: crate::ids::WarehouseId,
    pub name: WarehouseName,
    pub location: Option<String>,
}
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Warehouse {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let location: Option<String> = row.try_get("location")?;

        Ok(Self {
            id: crate::ids::WarehouseId::try_from(id).map_err(decode_sqlx_err)?,
            name: WarehouseName(name),
            location,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WarehouseName(pub String);
impl WarehouseName {
    /// # Errors
    /// Returns `ValidationError::InvalidWarehouseNameLength` if the name is not
    /// between 3 and 200 characters inclusive.
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

fn decode_sqlx_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> sqlx::Error {
    sqlx::Error::Decode(Box::new(e))
}
