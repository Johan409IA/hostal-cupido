use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Entrada en la cola FIFO de limpieza.
///
/// Regla del dominio: cada check-out encola la habitación y la limpieza
/// se atiende por orden de llegada (la que lleva más tiempo esperando
/// sale primero). Al finalizar, la habitación pasa a Disponible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntradaColaLimpieza {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub habitacion_id: i64,
    pub fecha_encolado: FechaHora,
    pub fecha_atencion: Option<FechaHora>,
}
