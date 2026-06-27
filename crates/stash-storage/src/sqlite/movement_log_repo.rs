use crate::movement_log_repository::MovementLogRepository;
use crate::StorageError;
use stash_core::ids::{ItemId, MovementId, WarehouseId};
use stash_core::stock::{StockMovement, StockMovementRecord};

pub struct MovementLogRepo {
    db: sqlx::sqlite::SqlitePool,
}
impl MovementLogRepo {
    #[must_use]
    pub const fn new(db: sqlx::sqlite::SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl MovementLogRepository for MovementLogRepo {
    async fn record(
        &self,
        item_id: ItemId,
        warehouse_id: WarehouseId,
        movement: &StockMovement,
    ) -> Result<StockMovementRecord, StorageError> {
        let id = MovementId::new();
        let created_at = time::OffsetDateTime::now_utc();
        sqlx::query(
            r"INSERT INTO stock_movements (id, item_id, warehouse_id, kind, qty_delta, reason, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id.0.to_string())
        .bind(item_id.0.to_string())
        .bind(warehouse_id.0.to_string())
        .bind(movement.kind_str())
        .bind(movement.quantity_delta())
        .bind(movement.reason())
        .bind(created_at)
        .execute(&self.db)
        .await?;
        Ok(StockMovementRecord {
            id,
            item_id,
            warehouse_id,
            movement: movement.clone(),
            created_at,
        })
    }

    async fn list_for_item(
        &self,
        item_id: ItemId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<StockMovementRecord>, StorageError> {
        let rows = sqlx::query_as::<_, MovementRow>(
            r"SELECT * FROM stock_movements
              WHERE item_id = ?
              ORDER BY created_at DESC
              LIMIT ? OFFSET ?",
        )
        .bind(String::from(item_id))
        .bind(i64::from(limit))
        .bind(i64::from(offset))
        .fetch_all(&self.db)
        .await?;

        rows.into_iter().map(StockMovementRecord::try_from).collect()
    }

    async fn list_recent(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<StockMovementRecord>, StorageError> {
        let rows = sqlx::query_as::<_, MovementRow>(
            r"SELECT * FROM stock_movements ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(i64::from(limit))
        .bind(i64::from(offset))
        .fetch_all(&self.db)
        .await?;

        rows.into_iter().map(StockMovementRecord::try_from).collect()
    }
}

#[derive(sqlx::FromRow)]
struct MovementRow {
    id: String,
    item_id: String,
    warehouse_id: String,
    kind: String,
    qty_delta: i64,
    reason: Option<String>,
    created_at: time::OffsetDateTime,
}
impl TryFrom<MovementRow> for StockMovementRecord {
    type Error = StorageError;

    fn try_from(row: MovementRow) -> Result<Self, Self::Error> {
        let movement = StockMovement::from_parts(&row.kind, row.qty_delta, row.reason)?;
        Ok(Self {
            id: MovementId::try_from(row.id).map_err(|_| {
                StorageError::Database(sqlx::Error::Decode("invalid movement id".into()))
            })?,
            item_id: ItemId::try_from(row.item_id).map_err(|_| {
                StorageError::Database(sqlx::Error::Decode("invalid item id".into()))
            })?,
            warehouse_id: WarehouseId::try_from(row.warehouse_id).map_err(|_| {
                StorageError::Database(sqlx::Error::Decode("invalid warehouse id".into()))
            })?,
            movement,
            created_at: row.created_at,
        })
    }
}
