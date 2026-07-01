use crate::stock_repository::StockRepository;
use crate::StorageError;
use stash_core::ids::{ItemId, MovementId, WarehouseId};
use stash_core::stock::{StockLevel, StockMovement};

pub struct StockRepo {
    db: sqlx::sqlite::SqlitePool,
}
impl StockRepo {
    #[must_use]
    pub const fn new(db: sqlx::sqlite::SqlitePool) -> Self {
        Self { db }
    }
}

#[derive(sqlx::FromRow)]
struct StockRow {
    item_id: String,
    warehouse_id: String,
    quantity: i64,
}
impl TryFrom<StockRow> for StockLevel {
    type Error = StorageError;
    fn try_from(row: StockRow) -> Result<Self, Self::Error> {
        Ok(Self {
            item_id: ItemId::try_from(row.item_id).map_err(|_| {
                StorageError::Database(sqlx::Error::Decode("invalid item id".into()))
            })?,
            warehouse_id: WarehouseId::try_from(row.warehouse_id).map_err(|_| {
                StorageError::Database(sqlx::Error::Decode("invalid warehouse id".into()))
            })?,
            // quantity is CHECK(>= 0) at the DB level, so this is safe.
            quantity: u32::try_from(row.quantity).unwrap_or(0),
        })
    }
}

#[async_trait::async_trait]
impl StockRepository for StockRepo {
    async fn get(
        &self,
        item_id: ItemId,
        warehouse_id: WarehouseId,
    ) -> Result<Option<StockLevel>, StorageError> {
        let row = sqlx::query_as::<_, StockRow>(
            "SELECT item_id, warehouse_id, quantity
            FROM stock_levels
            WHERE item_id = ? AND warehouse_id = ?",
        )
        .bind(String::from(item_id))
        .bind(String::from(warehouse_id))
        .fetch_optional(&self.db)
        .await?;
        row.map(StockLevel::try_from).transpose()
    }

    //noinspection ALL
    async fn list_for_item(&self, item_id: ItemId) -> Result<Vec<StockLevel>, StorageError> {
        let rows = sqlx::query_as::<_, StockRow>(
            "SELECT item_id, warehouse_id, quantity FROM stock_levels WHERE item_id = ?",
        )
        .bind(String::from(item_id))
        .fetch_all(&self.db)
        .await?;
        rows.into_iter().map(StockLevel::try_from).collect()
    }

    async fn apply_movement(
        &self,
        item_id: ItemId,
        warehouse_id: WarehouseId,
        movement: &StockMovement,
    ) -> Result<StockLevel, StorageError> {
        let mut tx = self.db.begin().await?;

        let current: Option<StockRow> = sqlx::query_as(
            "SELECT item_id, warehouse_id, quantity FROM stock_levels
            WHERE item_id = ? AND warehouse_id = ?",
        )
        .bind(String::from(item_id))
        .bind(String::from(warehouse_id))
        .fetch_optional(&mut *tx)
        .await?;

        let current_level = match current {
            Some(row) => StockLevel::try_from(row)?,
            None => StockLevel { item_id, warehouse_id, quantity: 0 },
        };

        // Domain rule enforcement (no negative stock, checked arithmetic) lives in
        // stash-core and is reused here untouched — the storage layer never re-implements it.
        let updated = current_level.apply(movement)?;

        sqlx::query(
            "INSERT INTO stock_levels (item_id, warehouse_id, quantity) VALUES (?, ?, ?)
            ON CONFLICT(item_id, warehouse_id) DO UPDATE SET quantity = excluded.quantity",
        )
        .bind(String::from(item_id))
        .bind(String::from(warehouse_id))
        .bind(i64::from(updated.quantity))
        .execute(&mut *tx)
        .await?;

        let id = MovementId::new();
        sqlx::query(
            "INSERT INTO stock_movements (id, item_id, warehouse_id, kind, qty_delta, reason)
            VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(id.0.to_string())
        .bind(String::from(item_id))
        .bind(String::from(warehouse_id))
        .bind(movement.kind_str())
        .bind(movement.quantity_delta())
        .bind(movement.reason())
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(updated)
    }
}
