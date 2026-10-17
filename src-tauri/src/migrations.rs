//! Sistema de migraciones versionadas de la base de datos.
//!
//! Cada migración es una `Migracion` con su `version` y SQL idempotente
//! (`IF NOT EXISTS`). Al arrancar, `aplicar` lee `schema_version`, ejecuta
//! cada migración pendiente dentro de una transacción (SQL + registro de
//! versión, atómico) y falla si la base es más nueva que la app
//! (protección offline-first: una base con migraciones futuras no debe
//! divergir silenciosamente).

use rusqlite::params;

use crate::db::DbPool;
use crate::error::{AppError, AppResult};

/// Una migración versionada: SQL idempotente + su versión.
pub struct Migracion {
    pub version: i64,
    pub sql: &'static str,
}

/// Lista ordenada de migraciones. Las nuevas se agregan al final con
/// la versión siguiente (`LISTA.last().version + 1`).
///
/// - v1: tabla arquetipo `habitaciones` (antes bootstrap de `db.rs`).
/// - v2: `clientes`, v3: `estancias`, v4: `transacciones`,
///   v5: `cola_limpieza`, v6: `auditoria` (esquema base de PLAN §5),
///   v7: índice de estados de habitaciones.
pub const LISTA: &[Migracion] = &[
    Migracion {
        version: 1,
        sql: "CREATE TABLE IF NOT EXISTS habitaciones (
                  id                INTEGER PRIMARY KEY AUTOINCREMENT,
                  numero            TEXT    NOT NULL UNIQUE,
                  tipo              TEXT    NOT NULL,
                  tarifa_dia        REAL    NOT NULL,
                  tarifa_momentanea REAL    NOT NULL,
                  estado            TEXT    NOT NULL DEFAULT 'disponible',
                  activa            INTEGER NOT NULL DEFAULT 1
              );",
    },
    Migracion {
        version: 2,
        sql: "CREATE TABLE IF NOT EXISTS clientes (
                  -- dni único, con índice explícito idx_clientes_dni (v2).
                  id             INTEGER PRIMARY KEY AUTOINCREMENT,
                  dni            TEXT    NOT NULL UNIQUE,
                  nombres        TEXT    NOT NULL,
                  apellidos      TEXT    NOT NULL,
                  telefono       TEXT    NOT NULL,
                  fecha_registro INTEGER NOT NULL
              );
              CREATE INDEX IF NOT EXISTS idx_clientes_dni ON clientes (dni);",
    },
    Migracion {
        version: 3,
        sql: "CREATE TABLE IF NOT EXISTS estancias (
                  -- modalidad: 'dia_completo' | 'momentanea'
                  -- estado: 'activa' | 'cerrada' (los valores los valida domain).
                  id                   INTEGER PRIMARY KEY AUTOINCREMENT,
                  cliente_id           INTEGER NOT NULL REFERENCES clientes (id) ON DELETE RESTRICT,
                  habitacion_id        INTEGER NOT NULL REFERENCES habitaciones (id) ON DELETE RESTRICT,
                  modalidad            TEXT    NOT NULL,
                  hora_ingreso         INTEGER NOT NULL,
                  hora_salida_prevista INTEGER NOT NULL,
                  hora_salida_real     INTEGER,
                  importe              REAL    NOT NULL,
                  estado               TEXT    NOT NULL DEFAULT 'activa'
              );
              CREATE INDEX IF NOT EXISTS idx_estancias_estado ON estancias (estado);",
    },
    Migracion {
        version: 4,
        sql: "CREATE TABLE IF NOT EXISTS transacciones (
                  -- método de pago libre (lo valida domain); nunca se borra
                  -- un pago: la anulación es un flag con su motivo.
                  id               INTEGER PRIMARY KEY AUTOINCREMENT,
                  estancia_id      INTEGER NOT NULL REFERENCES estancias (id) ON DELETE RESTRICT,
                  cliente_id       INTEGER NOT NULL REFERENCES clientes (id) ON DELETE RESTRICT,
                  importe          REAL    NOT NULL,
                  fecha            INTEGER NOT NULL,
                  metodo_pago      TEXT    NOT NULL,
                  anulada          INTEGER NOT NULL DEFAULT 0,
                  motivo_anulacion TEXT
              );",
    },
    Migracion {
        version: 5,
        sql: "CREATE TABLE IF NOT EXISTS cola_limpieza (
                  -- El orden FIFO de la cola lo da fecha_encolado;
                  -- fecha_atendido queda nula mientras esté pendiente.
                  id             INTEGER PRIMARY KEY AUTOINCREMENT,
                  habitacion_id  INTEGER NOT NULL REFERENCES habitaciones (id) ON DELETE RESTRICT,
                  fecha_encolado INTEGER NOT NULL,
                  fecha_atendido INTEGER
              );",
    },
    Migracion {
        version: 6,
        sql: "CREATE TABLE IF NOT EXISTS auditoria (
                  -- Sin FK: `entidad_id` es polimórfica (la entidad la nombra
                  -- `entidad`). Nunca se borran registros de auditoría.
                  id         INTEGER PRIMARY KEY AUTOINCREMENT,
                  accion     TEXT    NOT NULL,
                  entidad    TEXT    NOT NULL,
                  entidad_id INTEGER NOT NULL,
                  detalle    TEXT    NOT NULL,
                  fecha      INTEGER NOT NULL
              );",
    },
    Migracion {
        version: 7,
        // v1 no se toca (ya aplicada en dev-bases): el índice llega aparte.
        sql: "CREATE INDEX IF NOT EXISTS idx_habitaciones_estado ON habitaciones (estado);",
    },
];

/// Aplica las migraciones pendientes sobre la base de datos del pool.
///
/// - Crea `schema_version` si es base nueva; si no, lee su versión.
/// - Aplica cada migración pendiente: SQL + registro de versión en
///   **una transacción** (si falla una migración, no queda nada a medias).
/// - Si la base tiene versión mayor que la app, devuelve `AppError::Validacion`.
pub fn aplicar(pool: &DbPool) -> AppResult<()> {
    validar_contiguidad(LISTA)?;
    let mut conexion = pool.get()?;
    let tx = conexion.transaction()?;

    tx.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
             version        INTEGER PRIMARY KEY,
             fecha_aplicada INTEGER NOT NULL
         );",
    )?;
    let version_actual: i64 = tx.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_version",
        [],
        |fila| fila.get(0),
    )?;

    let ultima = LISTA.last().map(|m| m.version).unwrap_or(0);
    if version_actual > ultima {
        return Err(AppError::Validacion(format!(
            "la base de datos está en la versión {version_actual}, más nueva que \
             lo que soporta esta aplicación (versión {ultima}). Actualiza la aplicación."
        )));
    }

    for migracion in LISTA.iter().filter(|m| m.version > version_actual) {
        tx.execute_batch(migracion.sql)?;
        tx.execute(
            "INSERT INTO schema_version (version, fecha_aplicada) VALUES (?1, ?2)",
            params![migracion.version, ahora_en_segundos()],
        )?;
    }
    tx.commit()?;
    Ok(())
}

/// Fecha de registro como epoch en segundos, igual que `domain::FechaHora`.
fn ahora_en_segundos() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Valida que las versiones de la lista sean 1, 2, 3... consecutivas.
///
/// Un salto (p. ej. v3 sin v2) aplicaría una migración sin su paso previo:
/// mejor fallar al arrancar que corromper el esquema.
fn validar_contiguidad(lista: &[Migracion]) -> AppResult<()> {
    for (posicion, migracion) in lista.iter().enumerate() {
        if migracion.version != posicion as i64 + 1 {
            return Err(AppError::Validacion(format!(
                "las migraciones están mal numeradas: la posición {} tiene la \
                 versión {}, se esperaba {}",
                posicion + 1,
                migracion.version,
                posicion + 1
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod pruebas {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use r2d2_sqlite::SqliteConnectionManager;

    use super::*;

    /// Contador para dar a cada test su propio archivo SQLite
    /// (los tests corren en paralelo y no deben pisarse).
    static SECUENCIA: AtomicU64 = AtomicU64::new(0);

    /// Pool **crudo** (sin `with_init`) sobre un archivo SQLite único en el
    /// directorio temporal: así se prueba `aplicar` aislada, sin los PRAGMA
    /// de `db::crear_pool`. Devuelve `(pool, directorio)` para limpiar.
    fn pool_en_archivo() -> (crate::db::DbPool, PathBuf) {
        let n = SECUENCIA.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "hostal-cupido-migraciones-{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let manager = SqliteConnectionManager::file(dir.join("prueba.db"));
        let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();
        (pool, dir)
    }

    /// Borra el directorio temporal del test (mejor esfuerzo).
    fn limpiar(dir: &PathBuf) {
        let _ = std::fs::remove_dir_all(dir);
    }

    /// Lee `schema_version`: 0 si la base es nueva, la versión más alta si
    /// ya tiene migraciones.
    fn version_de(pool: &crate::db::DbPool) -> i64 {
        pool.get()
            .unwrap()
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |fila| fila.get(0),
            )
            .unwrap()
    }

    /// Ejecuta SQL arbitrario sobre la base del pool (para armar estados).
    fn ejecutar(pool: &crate::db::DbPool, sql: &str) {
        pool.get().unwrap().execute_batch(sql).unwrap();
    }

    #[test]
    fn base_nueva_aplica_todas_las_migraciones() {
        let (pool, dir) = pool_en_archivo();
        aplicar(&pool).unwrap();
        assert_eq!(version_de(&pool), 7);
        // Las 6 tablas base quedaron creadas (más `schema_version`).
        let conexion = pool.get().unwrap();
        for tabla in [
            "habitaciones",
            "clientes",
            "estancias",
            "transacciones",
            "cola_limpieza",
            "auditoria",
        ] {
            let total: i64 = conexion
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master
                     WHERE type = 'table' AND name = ?1",
                    [tabla],
                    |fila| fila.get(0),
                )
                .unwrap();
            assert_eq!(total, 1, "faltó la tabla {tabla}");
        }
        limpiar(&dir);
    }

    #[test]
    fn aplicar_dos_veces_es_idempotente() {
        let (pool, dir) = pool_en_archivo();
        aplicar(&pool).unwrap();
        aplicar(&pool).unwrap();
        assert_eq!(version_de(&pool), 7);
        // 7 migraciones → 7 filas (una por migración aplicada);
        // la segunda pasada no agregó ninguna extra.
        let total: i64 = pool
            .get()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM schema_version", [], |f| f.get(0))
            .unwrap();
        assert_eq!(total, 7);
        limpiar(&dir);
    }

    #[test]
    fn rechaza_base_con_migraciones_futuras() {
        let (pool, dir) = pool_en_archivo();
        // Simula una base "del futuro": ya migró a una versión que la app
        // (que solo conoce hasta v7) no reconoce.
        ejecutar(
            &pool,
            "CREATE TABLE schema_version (
                 id             INTEGER PRIMARY KEY AUTOINCREMENT,
                 version        INTEGER NOT NULL UNIQUE,
                 fecha_aplicada INTEGER NOT NULL
             );
             INSERT INTO schema_version (version, fecha_aplicada) VALUES (99, 0);",
        );
        let error = aplicar(&pool).unwrap_err();
        assert!(matches!(error, AppError::Validacion(_)));
        limpiar(&dir);
    }

    /// Dev-base anterior al sistema de migraciones: tiene la tabla
    /// `habitaciones` creada por el bootstrap y ningún registro de versión.
    /// El arranque con el runner debe converger sola: aplicar v1 sin error,
    /// registrar la versión y conservar los datos existentes. Se simula con
    /// `LISTA[0].sql` porque el bootstrap antiguo era literalmente ese SQL.
    #[test]
    fn converge_base_del_bootstrap_anterior() {
        let (pool, dir) = pool_en_archivo();
        ejecutar(&pool, LISTA[0].sql);
        ejecutar(
            &pool,
            "INSERT INTO habitaciones (numero, tipo, tarifa_dia, tarifa_momentanea, estado, activa)
             VALUES ('101', 'normal', 80.0, 45.0, 'disponible', 1);",
        );

        aplicar(&pool).unwrap();

        assert_eq!(version_de(&pool), 7);
        // Los datos de la dev-base se conservaron.
        let filas: i64 = pool
            .get()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM habitaciones", [], |f| f.get(0))
            .unwrap();
        assert_eq!(filas, 1);
        limpiar(&dir);
    }

    /// Las versiones de la lista deben ser consecutivas: un salto (v3 sin
    /// v2) significa aplicar SQL sin su paso previo y el runner debe
    /// rechazarlo al arrancar, no a medias.
    #[test]
    fn la_lista_de_versiones_debe_ser_consecutiva() {
        let lista_ok = [
            Migracion {
                version: 1,
                sql: "",
            },
            Migracion {
                version: 2,
                sql: "",
            },
        ];
        validar_contiguidad(&lista_ok).unwrap();

        let lista_con_hueco = [
            Migracion {
                version: 1,
                sql: "",
            },
            Migracion {
                version: 3,
                sql: "",
            },
        ];
        let error = validar_contiguidad(&lista_con_hueco).unwrap_err();
        assert!(matches!(error, AppError::Validacion(_)));
    }

    #[test]
    fn dni_de_cliente_es_unico() {
        let (pool, dir) = pool_en_archivo();
        aplicar(&pool).unwrap();
        ejecutar(
            &pool,
            "INSERT INTO clientes (dni, nombres, apellidos, telefono, fecha_registro)
             VALUES ('76123456', 'Ana', 'Pérez', '987654321', 1700000000);",
        );
        let duplicado = pool.get().unwrap().execute(
            "INSERT INTO clientes (dni, nombres, apellidos, telefono, fecha_registro)
             VALUES ('76123456', 'Luis', 'García', '912345678', 1700000100)",
            [],
        );
        assert!(duplicado.is_err());
        limpiar(&dir);
    }

    #[test]
    fn indices_minimos_existen() {
        let (pool, dir) = pool_en_archivo();
        aplicar(&pool).unwrap();
        let conexion = pool.get().unwrap();
        for (nombre, tabla) in [
            ("idx_clientes_dni", "clientes"),
            ("idx_habitaciones_estado", "habitaciones"),
            ("idx_estancias_estado", "estancias"),
        ] {
            let total: i64 = conexion
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master
                     WHERE type = 'index' AND name = ?1 AND tbl_name = ?2",
                    rusqlite::params![nombre, tabla],
                    |fila| fila.get(0),
                )
                .unwrap();
            assert_eq!(total, 1, "faltó el índice {nombre}");
        }
        limpiar(&dir);
    }
}
