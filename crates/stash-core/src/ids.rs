#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct ItemId(pub uuid::Uuid);
impl Default for ItemId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl ItemId {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
impl From<ItemId> for String {
    fn from(id: ItemId) -> Self {
        id.0.to_string()
    }
}
impl TryFrom<String> for ItemId {
    type Error = uuid::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::parse_str(&s)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct WarehouseId(pub uuid::Uuid);
impl Default for WarehouseId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl WarehouseId {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
impl From<WarehouseId> for String {
    fn from(id: WarehouseId) -> Self {
        id.0.to_string()
    }
}
impl TryFrom<String> for WarehouseId {
    type Error = uuid::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::parse_str(&s)?))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct CategoryId(pub uuid::Uuid);
impl Default for CategoryId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
impl CategoryId {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
impl From<CategoryId> for String {
    fn from(id: CategoryId) -> Self {
        id.0.to_string()
    }
}
impl TryFrom<String> for CategoryId {
    type Error = uuid::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::parse_str(&s)?))
    }
}
