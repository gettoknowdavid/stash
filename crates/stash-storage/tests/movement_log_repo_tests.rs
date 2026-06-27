use stash_core::ids::{CategoryId, ItemId, WarehouseId};
use stash_core::stock::StockMovement;
use stash_storage::movement_log_repository::MovementLogRepository;
use stash_storage::sqlite::movement_log_repo::MovementLogRepo;

async fn setup_db() -> sqlx::sqlite::SqlitePool {
    let db = sqlx::sqlite::SqlitePoolOptions::new().connect(":memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    db
}

async fn seed_test_data(db: &sqlx::sqlite::SqlitePool) -> (ItemId, WarehouseId) {
    let warehouse_id = WarehouseId::new();
    sqlx::query("INSERT INTO warehouses (id, name) VALUES (?, ?)")
        .bind(warehouse_id.0.to_string())
        .bind("Test Warehouse")
        .execute(db)
        .await
        .unwrap();

    let category_id = CategoryId::new();
    sqlx::query("INSERT INTO categories (id, name) VALUES (?, ?)")
        .bind(category_id.0.to_string())
        .bind("General")
        .execute(db)
        .await
        .unwrap();

    let item_id = ItemId::new();
    sqlx::query(
        "INSERT INTO items (id, sku, name, category_id, unit_cost, reorder_threshold)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(item_id.0.to_string())
    .bind("TEST-001")
    .bind("Test Item")
    .bind(category_id.0.to_string())
    .bind(1000i64)
    .bind(10i64)
    .execute(db)
    .await
    .unwrap();

    (item_id, warehouse_id)
}

#[tokio::test]
async fn record_inbound_movement() {
    let db = setup_db().await;
    let (item_id, warehouse_id) = seed_test_data(&db).await;
    let repo = MovementLogRepo::new(db);
    let movement = StockMovement::Inbound { qty: 50, reason: "Restock".to_string() };
    let record = repo.record(item_id, warehouse_id, &movement).await;
    assert!(record.is_ok());
}

#[tokio::test]
async fn record_outbound_movement() {
    let db = setup_db().await;
    let (item_id, warehouse_id) = seed_test_data(&db).await;
    let repo = MovementLogRepo::new(db);
    let movement = StockMovement::Outbound { qty: 20, reason: "Sale".to_string() };
    let record = repo.record(item_id, warehouse_id, &movement).await;
    assert!(record.is_ok());
}

#[tokio::test]
async fn list_for_item_returns_records() {
    let db = setup_db().await;
    let (item_id, warehouse_id) = seed_test_data(&db).await;
    let repo = MovementLogRepo::new(db.clone());

    let m1 = StockMovement::Inbound { qty: 100, reason: "Initial".into() };
    let m2 = StockMovement::Outbound { qty: 30, reason: "Sale".into() };

    repo.record(item_id, warehouse_id, &m1).await.unwrap();
    repo.record(item_id, warehouse_id, &m2).await.unwrap();

    let records = repo.list_for_item(item_id, 10, 0).await.unwrap();

    assert_eq!(records.len(), 2);
    assert!(records.iter().any(|m| m.movement == m1));
    assert!(records.iter().any(|m| m.movement == m2));
}

#[tokio::test]
async fn list_recent_respects_limit() {
    let db = setup_db().await;
    let (item_id, warehouse_id) = seed_test_data(&db).await;
    let repo = MovementLogRepo::new(db);

    for i in 0..5 {
        let movement = StockMovement::Inbound { qty: 10, reason: format!("Test {}", i) };
        repo.record(item_id, warehouse_id, &movement).await.unwrap();
    }

    let records = repo.list_recent(3, 0).await.unwrap();
    assert_eq!(records.len(), 3);
}
