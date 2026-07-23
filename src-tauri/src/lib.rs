mod commands;
mod database;
mod error;
mod models;
mod repositories;
mod server;
mod services;
mod state;

use crate::{
    commands::orders::{
        create_order,
        delete_order,
        list_orders,
    },
    database::{
        create_connection,
        run_migrations,
    },
    state::AppState,
};

use std::fs;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir =
                app.path().app_data_dir()?;

            fs::create_dir_all(&app_data_dir)?;

            let database_path =
                app_data_dir.join("restaurant.db");

            let database =
                create_connection(database_path)?;

            {
                let connection = database
                    .lock()
                    .map_err(|error| {
                        std::io::Error::other(
                            error.to_string(),
                        )
                    })?;

                run_migrations(&connection)?;
            }

            app.manage(
                AppState::new(database),
            );

            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                list_orders,
                create_order,
                delete_order
            ],
        )
        .run(tauri::generate_context!())
        .expect(
            "No fue posible iniciar App Restaurante",
        );
}