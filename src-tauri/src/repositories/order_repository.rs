use crate::{
    database::DatabaseConnection,
    error::AppError,
    models::{
        CreateOrderInput,
        Order,
        OrderItem,
        OrderStatus,
    },
};

use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

pub struct OrderRepository;

impl OrderRepository {
    pub fn list(
        database: &DatabaseConnection,
    ) -> Result<Vec<Order>, AppError> {
        let connection = database
            .lock()
            .map_err(|error| {
                AppError::Internal(error.to_string())
            })?;

        let mut statement = connection.prepare(
            r#"
            SELECT
                id,
                table_number,
                customer_name,
                notes,
                status,
                total_cents,
                created_at,
                updated_at
            FROM orders
            ORDER BY created_at DESC
            "#,
        )?;

        let rows = statement.query_map([], |row| {
            let status: String = row.get(4)?;

            Ok(Order {
                id: row.get(0)?,
                table_number: row.get(1)?,
                customer_name: row.get(2)?,
                notes: row.get(3)?,
                status: parse_status(&status),
                total_cents: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                items: Vec::new(),
            })
        })?;

        let mut orders = Vec::new();

        for row in rows {
            let mut order = row?;
            order.items =
                Self::list_items(&connection, &order.id)?;
            orders.push(order);
        }

        Ok(orders)
    }

    pub fn create(
        database: &DatabaseConnection,
        input: CreateOrderInput,
    ) -> Result<Order, AppError> {
        let mut connection = database
            .lock()
            .map_err(|error| {
                AppError::Internal(error.to_string())
            })?;

        let transaction = connection.transaction()?;

        let order_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let total_cents = input
            .items
            .iter()
            .map(|item| {
                item.unit_price_cents
                    * i64::from(item.quantity)
            })
            .sum::<i64>();

        transaction.execute(
            r#"
            INSERT INTO orders (
                id,
                table_number,
                customer_name,
                notes,
                status,
                total_cents,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                order_id,
                input.table_number,
                input.customer_name,
                input.notes,
                "pending",
                total_cents,
                now,
                now
            ],
        )?;

        for item in &input.items {
            transaction.execute(
                r#"
                INSERT INTO order_items (
                    id,
                    order_id,
                    product_name,
                    quantity,
                    unit_price_cents,
                    notes
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    Uuid::new_v4().to_string(),
                    order_id,
                    item.product_name,
                    item.quantity,
                    item.unit_price_cents,
                    item.notes
                ],
            )?;
        }

        transaction.commit()?;

        drop(connection);

        Self::find_by_id(database, &order_id)?
            .ok_or(AppError::OrderNotFound)
    }

    pub fn find_by_id(
        database: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<Order>, AppError> {
        let connection = database
            .lock()
            .map_err(|error| {
                AppError::Internal(error.to_string())
            })?;

        let order = connection
            .query_row(
                r#"
                SELECT
                    id,
                    table_number,
                    customer_name,
                    notes,
                    status,
                    total_cents,
                    created_at,
                    updated_at
                FROM orders
                WHERE id = ?1
                "#,
                [id],
                |row| {
                    let status: String = row.get(4)?;

                    Ok(Order {
                        id: row.get(0)?,
                        table_number: row.get(1)?,
                        customer_name: row.get(2)?,
                        notes: row.get(3)?,
                        status: parse_status(&status),
                        total_cents: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        items: Vec::new(),
                    })
                },
            )
            .optional()?;

        match order {
            Some(mut order) => {
                order.items =
                    Self::list_items(&connection, id)?;
                Ok(Some(order))
            }
            None => Ok(None),
        }
    }

    pub fn delete(
        database: &DatabaseConnection,
        id: &str,
    ) -> Result<(), AppError> {
        let connection = database
            .lock()
            .map_err(|error| {
                AppError::Internal(error.to_string())
            })?;

        let affected = connection.execute(
            "DELETE FROM orders WHERE id = ?1",
            [id],
        )?;

        if affected == 0 {
            return Err(AppError::OrderNotFound);
        }

        Ok(())
    }

    fn list_items(
        connection: &rusqlite::Connection,
        order_id: &str,
    ) -> Result<Vec<OrderItem>, AppError> {
        let mut statement = connection.prepare(
            r#"
            SELECT
                id,
                order_id,
                product_name,
                quantity,
                unit_price_cents,
                notes
            FROM order_items
            WHERE order_id = ?1
            "#,
        )?;

        let rows = statement.query_map(
            [order_id],
            |row| {
                Ok(OrderItem {
                    id: row.get(0)?,
                    order_id: row.get(1)?,
                    product_name: row.get(2)?,
                    quantity: row.get(3)?,
                    unit_price_cents: row.get(4)?,
                    notes: row.get(5)?,
                })
            },
        )?;

        let mut items = Vec::new();

        for row in rows {
            items.push(row?);
        }

        Ok(items)
    }
}

fn parse_status(status: &str) -> OrderStatus {
    match status {
        "preparing" => OrderStatus::Preparing,
        "ready" => OrderStatus::Ready,
        "delivered" => OrderStatus::Delivered,
        "cancelled" => OrderStatus::Cancelled,
        _ => OrderStatus::Pending,
    }
}