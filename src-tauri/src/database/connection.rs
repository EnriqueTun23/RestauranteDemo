use rusqlite::Connection;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub type DatabaseConnection = Arc<Mutex<Connection>>;

pub fn create_connection(
    database_path: PathBuf,
) -> Result<DatabaseConnection, rusqlite::Error> {
    let connection = Connection::open(database_path)?;

    connection.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        "#,
    )?;

    Ok(Arc::new(Mutex::new(connection)))
}