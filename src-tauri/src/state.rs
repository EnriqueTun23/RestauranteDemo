use crate::database::DatabaseConnection;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub server_started: Arc<AtomicBool>,
}

impl AppState {
    pub fn new(database: DatabaseConnection) -> Self {
        Self {
            database,
            server_started: Arc::new(
                AtomicBool::new(false),
            ),
        }
    }

    pub fn is_server_started(&self) -> bool {
        self.server_started.load(Ordering::Relaxed)
    }

    pub fn set_server_started(&self, value: bool) {
        self.server_started
            .store(value, Ordering::Relaxed);
    }
}