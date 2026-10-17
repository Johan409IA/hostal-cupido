use std::path::Path;

use r2d2_sqlite::SqliteConnectionManager;

use crate::error::{AppError, AppResult};

/// Pool de conexiones a SQLite.
pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

/// Crea el pool con los PRAGMAs requeridos en cada conexión
/// (WAL para rendimiento de escritura y foreign keys activadas).
pub fn crear_pool(ruta: &Path) -> AppResult<DbPool> {
    let manager = SqliteConnectionManager::file(ruta).with_init(|conexion| {
        conexion.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;",
        )
    });
    let pool = r2d2::Pool::builder()
        .build(manager)
        .map_err(AppError::Conexion)?;
    inicializar_esquema(&pool)?;
    Ok(pool)
}

/// Bootstrap del esquema: crea la tabla arquetipo `habitaciones` si no existe.
///
/// El esquema completo de las demás tablas llega con el sistema
/// de migraciones (tarea de base de datos, Fase 0).
pub fn inicializar_esquema(pool: &DbPool) -> AppResult<()> {
    let conexion = pool.get()?;
    conexion.execute_batch(
        "CREATE TABLE IF NOT EXISTS habitaciones (
            id                INTEGER PRIMARY KEY AUTOINCREMENT,
            numero            TEXT    NOT NULL UNIQUE,
            tipo              TEXT    NOT NULL,
            tarifa_dia        REAL    NOT NULL,
            tarifa_momentanea REAL    NOT NULL,
            estado            TEXT    NOT NULL DEFAULT 'disponible',
            activa            INTEGER NOT NULL DEFAULT 1
        );",
    )?;
    Ok(())
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn pool_en_memoria() -> DbPool {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();
        inicializar_esquema(&pool).unwrap();
        pool
    }

    #[test]
    fn crea_la_tabla_habitaciones() {
        let pool = pool_en_memoria();
        let conexion = pool.get().unwrap();
        let total: i64 = conexion
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'table' AND name = 'habitaciones'",
                [],
                |fila| fila.get(0),
            )
            .unwrap();
        assert_eq!(total, 1);
    }
}
