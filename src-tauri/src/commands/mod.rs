//! Capa IPC: comandos `#[tauri::command]` expuestos al frontend.
//!
//! Regla de la arquitectura: los comandos solo validan la entrada mínima,
//! orquestan los repositorios y devuelven `Result<T, AppError>`.
//! Sin lógica de negocio propia.

pub mod habitaciones;
