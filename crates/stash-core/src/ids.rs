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
impl From<String> for ItemId {
    fn from(s: String) -> Self {
        Self(uuid::Uuid::parse_str(&s).unwrap_or_default())
    }
}
impl From<Option<String>> for ItemId {
    fn from(s: Option<String>) -> Self {
        s.map_or_else(Self::new, |value| Self(uuid::Uuid::parse_str(&value).unwrap_or_default()))
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
impl From<String> for WarehouseId {
    fn from(s: String) -> Self {
        Self(uuid::Uuid::parse_str(&s).unwrap_or_default())
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
impl From<String> for CategoryId {
    fn from(s: String) -> Self {
        Self(uuid::Uuid::parse_str(&s).unwrap_or_default())
    }
}
