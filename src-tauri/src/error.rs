use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// Tipo de error unificado del backend.
///
/// Tauri lo serializa automáticamente al invocar un comando que falla,
/// por lo que el frontend recibe `{ code, message }`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("error de base de datos: {0}")]
    BaseDatos(#[from] rusqlite::Error),

    #[error("error de conexión: {0}")]
    Conexion(#[from] r2d2::Error),

    #[error("{0}")]
    Validacion(String),

    #[error("no encontrado: {0}")]
    NoEncontrado(String),

    #[error("conflicto: {0}")]
    Conflicto(String),
}

impl AppError {
    pub fn codigo(&self) -> &'static str {
        match self {
            Self::BaseDatos(_) => "base_datos",
            Self::Conexion(_) => "conexion",
            Self::Validacion(_) => "validacion",
            Self::NoEncontrado(_) => "no_encontrado",
            Self::Conflicto(_) => "conflicto",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut estado = serializer.serialize_struct("AppError", 2)?;
        estado.serialize_field("code", self.codigo())?;
        estado.serialize_field("message", &self.to_string())?;
        estado.end()
    }
}

pub type AppResult<T> = Result<T, AppError>;
