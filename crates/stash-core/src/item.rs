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

#[derive(Debug, Clone)]
pub struct ItemFilter {
    pub category_id: Option<CategoryId>,
    pub below_threshold_only: bool,
    pub search_text: Option<String>,
    pub sku_prefix: Option<String>,
    pub limit: u32,
    pub offset: u32,
}
impl Default for ItemFilter {
    fn default() -> Self {
        Self {
            category_id: None,
            below_threshold_only: false,
            search_text: None,
            sku_prefix: None,
            limit: 500,
            offset: 0,
        }
    }
}
#[derive(Clone, Debug)]
pub struct ItemWithStock {
    pub item: Item,
    pub qty: i64,
}
impl ItemWithStock {
    #[must_use]
    pub const fn from_item(item: Item) -> Self {
        Self { item, qty: 0 }
    }
}
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for ItemWithStock {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            item: Item::from_row(row)?, // reuse Item's FromRow
            qty: row.try_get("total_quantity")?,
        })
    }
}

fn decode_sqlx_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> sqlx::Error {
    sqlx::Error::Decode(Box::new(e))
}
