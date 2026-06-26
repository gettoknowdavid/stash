use crate::error::ValidationError;
use crate::ids::{CategoryId, ItemId};
use crate::money::Money;
use crate::sku::Sku;
use sqlx::Row;

#[derive(Clone, Debug)]
pub struct Item {
    pub id: ItemId,
    pub sku: Sku,
    pub name: String,
    pub description: Option<String>,
    pub category_id: CategoryId,
    pub unit_cost: Money,
    pub reorder_threshold: i64,
}
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Item {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let sku: String = row.try_get("sku")?;
        let name: String = row.try_get("name")?;
        let description: Option<String> = row.try_get("description")?;
        let c_id: String = row.try_get("category_id")?;
        let unit_cost: i64 = row.try_get("unit_cost")?;
        let reorder_threshold: i64 = row.try_get("reorder_threshold")?;

        Ok(Self {
            id: ItemId::try_from(id).map_err(decode_sqlx_err)?,
            sku: Sku::try_from(sku).map_err(decode_sqlx_err)?,
            name,
            description,
            category_id: CategoryId::try_from(c_id).map_err(decode_sqlx_err)?,
            unit_cost: Money(unit_cost),
            reorder_threshold,
        })
    }
}

pub struct ItemBuilder {
    sku: Option<Sku>,
    name: Option<String>,
    unit_cost: Option<Money>,
    reorder_threshold: i64,
}
impl Default for ItemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl ItemBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self { sku: None, name: None, unit_cost: None, reorder_threshold: 0 }
    }

    #[must_use]
    pub fn sku(mut self, sku: Sku) -> Self {
        self.sku = Some(sku);
        self
    }

    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub const fn unit_cost(mut self, unit_cost: Money) -> Self {
        self.unit_cost = Some(unit_cost);
        self
    }

    /// Attempts to build an `Item` instance from the provided builder object.
    ///
    /// # Returns
    /// - `Ok(Item)` if all required fields (`sku`, `name`, and `unit_cost`) are present and valid.
    /// - `Err(ValidationError)` if any of the required fields are missing.
    ///
    /// # Errors
    /// - Returns `ValidationError::MissingField` with the name of the missing field if one
    ///   of the required fields (`sku`, `name`, or `unit_cost`) is not provided.
    ///
    /// # Fields
    /// - `sku`: The stock-keeping unit identifier of the item.
    /// - `name`: The name of the item.
    /// - `unit_cost`: The unit cost of the item.
    /// - `reorder_threshold` (optional): The threshold quantity for reordering the item.
    ///   Defaults to `None` if not specified.
    ///
    /// # Item Defaults
    /// - `id`: Automatically assigned with a default value from `ItemId::default()`.
    /// - `description`: Initialized as an empty `String`.
    /// - `category`: Defaulted to `Category::Other("uncategorized")`.
    ///
    /// In this example, if any of the required fields (`sku`, `name`, or `unit_cost`) are not set,
    /// `ValidationError` will be returned describing the missing field.
    pub fn build(self) -> Result<Item, ValidationError> {
        let sku = self.sku.ok_or(ValidationError::MissingField("sku"))?;
        let name = self.name.ok_or(ValidationError::MissingField("name"))?;
        let unit_cost = self.unit_cost.ok_or(ValidationError::MissingField("unit_cost"))?;

        Ok(Item {
            id: ItemId::new(),
            sku,
            name,
            description: None,
            category_id: CategoryId::new(),
            unit_cost,
            reorder_threshold: self.reorder_threshold,
        })
    }
}

#[derive(Default)]
pub struct ItemFilter {
    pub category_id: Option<CategoryId>,
    pub below_threshold_only: bool,
    pub search_text: Option<String>,
    pub sku_prefix: Option<String>,

    pub limit: u32,
    pub offset: u32,
}

fn decode_sqlx_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> sqlx::Error {
    sqlx::Error::Decode(Box::new(e))
}
