use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub table_number: String,
    pub customer_name: Option<String>,
    pub notes: Option<String>,
    pub status: OrderStatus,
    pub total_cents: i64,
    pub created_at: String,
    pub updated_at: String,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Preparing,
    Ready,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderInput {
    pub table_number: String,
    pub customer_name: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<CreateOrderItemInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderItemInput {
    pub product_name: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrderInput {
    pub table_number: String,
    pub customer_name: Option<String>,
    pub notes: Option<String>,
    pub status: OrderStatus,
    pub items: Vec<CreateOrderItemInput>,
}