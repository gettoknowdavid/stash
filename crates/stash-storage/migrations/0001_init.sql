CREATE TABLE categories
(
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE warehouses
(
    id       TEXT PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE,
    location TEXT
);

CREATE TABLE items
(
    id                TEXT PRIMARY KEY,
    sku               TEXT    NOT NULL UNIQUE,
    name              TEXT    NOT NULL,
    description       TEXT,
    category_id       TEXT    NOT NULL REFERENCES categories (id),
    unit_cost         INTEGER NOT NULL,
    reorder_threshold INTEGER NOT NULL,
    created_at        TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TEXT             DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE stock_levels
(
    item_id      TEXT    NOT NULL REFERENCES items (id),
    warehouse_id TEXT    NOT NULL REFERENCES warehouses (id),
    quantity     INTEGER NOT NULL CHECK ( quantity >= 0 )
);

CREATE TABLE stock_movements
(
    id           TEXT PRIMARY KEY,
    item_id      TEXT    NOT NULL REFERENCES items (id),
    warehouse_id TEXT    NOT NULL REFERENCES warehouses (id),
    kind         TEXT    NOT NULL,
    qty_delta    INTEGER NOT NULL,
    reason       TEXT,
    created_at   TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP
);