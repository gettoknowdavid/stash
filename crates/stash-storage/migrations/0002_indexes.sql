CREATE INDEX idx_item_category ON items (id, category_id);

CREATE UNIQUE INDEX idx_stock_item_warehouse
    ON stock_levels (item_id, warehouse_id);

CREATE INDEX idx_movements_item_time
    ON stock_movements (item_id, created_at);

