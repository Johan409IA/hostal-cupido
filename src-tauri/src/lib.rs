//! Punto de montaje de la aplicación Tauri.
//!
//! La arquitectura por capas vive en:
//! - `domain/`: entidades y reglas de negocio puras (sin Tauri ni SQLite).
//! - `repository/`: traits por entidad; `repository/sqlite/` único lugar con SQL.
//! - `commands/`: comandos IPC finos que orquestan y devuelven `AppError`.
//! - `error.rs`: error unificado serializable al frontend (`{code, message}`).
//! - `state.rs` + `db.rs`: estado compartido con pool de conexiones SQLite.

pub mod commands;
pub mod db;
pub mod domain;
pub mod error;
pub mod repository;
pub mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;

            let directorio = app.path().app_data_dir()?;
            std::fs::create_dir_all(&directorio)?;
            let pool = db::crear_pool(&directorio.join("hostal_cupido.db"))
                .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;
            app.manage(state::AppState::new(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::habitaciones::crear_habitacion,
            commands::habitaciones::listar_habitaciones,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
