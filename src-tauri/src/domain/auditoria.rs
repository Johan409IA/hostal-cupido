use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Registro inmutable de auditoría de operaciones sensibles.
///
/// Regla del dominio: nunca se borran entradas de auditoría.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntradaAuditoria {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    /// Acción ejecutada (ej.: "check_out").
    pub accion: String,
    /// Tipo de entidad afectada (ej.: "estancia").
    pub entidad: String,
    /// Identificador de la entidad afectada.
    pub entidad_afectada: String,
    /// Estado anterior de la entidad, si aplica.
    pub detalle_antes: Option<String>,
    /// Estado posterior de la entidad, si aplica.
    pub detalle_despues: Option<String>,
    pub fecha: FechaHora,
}
