use crate::db::DbPool;

/// Estado compartido de la aplicación.
///
/// Tauri lo administra con `app.manage()` y los comandos lo reciben
/// con `tauri::State<AppState>`.
pub struct AppState {
    pub db: DbPool,
}

impl AppState {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}
