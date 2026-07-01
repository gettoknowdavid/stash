use crate::StorageError;
use stash_core::ids::{CategoryId, ItemId};
use stash_core::item::{Item, ItemFilter, ItemWithStock};
use stash_core::money::Money;
use stash_core::sku::Sku;

#[async_trait::async_trait]
pub trait ItemRepository: Send + Sync {
    /// Creates a new item in the storage.
    ///
    /// # Parameters
    /// - `item`: An instance of `NewItem` containing the data for the item to be created.
    ///
    /// # Returns
    /// - `Result<Item, StorageError>`:
    ///     - `Ok(Item)`: The newly created item if the operation is successful.
    ///     - `Err(StorageError)`: An error if the operation fails, such as due to storage issues.
    ///
    /// # Errors
    /// This function returns a `StorageError` in the event of issues like:
    /// - Database connection errors.
    /// - Duplicate entries violating unique constraints.
    /// - General storage-related failures.
    ///
    /// This function is part of a storage interface and is expected to persist
    /// the provided `NewItem` to the underlying datastore.
    async fn create(&self, input: &CreateItemInput) -> Result<Item, StorageError>;

    /// Retrieves an item from the storage by its unique identifier.
    ///
    /// # Arguments
    /// * `id` - The identifier of the item to be retrieved. This is of type `ItemId`.
    ///
    /// # Returns
    /// * `Ok(Some(Item))` - If the item with the specified `id` exists in the storage.
    /// * `Ok(None)` - If no item with the specified `id` is found in the storage.
    /// * `Err(StorageError)` - If an error occurs while trying to access the storage.
    ///
    /// # Errors
    /// This function returns a `StorageError` if there are issues interacting with the storage, such
    /// as a network failure, database corruption, or insufficient permissions.
    ///
    /// # Examples
    async fn get(&self, id: ItemId) -> Result<Option<Item>, StorageError>;

    /// Retrieves a list of `Item`s from the storage that match the specified filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - A parameter of type `ItemFilter` used to define the criteria for filtering items.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Item>)` - A vector of `Item` objects that satisfy the given filter criteria.
    /// * `Err(StorageError)` - An error indicating a failure in retrieving the items from storage.
    ///
    /// # Errors
    ///
    /// Returns a `StorageError` if there is an issue with the storage system or while applying the filter.
    ///
    /// # Examples
    async fn list(&self, filter: ItemFilter) -> Result<Vec<ItemWithStock>, StorageError>;

    /// Updates an existing item in the storage.
    ///
    /// # Parameters
    /// - `item`: The `Item` instance to be updated in the storage. It should contain
    ///   the necessary information to locate and modify the existing record.
    ///
    /// # Returns
    /// - `Ok(())` if the update operation was successful.
    /// - `Err(StorageError)` if an error occurred during the update process, such as:
    ///     - The item does not exist in the storage.
    ///     - There are issues with the underlying storage system (e.g., connection failure).
    ///
    /// # Errors
    /// This function will return a `StorageError` in case of failure. Refer to
    /// the `StorageError` documentation for more details regarding the possible error variants.
    async fn update(&self,  input: &UpdateItemInput) -> Result<Item, StorageError>;

    /// Deletes an item from the storage.
    ///
    /// # Parameters
    /// - `id`: The unique identifier (`ItemId`) of the item to be removed.
    ///
    /// # Returns
    /// - `Ok(())` if the item was successfully deleted.
    /// - `Err(StorageError)` if there was an issue during deletion, such as the item not being found or a storage failure.
    ///
    /// # Errors
    /// This function will return a `StorageError` in cases such as:
    /// - The specified `id` does not exist in the storage.
    /// - An internal error occurred while attempting to delete the item.
    async fn delete(&self, id: ItemId) -> Result<(), StorageError>;
}

#[derive(Clone, Debug)]
pub struct CreateItemInput {
    pub id: ItemId,
    pub sku: Sku,
    pub name: String,
    pub description: Option<String>,
    pub category_id: CategoryId,
    pub unit_cost: Money,
    pub reorder_threshold: u32,
}

#[derive(Clone, Debug)]
pub struct UpdateItemInput {
    pub id: ItemId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<CategoryId>,
    pub unit_cost: Option<Money>,
    pub reorder_threshold: Option<u32>,
}
