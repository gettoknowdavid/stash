use crate::warehouse_repository::{CreateWarehouseInput, UpdateWarehouseInput};
use crate::StorageError;
use stash_core::ids::WarehouseId;
use stash_core::warehouse::Warehouse;

#[derive(Clone, Debug)]
pub struct WarehouseRepo {
    db: sqlx::sqlite::SqlitePool,
}
impl WarehouseRepo {
    #[must_use]
    pub const fn new(db: sqlx::sqlite::SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl crate::warehouse_repository::WarehouseRepository for WarehouseRepo {
    async fn create(&self, input: &CreateWarehouseInput) -> Result<Warehouse, StorageError> {
        sqlx::query_as::<_, Warehouse>(
            r"INSERT INTO warehouses (id, name, location)
            VALUES (?, ?, ?)
            RETURNING *",
        )
        .bind(String::from(input.id))
        .bind(input.name.0.as_str())
        .bind(input.location.as_deref())
        .fetch_one(&self.db)
        .await
        .map_err(StorageError::Database)
    }

    //noinspection SqlType
    async fn get(&self, id: WarehouseId) -> Result<Option<Warehouse>, StorageError> {
        sqlx::query_as::<_, Warehouse>(r"SELECT * FROM warehouses WHERE id = ?")
            .bind(String::from(id))
            .fetch_optional(&self.db)
            .await
            .map_err(StorageError::Database)
    }

    //noinspection ALL
    async fn list(&self) -> Result<Vec<Warehouse>, StorageError> {
        sqlx::query_as::<_, Warehouse>(r"SELECT * FROM warehouses WHERE 1 = 1")
            .fetch_all(&self.db)
            .await
            .map_err(StorageError::Database)
    }

    async fn update(&self, input: &UpdateWarehouseInput) -> Result<Warehouse, StorageError> {
        let mut qb = sqlx::QueryBuilder::new("UPDATE warehouses SET ");
        if let Some(name) = &input.name {
            qb.push(" name = ").push_bind(name);
        }
        if let Some(location) = &input.location {
            qb.push(" location = ").push_bind(location);
        }
        qb.push(" WHERE id = ").push_bind(input.id.0.to_string());
        qb.push(" RETURNING *");
        let row = qb.build_query_as::<Warehouse>().fetch_one(&self.db).await?;
        Ok(row)
    }

    //noinspection SqlType
    async fn delete(&self, id: WarehouseId) -> Result<(), StorageError> {
        // let is_used: bool =
        //     sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM items WHERE category_id = ?)")
        //         .bind(id.0.to_string())
        //         .fetch_one(&self.db)
        //         .await?;
        // if is_used {
        //     return Err(StorageError::WarehouseInUse(id));
        // }

        sqlx::query("DELETE FROM warehouses WHERE id = ?")
            .bind(id.0.to_string())
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
