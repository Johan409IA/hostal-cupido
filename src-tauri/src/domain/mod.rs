//! Capa de dominio: entidades, value objects y reglas de negocio puras.
//!
//! Regla de la arquitectura: este módulo no depende de `tauri`, `rusqlite`
//! ni `r2d2`; solo de `serde` y de la biblioteca estándar. Aquí vive la
//! lógica de negocio (se implementa en Fase 1).

pub mod auditoria;
pub mod cliente;
pub mod configuracion;
pub mod estancia;
pub mod habitacion;
pub mod limpieza;
pub mod transaccion;

/// Marca de tiempo en segundos desde el epoch Unix (UTC).
pub type FechaHora = i64;
