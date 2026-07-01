use crate::StorageError;
use stash_core::category::Category;
use stash_core::ids::CategoryId;

#[async_trait::async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, input: &CreateCategoryInput) -> Result<Category, StorageError>;
    async fn get(&self, id: CategoryId) -> Result<Option<Category>, StorageError>;
    async fn list(&self) -> Result<Vec<Category>, StorageError>;
    async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, StorageError>;
    async fn delete(&self, id: CategoryId) -> Result<(), StorageError>;
}

#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
    pub id: CategoryId,
    pub name: stash_core::category::CategoryName,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCategoryInput {
    pub id: CategoryId,
    pub name: Option<stash_core::category::CategoryName>,
    pub description: Option<String>,
}
