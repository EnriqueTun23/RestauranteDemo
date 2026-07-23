use crate::{
    error::AppError,
    models::{CreateOrderInput, Order},
    repositories::OrderRepository,
    state::AppState,
};

use tauri::State;

#[tauri::command]
pub async fn list_orders(
    state: State<'_, AppState>,
) -> Result<Vec<Order>, AppError> {
    OrderRepository::list(&state.database)
}

#[tauri::command]
pub async fn create_order(
    input: CreateOrderInput,
    state: State<'_, AppState>,
) -> Result<Order, AppError> {
    validate_order(&input)?;

    OrderRepository::create(
        &state.database,
        input,
    )
}

#[tauri::command]
pub async fn delete_order(
    order_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    OrderRepository::delete(
        &state.database,
        &order_id,
    )
}

fn validate_order(
    input: &CreateOrderInput,
) -> Result<(), AppError> {
    if input.table_number.trim().is_empty() {
        return Err(AppError::Validation(
            "Debes indicar la mesa".to_string(),
        ));
    }

    if input.items.is_empty() {
        return Err(AppError::Validation(
            "El pedido debe tener al menos un producto"
                .to_string(),
        ));
    }

    for item in &input.items {
        if item.product_name.trim().is_empty() {
            return Err(AppError::Validation(
                "Todos los productos necesitan un nombre"
                    .to_string(),
            ));
        }

        if item.quantity <= 0 {
            return Err(AppError::Validation(
                "La cantidad debe ser mayor que cero"
                    .to_string(),
            ));
        }
    }

    Ok(())
}