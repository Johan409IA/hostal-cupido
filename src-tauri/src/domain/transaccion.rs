use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Transacción de pago asociada a una estancia y su cliente.
///
/// Regla del dominio: solo la transacción en el tope de la pila de
/// recientes (la más reciente) puede anularse o corregirse, y nunca
/// se borra físicamente (trazabilidad auditada).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaccion {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub estancia_id: i64,
    pub cliente_id: i64,
    pub importe: f64,
    pub fecha: FechaHora,
    pub metodo_pago: MetodoPago,
    pub estado: EstadoTransaccion,
    /// Motivo requerido cuando la transacción queda `Anulada`.
    pub motivo_anulacion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetodoPago {
    Efectivo,
    Tarjeta,
    Transferencia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoTransaccion {
    Vigente,
    Anulada,
}
