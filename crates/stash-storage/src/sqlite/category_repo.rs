use crate::category_repository::{CreateCategoryInput, UpdateCategoryInput};
use crate::StorageError;
use stash_core::category::Category;
use stash_core::ids::CategoryId;

#[derive(Clone, Debug)]
pub struct CategoryRepo {
    db: sqlx::sqlite::SqlitePool,
}
impl CategoryRepo {
    #[must_use]
    pub const fn new(db: sqlx::sqlite::SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl crate::category_repository::CategoryRepository for CategoryRepo {
    async fn create(&self, input: &CreateCategoryInput) -> Result<Category, StorageError> {
        sqlx::query_as::<_, Category>(
            r"INSERT INTO categories (id, name, description)
            VALUES (?, ?, ?)
            RETURNING *",
        )
        .bind(String::from(input.id))
        .bind(input.name.0.as_str())
        .bind(input.description.as_deref())
        .fetch_one(&self.db)
        .await
        .map_err(StorageError::Database)
    }

    //noinspection SqlType
    async fn get(&self, id: CategoryId) -> Result<Option<Category>, StorageError> {
        sqlx::query_as::<_, Category>(r"SELECT * FROM categories WHERE id = ?")
            .bind(String::from(id))
            .fetch_optional(&self.db)
            .await
            .map_err(StorageError::Database)
    }

    async fn list(&self) -> Result<Vec<Category>, StorageError> {
        sqlx::query_as::<_, Category>(r"SELECT * FROM categories")
            .fetch_all(&self.db)
            .await
            .map_err(StorageError::Database)
    }

    async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, StorageError> {
        let mut qb = sqlx::QueryBuilder::new("UPDATE categories SET ");
        if let Some(name) = &input.name {
            qb.push(" name = ").push_bind(name.0.as_str());
        }
        if let Some(description) = &input.description {
            qb.push(" description = ").push_bind(description);
        }
        qb.push(" WHERE id = ").push_bind(input.id.0.to_string());
        qb.push(" RETURNING *");
        let row = qb.build_query_as::<Category>().fetch_one(&self.db).await?;
        Ok(row)
    }

    //noinspection SqlType
    async fn delete(&self, id: CategoryId) -> Result<(), StorageError> {
        let is_used: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM items WHERE category_id = ?)")
                .bind(id.0.to_string())
                .fetch_one(&self.db)
                .await?;
        if is_used {
            return Err(StorageError::CategoryInUse(id));
        }

        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id.0.to_string())
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
