use stash_core::category::CategoryName;
use stash_storage::category_repository::{
    CategoryRepository, CreateCategoryInput, UpdateCategoryInput,
};
use stash_storage::item_repository::ItemRepository;
use stash_storage::sqlite::category_repo::CategoryRepo;

async fn setup() -> sqlx::sqlite::SqlitePool {
    let db = sqlx::sqlite::SqlitePoolOptions::new().connect(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    db
}

#[tokio::test]
async fn create_and_get_category() {
    let repo = CategoryRepo::new(setup().await);
    let input = CreateCategoryInput {
        id: stash_core::ids::CategoryId::new(),
        name: CategoryName::parse("Electronics").unwrap(),
        description: Some("Gadgets and gizmos".into()),
    };
    let created = repo.create(&input).await.unwrap();
    let fetched = repo.get(created.id).await.unwrap().unwrap();
    assert_eq!(fetched.name.0, "Electronics");
}

#[tokio::test]
async fn duplicate_name_is_rejected() {
    let repo = CategoryRepo::new(setup().await);
    let make = || CreateCategoryInput {
        id: stash_core::ids::CategoryId::new(),
        name: CategoryName::parse("Books").unwrap(),
        description: None,
    };
    repo.create(&make()).await.unwrap();
    let err = repo.create(&make()).await.unwrap_err();
    assert!(err.message().contains("already exists"));
}

#[tokio::test]
async fn update_changes_name() {
    let repo = CategoryRepo::new(setup().await);
    let created = repo
        .create(&CreateCategoryInput {
            id: stash_core::ids::CategoryId::new(),
            name: CategoryName::parse("Old").unwrap(),
            description: None,
        })
        .await
        .unwrap();

    let updated = repo
        .update(&UpdateCategoryInput {
            id: created.id,
            name: Some(CategoryName::parse("New").unwrap()),
            description: None,
        })
        .await
        .unwrap();
    assert_eq!(updated.name.0, "New");
}

#[tokio::test]
async fn delete_in_use_category_is_rejected() {
    let db = setup().await;
    let category_repo = CategoryRepo::new(db.clone());
    let category = category_repo
        .create(&CreateCategoryInput {
            id: stash_core::ids::CategoryId::new(),
            name: CategoryName::parse("InUse").unwrap(),
            description: None,
        })
        .await
        .unwrap();

    let item_repo = stash_storage::sqlite::item_repo::ItemRepo::new(db);
    item_repo
        .create(&stash_storage::item_repository::CreateItemInput {
            id: stash_core::ids::ItemId::new(),
            sku: stash_core::sku::Sku::parse("SKU-999").unwrap(),
            name: "Blocker".into(),
            description: None,
            category_id: category.id,
            unit_cost: stash_core::money::Money(100),
            reorder_threshold: 1,
        })
        .await
        .unwrap();

    let err = category_repo.delete(category.id).await.unwrap_err();
    assert!(matches!(err, stash_storage::StorageError::CategoryInUse(_)));
}
