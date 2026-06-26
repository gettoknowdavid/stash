use crate::repository::{CreateItemInput, ItemRepository, UpdateItemInput};
use crate::StorageError;
use stash_core::ids::ItemId;
use stash_core::item::{Item, ItemFilter};

pub struct SqliteItemRepository {
    pool: sqlx::sqlite::SqlitePool,
}

#[async_trait::async_trait]
impl ItemRepository for SqliteItemRepository {
    async fn create(&self, input: &CreateItemInput<'_>) -> Result<Item, StorageError> {
        let id_str: String = input.id.into();
        let cat_str: String = input.category_id.into();

        let row = sqlx::query_as::<_, Item>(
            r"INSERT INTO items (id, sku, name, description, category_id, unit_cost, reorder_threshold)
              VALUES (?, ?, ?, ?, ?, ?, ?)
              RETURNING *",
        )
            .bind(id_str)
            .bind(&input.sku.0)
            .bind(input.name)
            .bind(input.description)
            .bind(cat_str)
            .bind(input.unit_cost.0)
            .bind(i64::from(input.reorder_threshold))
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    //noinspection SqlType
    async fn get(&self, id: ItemId) -> Result<Option<Item>, StorageError> {
        let id_str: String = id.into();
        let row = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = ?")
            .bind(id_str)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn list(&self, filter: ItemFilter) -> Result<Vec<Item>, StorageError> {
        let mut qb = sqlx::QueryBuilder::new(
            "SELECT items.* FROM items \
             LEFT JOIN stock_levels ON stock_levels.item_id = items.id \
             WHERE 1 = 1",
        );

        if let Some(category_id) = &filter.category_id {
            qb.push(" AND items.category_id = ").push_bind(category_id.0.to_string());
        }

        if filter.below_threshold_only {
            qb.push(" AND stock_levels.quantity < items.reorder_threshold");
        }

        if let Some(text) = &filter.search_text {
            let search = format!("%{}%", text.trim().to_lowercase());
            qb.push(" AND (LOWER(items.name) LIKE ")
                .push_bind(search.clone())
                .push(" OR LOWER(COALESCE(items.description, '')) LIKE ")
                .push_bind(search)
                .push(")");
        }

        qb.push(" GROUP BY items.id");
        qb.push(" LIMIT ").push_bind(i64::from(filter.limit));
        qb.push(" OFFSET ").push_bind(i64::from(filter.offset));

        let rows = qb.build_query_as::<Item>().fetch_all(&self.pool).await?;
        Ok(rows)
    }

    async fn update(&self, id: ItemId, input: &UpdateItemInput<'_>) -> Result<Item, StorageError> {
        let mut qb = sqlx::QueryBuilder::new("UPDATE items SET updated_at = CURRENT_TIMESTAMP");

        if let Some(name) = input.name {
            qb.push(", name = ").push_bind(name);
        }
        if let Some(description) = input.description {
            qb.push(", description = ").push_bind(description);
        }
        if let Some(category_id) = input.category_id {
            qb.push(", category_id = ").push_bind(category_id.0.to_string());
        }
        if let Some(unit_cost) = &input.unit_cost {
            qb.push(", unit_cost = ").push_bind(unit_cost.0);
        }
        if let Some(reorder_threshold) = input.reorder_threshold {
            qb.push(", reorder_threshold = ").push_bind(i64::from(reorder_threshold));
        }

        qb.push(" WHERE id = ").push_bind(id.0.to_string());
        qb.push(" RETURNING *");

        let row = qb.build_query_as::<Item>().fetch_one(&self.pool).await?;
        Ok(row)
    }

    //noinspection SqlType
    async fn delete(&self, id: ItemId) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM items WHERE id = ?")
            .bind(id.0.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
