use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Cliente registrado del hostal, identificado de forma única por su DNI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub dni: String,
    pub nombres: String,
    pub apellidos: String,
    pub telefono: String,
    pub fecha_registro: FechaHora,
}
