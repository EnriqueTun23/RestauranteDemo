PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY NOT NULL,
    table_number TEXT NOT NULL,
    customer_name TEXT,
    notes TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    total_cents INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS order_items (
    id TEXT PRIMARY KEY NOT NULL,
    order_id TEXT NOT NULL,
    product_name TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price_cents INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    FOREIGN KEY (order_id)
        REFERENCES orders(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    last_connection_at TEXT
);

CREATE TABLE IF NOT EXISTS app_settings (
    setting_key TEXT PRIMARY KEY NOT NULL,
    setting_value TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_orders_status
ON orders(status);

CREATE INDEX IF NOT EXISTS idx_order_items_order
ON order_items(order_id);