use std::path::Path;

use r2d2_sqlite::SqliteConnectionManager;

use crate::error::{AppError, AppResult};

/// Pool de conexiones a SQLite.
pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

/// Crea el pool con los PRAGMAs requeridos en cada conexión
/// (WAL para escritura concurrente y foreign keys activadas) y aplica
/// las migraciones versionadas sobre la base recién abierta.
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
    crate::migrations::aplicar(&pool)?;
    Ok(pool)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Contador para dar a cada test su propio archivo SQLite
    /// (los tests corren en paralelo y no deben pisarse).
    static SECUENCIA: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    #[test]
    fn crear_pool_aplica_migraciones_y_activa_pragmas() {
        let n = SECUENCIA.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "hostal-cupido-crear-pool-{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let ruta = dir.join("prueba.db");

        let pool = crear_pool(&ruta).unwrap();

        // Migraciones v1..v7 aplicadas y registradas en una base recién creada.
        let version: i64 = pool
            .get()
            .unwrap()
            .query_row("SELECT MAX(version) FROM schema_version", [], |f| f.get(0))
            .unwrap();
        assert_eq!(version, 7);
        let tablas: i64 = pool
            .get()
            .unwrap()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'table' AND name IN ('habitaciones', 'schema_version')",
                [],
                |f| f.get(0),
            )
            .unwrap();
        assert_eq!(tablas, 2);

        // PRAGMAs activos en una conexión real de archivo.
        let conexion = pool.get().unwrap();
        let wal: String = conexion
            .query_row("PRAGMA journal_mode", [], |f| f.get(0))
            .unwrap();
        assert_eq!(wal, "wal");
        let fks: i64 = conexion
            .query_row("PRAGMA foreign_keys", [], |f| f.get(0))
            .unwrap();
        assert_eq!(fks, 1);

        let _ = std::fs::remove_dir_all(dir);
    }

    /// Con el pool de producción (`foreign_keys = ON`) el esquema completo
    /// debe exigir la cadena de referencias: habitación → cliente →
    /// estancia → transacción/limpieza. La auditoría, en cambio, es
    /// polimórfica (`entidad_id` sin FK) y acepta cualquier entidad.
    #[test]
    fn crear_pool_cumple_foreign_keys_del_esquema() {
        let n = SECUENCIA.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("hostal-cupido-fk-{}-{n}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let pool = crear_pool(&dir.join("prueba.db")).unwrap();

        let conexion = pool.get().unwrap();
        // Cadena válida completa, sin huecos de referencias.
        conexion
            .execute_batch(
                "INSERT INTO habitaciones (numero, tipo, tarifa_dia, tarifa_momentanea, estado, activa)
                 VALUES ('101', 'normal', 80.0, 45.0, 'disponible', 1);
                 INSERT INTO clientes (dni, nombres, apellidos, telefono, fecha_registro)
                 VALUES ('76123456', 'Ana', 'Pérez', '987654321', 1700000000);
                 INSERT INTO estancias (cliente_id, habitacion_id, modalidad, hora_ingreso,
                                        hora_salida_prevista, hora_salida_real, importe, estado)
                 VALUES (1, 1, 'dia_completo', 1700000000, 1700043200, NULL, 80.0, 'activa');
                 INSERT INTO transacciones (estancia_id, cliente_id, importe, fecha,
                                            metodo_pago, anulada, motivo_anulacion)
                 VALUES (1, 1, 80.0, 1700000100, 'efectivo', 0, NULL);
                 INSERT INTO cola_limpieza (habitacion_id, fecha_encolado, fecha_atendido)
                 VALUES (1, 1700043000, NULL);
                 INSERT INTO auditoria (accion, entidad, entidad_id, detalle, fecha)
                 VALUES ('crear', 'cliente', 1, '{}', 1700000000);",
            )
            .unwrap();

        // Referencias huérfanas: bloqueadas por las foreign keys.
        let casos = [
            // estancias con cliente inexistente
            "INSERT INTO estancias (cliente_id, habitacion_id, modalidad, hora_ingreso, \
                  hora_salida_prevista, hora_salida_real, importe, estado)
             VALUES (999, 1, 'dia_completo', 1, 2, NULL, 1.0, 'activa')",
            // estancias con habitación inexistente
            "INSERT INTO estancias (cliente_id, habitacion_id, modalidad, hora_ingreso, \
                  hora_salida_prevista, hora_salida_real, importe, estado)
             VALUES (1, 999, 'dia_completo', 1, 2, NULL, 1.0, 'activa')",
            // transacciones con estancia inexistente
            "INSERT INTO transacciones (estancia_id, cliente_id, importe, fecha, \
                  metodo_pago, anulada, motivo_anulacion)
             VALUES (999, 1, 1.0, 1, 'efectivo', 0, NULL)",
            // transacciones con cliente inexistente
            "INSERT INTO transacciones (estancia_id, cliente_id, importe, fecha, \
                  metodo_pago, anulada, motivo_anulacion)
             VALUES (1, 999, 1.0, 1, 'efectivo', 0, NULL)",
            // cola de limpieza con habitación inexistente
            "INSERT INTO cola_limpieza (habitacion_id, fecha_encolado, fecha_atendido)
             VALUES (999, 1, NULL)",
        ];
        for sql in casos {
            let resultado = conexion.execute(sql, []);
            assert!(resultado.is_err(), "debió rechazar: {sql}");
        }

        let _ = std::fs::remove_dir_all(dir);
    }
}
