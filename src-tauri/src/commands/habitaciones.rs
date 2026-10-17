//! Comandos IPC del módulo de habitaciones.

use tauri::State;

use crate::domain::habitacion::{EstadoHabitacion, Habitacion, TipoHabitacion};
use crate::error::{AppError, AppResult};
use crate::repository::sqlite::habitaciones::SqliteHabitacionesRepo;
use crate::repository::HabitacionRepository;
use crate::state::AppState;

/// Crea una habitación nueva en estado Disponible.
///
/// La validación aquí es de entrada mínima; las reglas de negocio
/// (transiciones de estado, tarifas) viven en `domain/` (Fase 1).
#[tauri::command]
pub fn crear_habitacion(
    estado: State<'_, AppState>,
    numero: String,
    tipo: TipoHabitacion,
    tarifa_dia: f64,
    tarifa_momentanea: f64,
) -> AppResult<Habitacion> {
    let numero = numero.trim().to_string();
    if numero.is_empty() {
        return Err(AppError::Validacion(
            "el número de habitación es obligatorio".into(),
        ));
    }
    if tarifa_dia <= 0.0 || tarifa_momentanea <= 0.0 {
        return Err(AppError::Validacion(
            "las tarifas deben ser mayores a cero".into(),
        ));
    }
    let repo = SqliteHabitacionesRepo::new(estado.db.clone());
    repo.crear(&Habitacion {
        id: None,
        numero,
        tipo,
        tarifa_dia,
        tarifa_momentanea,
        estado: EstadoHabitacion::Disponible,
        activa: true,
    })
}

/// Lista las habitaciones activas, ordenadas por número.
#[tauri::command]
pub fn listar_habitaciones(estado: State<'_, AppState>) -> AppResult<Vec<Habitacion>> {
    let repo = SqliteHabitacionesRepo::new(estado.db.clone());
    repo.listar()
}
