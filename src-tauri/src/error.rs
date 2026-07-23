use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    Database(String),

    #[error("El pedido no fue encontrado")]
    OrderNotFound,

    #[error("Los datos recibidos no son válidos: {0}")]
    Validation(String),

    #[error("Error de red: {0}")]
    Network(String),

    #[error("No autorizado")]
    Unauthorized,

    #[error("Error interno: {0}")]
    Internal(String),
}

impl Serialize for AppError {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error.to_string())
    }
}