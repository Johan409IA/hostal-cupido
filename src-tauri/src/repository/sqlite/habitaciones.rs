//! Repositorio de habitaciones sobre SQLite (arquetipo de la arquitectura).

use rusqlite::params;
use rusqlite::{ErrorCode, Row};

use crate::db::DbPool;
use crate::domain::habitacion::{EstadoHabitacion, Habitacion, TipoHabitacion};
use crate::error::{AppError, AppResult};
use crate::repository::HabitacionRepository;

pub struct SqliteHabitacionesRepo {
    pool: DbPool,
}

impl SqliteHabitacionesRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

fn ia_texto(tipo: TipoHabitacion) -> &'static str {
    match tipo {
        TipoHabitacion::Normal => "normal",
        TipoHabitacion::Matrimonial => "matrimonial",
    }
}

fn tipo_de_texto(texto: &str) -> AppResult<TipoHabitacion> {
    match texto {
        "normal" => Ok(TipoHabitacion::Normal),
        "matrimonial" => Ok(TipoHabitacion::Matrimonial),
        otro => Err(AppError::BaseDatos(
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                format!("tipo de habitación desconocido: {otro}").into(),
            ),
        )),
    }
}

fn estado_a_texto(estado: EstadoHabitacion) -> &'static str {
    match estado {
        EstadoHabitacion::Disponible => "disponible",
        EstadoHabitacion::Ocupada => "ocupada",
        EstadoHabitacion::Sucia => "sucia",
    }
}

fn estado_de_texto(texto: &str) -> AppResult<EstadoHabitacion> {
    match texto {
        "disponible" => Ok(EstadoHabitacion::Disponible),
        "ocupada" => Ok(EstadoHabitacion::Ocupada),
        "sucia" => Ok(EstadoHabitacion::Sucia),
        otro => Err(AppError::BaseDatos(
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                format!("estado de habitación desconocido: {otro}").into(),
            ),
        )),
    }
}

fn mapear_fila(fila: &Row<'_>) -> rusqlite::Result<Habitacion> {
    let tipo: String = fila.get("tipo")?;
    let estado: String = fila.get("estado")?;
    Ok(Habitacion {
        id: Some(fila.get("id")?),
        numero: fila.get("numero")?,
        tipo: tipo_de_texto(&tipo).expect("tipo persistido válido"),
        tarifa_dia: fila.get("tarifa_dia")?,
        tarifa_momentanea: fila.get("tarifa_momentanea")?,
        estado: estado_de_texto(&estado).expect("estado persistido válido"),
        activa: fila.get("activa")?,
    })
}

impl HabitacionRepository for SqliteHabitacionesRepo {
    fn crear(&self, habitacion: &Habitacion) -> AppResult<Habitacion> {
        let conexion = self.pool.get()?;
        match conexion.execute(
            "INSERT INTO habitaciones
                (numero, tipo, tarifa_dia, tarifa_momentanea, estado, activa)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                habitacion.numero,
                ia_texto(habitacion.tipo),
                habitacion.tarifa_dia,
                habitacion.tarifa_momentanea,
                estado_a_texto(habitacion.estado),
                habitacion.activa,
            ],
        ) {
            Ok(_) => {}
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == ErrorCode::ConstraintViolation =>
            {
                return Err(AppError::Conflicto(format!(
                    "ya existe una habitación con el número {}",
                    habitacion.numero
                )));
            }
            Err(err) => return Err(err.into()),
        }
        let id = conexion.last_insert_rowid();
        Ok(Habitacion {
            id: Some(id),
            ..habitacion.clone()
        })
    }

    fn listar(&self) -> AppResult<Vec<Habitacion>> {
        let conexion = self.pool.get()?;
        let filas = conexion
            .prepare("SELECT * FROM habitaciones WHERE activa = 1 ORDER BY numero")?
            .query_map([], mapear_fila)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(filas)
    }

    fn obtener_por_id(&self, id: i64) -> AppResult<Habitacion> {
        let conexion = self.pool.get()?;
        conexion
            .query_row(
                "SELECT * FROM habitaciones WHERE id = ?1",
                params![id],
                mapear_fila,
            )
            .map_err(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NoEncontrado(format!("habitación id {id}"))
                }
                otro => AppError::BaseDatos(otro),
            })
    }

    fn obtener_por_numero(&self, numero: &str) -> AppResult<Habitacion> {
        let conexion = self.pool.get()?;
        conexion
            .query_row(
                "SELECT * FROM habitaciones WHERE numero = ?1",
                params![numero],
                mapear_fila,
            )
            .map_err(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NoEncontrado(format!("habitación número {numero}"))
                }
                otro => AppError::BaseDatos(otro),
            })
    }

    fn actualizar(&self, habitacion: &Habitacion) -> AppResult<Habitacion> {
        let Some(id) = habitacion.id else {
            return Err(AppError::Validacion(
                "la habitación a actualizar debe tener id".into(),
            ));
        };
        let conexion = self.pool.get()?;
        let actualizadas = match conexion.execute(
            "UPDATE habitaciones
             SET numero = ?1, tipo = ?2, tarifa_dia = ?3, tarifa_momentanea = ?4,
                 estado = ?5, activa = ?6
             WHERE id = ?7",
            params![
                habitacion.numero,
                ia_texto(habitacion.tipo),
                habitacion.tarifa_dia,
                habitacion.tarifa_momentanea,
                estado_a_texto(habitacion.estado),
                habitacion.activa,
                id,
            ],
        ) {
            Ok(n) => n,
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == ErrorCode::ConstraintViolation =>
            {
                return Err(AppError::Conflicto(format!(
                    "ya existe una habitación con el número {}",
                    habitacion.numero
                )));
            }
            Err(err) => return Err(err.into()),
        };
        if actualizadas == 0 {
            return Err(AppError::NoEncontrado(format!("habitación id {id}")));
        }
        Ok(habitacion.clone())
    }

    fn desactivar(&self, id: i64) -> AppResult<()> {
        let conexion = self.pool.get()?;
        let actualizadas = conexion.execute(
            "UPDATE habitaciones SET activa = 0 WHERE id = ?1",
            params![id],
        )?;
        if actualizadas == 0 {
            return Err(AppError::NoEncontrado(format!("habitación id {id}")));
        }
        Ok(())
    }

    fn cambiar_estado(&self, id: i64, estado: EstadoHabitacion) -> AppResult<Habitacion> {
        self.cambiar_estado_sql(id, estado)?;
        self.obtener_por_id(id)
    }
}

impl SqliteHabitacionesRepo {
    fn cambiar_estado_sql(&self, id: i64, estado: EstadoHabitacion) -> AppResult<()> {
        let conexion = self.pool.get()?;
        let actualizadas = conexion.execute(
            "UPDATE habitaciones SET estado = ?1 WHERE id = ?2",
            params![estado_a_texto(estado), id],
        )?;
        if actualizadas == 0 {
            return Err(AppError::NoEncontrado(format!("habitación id {id}")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn pool_en_memoria() -> DbPool {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = r2d2::Pool::builder().max_size(1).build(manager).unwrap();
        crate::db::inicializar_esquema(&pool).unwrap();
        pool
    }

    fn repo() -> SqliteHabitacionesRepo {
        SqliteHabitacionesRepo::new(pool_en_memoria())
    }

    fn habitacion() -> Habitacion {
        Habitacion {
            id: None,
            numero: "101".into(),
            tipo: TipoHabitacion::Normal,
            tarifa_dia: 80.0,
            tarifa_momentanea: 45.0,
            estado: EstadoHabitacion::Disponible,
            activa: true,
        }
    }

    #[test]
    fn crea_y_obtiene_por_numero() {
        let repo = repo();
        let creada = repo.crear(&habitacion()).unwrap();
        assert_eq!(creada.id, Some(1));

        let encontrada = repo.obtener_por_numero("101").unwrap();
        assert_eq!(encontrada.numero, "101");
        assert_eq!(encontrada.tipo, TipoHabitacion::Normal);
        assert_eq!(encontrada.estado, EstadoHabitacion::Disponible);
        assert!(encontrada.activa);
    }

    #[test]
    fn rechaza_numero_duplicado() {
        let repo = repo();
        repo.crear(&habitacion()).unwrap();
        let error = repo.crear(&habitacion()).unwrap_err();
        assert!(matches!(error, AppError::Conflicto(_)));
    }

    #[test]
    fn actualiza_y_desactiva() {
        let repo = repo();
        let creada = repo.crear(&habitacion()).unwrap();

        let mut corneta = creada.clone();
        corneta.numero = "102".into();
        corneta.tipo = TipoHabitacion::Matrimonial;
        let actualizada = repo.actualizar(&corneta).unwrap();
        assert_eq!(actualizada.numero, "102");

        repo.desactivar(creada.id.unwrap()).unwrap();
        assert!(!repo.obtener_por_id(1).unwrap().activa);
    }
}
