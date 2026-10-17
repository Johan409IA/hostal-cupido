use serde::{Deserialize, Serialize};

/// Habitación del hostal: inventario físico con sus tarifas y estado operativo.
///
/// Transiciones de estado válidas (se implementan en Fase 1):
/// `Disponible → Ocupada` (check-in), `Ocupada → Sucia` (check-out)
/// y `Sucia → Disponible` (fin de limpieza). Ninguna otra.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habitacion {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    /// Número único de la habitación (ej.: "101").
    pub numero: String,
    pub tipo: TipoHabitacion,
    pub tarifa_dia: f64,
    pub tarifa_momentanea: f64,
    pub estado: EstadoHabitacion,
    pub activa: bool,
}

/// Tipo de habitación según su equipamiento.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TipoHabitacion {
    Normal,
    Matrimonial,
}

/// Estado operativo de la habitación.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoHabitacion {
    Disponible,
    Ocupada,
    Sucia,
}
