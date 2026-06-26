use stash_storage::repository::ItemRepository;

async fn setup() -> sqlx::sqlite::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

async fn seed_category(pool: &sqlx::sqlite::SqlitePool) -> stash_core::ids::CategoryId {
    let id = stash_core::ids::CategoryId::new();
    sqlx::query("INSERT INTO categories (id, name) VALUES (?, ?)")
        .bind(id.0.to_string())
        .bind("General")
        .execute(pool)
        .await
        .unwrap();
    id
}

#[tokio::test]
async fn create_and_get_item() {
    let db = setup().await;
    let category_id = seed_category(&db).await;
    let repo = stash_storage::sqlite::item_repo::SqliteItemRepository::new(db);

    let input = stash_storage::repository::CreateItemInput {
        id: stash_core::ids::ItemId::new(),
        sku: stash_core::sku::Sku::parse("SKU-001").unwrap(),
        name: "Blue Tumbler".to_string(),
        description: Some("A nice blue tumbler with ornamental designs".to_string()),
        category_id,
        unit_cost: stash_core::money::Money(500),
        reorder_threshold: 10,
    };

    let created = repo.create(&input).await.unwrap();
    let fetched = repo.get(created.id).await.unwrap().unwrap();

    assert_eq!(fetched.name, "Blue Tumbler");
    assert_eq!(fetched.sku.0, "SKU-001");
}

#[tokio::test]
async fn get_missing_item_returns_none() {
    let db = setup().await;
    let repo = stash_storage::sqlite::item_repo::SqliteItemRepository::new(db);
    let result = repo.get(stash_core::ids::ItemId::new()).await.unwrap();
    assert!(result.is_none());
}

#[tokio::test]
async fn update_item_changes_field_values() {
    let db = setup().await;
    let category_id = seed_category(&db).await;
    let repo = stash_storage::sqlite::item_repo::SqliteItemRepository::new(db);

    let input = stash_storage::repository::CreateItemInput {
        id: stash_core::ids::ItemId::new(),
        sku: stash_core::sku::Sku::parse("SKU-002").unwrap(),
        name: "Old Name".to_string(),
        description: None,
        category_id,
        unit_cost: stash_core::money::Money(100),
        reorder_threshold: 5,
    };
    let created = repo.create(&input).await.unwrap();

    let update = stash_storage::repository::UpdateItemInput {
        name: Some("New Name"),
        description: None,
        category_id: None,
        unit_cost: None,
        reorder_threshold: None,
    };
    let updated = repo.update(created.id, &update).await.unwrap();

    assert_eq!(updated.name, "New Name");
}

#[tokio::test]
async fn delete_removes_item() {
    let pool = setup().await;
    let category_id = seed_category(&pool).await;
    let repo = stash_storage::sqlite::item_repo::SqliteItemRepository::new(pool);

    let input = stash_storage::repository::CreateItemInput {
        id: stash_core::ids::ItemId::new(),
        sku: stash_core::sku::Sku::parse("SKU-003").unwrap(),
        name: "To Delete".to_string(),
        description: None,
        category_id,
        unit_cost: stash_core::money::Money(100),
        reorder_threshold: 5,
    };
    let created = repo.create(&input).await.unwrap();
    repo.delete(created.id).await.unwrap();

    assert!(repo.get(created.id).await.unwrap().is_none());
}

#[tokio::test]
async fn list_filters_by_search_text() {
    let pool = setup().await;
    let category_id = seed_category(&pool).await;
    let repo = stash_storage::sqlite::item_repo::SqliteItemRepository::new(pool);

    for (sku, name) in [("SKU-010", "Red Widget"), ("SKU-011", "Blue Gadget")] {
        let input = stash_storage::repository::CreateItemInput {
            id: stash_core::ids::ItemId::new(),
            sku: stash_core::sku::Sku::parse(sku).unwrap(),
            name: name.to_string(),
            description: None,
            category_id,
            unit_cost: stash_core::money::Money(100),
            reorder_threshold: 5,
        };
        repo.create(&input).await.unwrap();
    }

    let filter = stash_core::item::ItemFilter {
        search_text: Some("widget".into()),
        limit: 10,
        offset: 0,
        ..Default::default()
    };
    let results = repo.list(filter).await.unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].item.name, "Red Widget");
}
