use crate::repository::{CreateItemInput, ItemRepository, UpdateItemInput};
use crate::StorageError;
use stash_core::ids::{CategoryId, ItemId};
use stash_core::item::{Item, ItemFilter};
use stash_core::money::Money;
use stash_core::sku::Sku;

pub struct SqliteItemRepository {
    pool: sqlx::sqlite::SqlitePool,
}

#[async_trait::async_trait]
impl ItemRepository for SqliteItemRepository {
    async fn create(&self, input: &CreateItemInput<'_>) -> Result<Item, StorageError> {
        let row = sqlx::query!(
            r"INSERT INTO items (id, sku, category_id, name, unit_cost, reorder_threshold)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *",
            input.id.0,
            input.sku.0,
            input.category_id.0,
            input.name,
            input.unit_cost.0,
            input.reorder_threshold,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Item {
            id: input.id,
            sku: Sku::parse(&row.sku)?,
            name: row.name,
            description: row.description,
            category_id: CategoryId::from(row.category_id),
            unit_cost: Money(row.unit_cost),
            reorder_threshold: row.reorder_threshold,
        })
    }

    async fn get(&self, id: ItemId) -> Result<Option<Item>, StorageError> {
        let row = sqlx::query!(r"SELECT * FROM items WHERE id == ?1", id.0,)
            .fetch_optional(&self.pool)
            .await
            .map_err(StorageError::Database)?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(Item {
            id,
            sku: Sku::parse(&row.sku)?,
            name: row.name,
            description: row.description,
            category_id: CategoryId::from(row.category_id),
            unit_cost: Money(row.unit_cost),
            reorder_threshold: row.reorder_threshold,
        }))
    }

    async fn list(&self, filter: ItemFilter) -> Result<Vec<Item>, StorageError> {
        let mut qb = sqlx::QueryBuilder::new("SELECT * FROM items");

        if let Some(category_id) = &filter.category_id {
            qb.push(" AND category_id = ").push_bind(category_id.0);
        }

        if filter.below_threshold_only {
            qb.push(" AND quantity < reorder_threshold");
        }

        if let Some(text) = &filter.search_text {
            let search = format!("%{}%", text.trim().to_lowercase());
            qb.push(" AND (LOWER(name) LIKE ")
                .push_bind(&search)
                .push(" OR LOWER(COALESCE(description, '')) LIKE ")
                .push_bind(&search)
                .push(")");
        }

        qb.push(" LIMIT ")
            .push_bind(i64::from(filter.limit))
            .push(" OFFSET ")
            .push_bind(i64::from(filter.offset));

        let rows = qb.build_query_as::<Item>().fetch_all(&self.pool).await?;
        Ok(rows)
    }

    async fn update(&self, id: ItemId, input: &UpdateItemInput<'_>) -> Result<(), StorageError> {
        let mut qb = sqlx::QueryBuilder::new("UPDATE items SET updated_at = now()");
        if let Some(name) = input.name {
            qb.push(", name =").push_bind(name);
        }
        if let Some(description) = input.description {
            qb.push(", description = ").push_bind(description);
        }
        if let Some(category_id) = input.category_id {
            qb.push(", category_id = ").push_bind(category_id.0);
        }
        if let Some(unit_cost) = &input.unit_cost {
            qb.push(", unit_cost = ").push_bind(unit_cost.0);
        }
        if let Some(reorder_threshold) = input.reorder_threshold {
            qb.push(", reorder_threshold = ").push_bind(reorder_threshold);
        }

        qb.push(" WHERE id == ").push_bind(id.0);
        qb.push(" RETURNING *");

        qb.build().execute(&self.pool).await?;
        Ok(())
    }

    async fn delete(&self, id: ItemId) -> Result<(), StorageError> {
        sqlx::query!("DELETE FROM items WHERE id == ?", id.0).execute(&self.pool).await?;
        Ok(())
    }
}
