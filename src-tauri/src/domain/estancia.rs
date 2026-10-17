use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Estancia: una estadía de un cliente en una habitación.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estancia {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub cliente_id: i64,
    pub habitacion_id: i64,
    pub modalidad: ModalidadEstancia,
    pub ingreso: FechaHora,
    pub salida_prevista: FechaHora,
    pub salida_real: Option<FechaHora>,
    /// Importe calculado por el backend (el frontend solo previsualiza).
    pub importe: f64,
    pub estado: EstadoEstancia,
}

/// Modalidad de la estancia.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalidadEstancia {
    /// Día completo: duración de 24 horas.
    DiaCompleto,
    /// Estancia momentánea: duración de 4 horas.
    Momentanea,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoEstancia {
    Activa,
    Cerrada,
}
