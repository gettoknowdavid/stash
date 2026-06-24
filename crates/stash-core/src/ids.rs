#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(uuid::Uuid);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WarehouseId(uuid::Uuid);
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
