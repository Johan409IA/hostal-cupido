//! Capa de repositorios: acceso a datos por entidad.
//!
//! Regla de la arquitectura: aquí solo se definen los traits por entidad
//! y en `sqlite/` viven sus implementaciones concretas — el único lugar
//! del backend con SQL.

pub mod sqlite;

use crate::domain::auditoria::EntradaAuditoria;
use crate::domain::cliente::Cliente;
use crate::domain::configuracion::Configuracion;
use crate::domain::estancia::Estancia;
use crate::domain::habitacion::{EstadoHabitacion, Habitacion};
use crate::domain::limpieza::EntradaColaLimpieza;
use crate::domain::transaccion::{MetodoPago, Transaccion};
use crate::domain::FechaHora;
use crate::error::AppResult;

/// Acceso a datos de las habitaciones.
pub trait HabitacionRepository {
    /// Inserta una habitación y la devuelve con su id asignado.
    fn crear(&self, habitacion: &Habitacion) -> AppResult<Habitacion>;
    /// Lista las habitaciones activas, ordenadas por número.
    fn listar(&self) -> AppResult<Vec<Habitacion>>;
    fn obtener_por_id(&self, id: i64) -> AppResult<Habitacion>;
    fn obtener_por_numero(&self, numero: &str) -> AppResult<Habitacion>;
    /// Actualiza los campos de la habitación (id obligatorio).
    fn actualizar(&self, habitacion: &Habitacion) -> AppResult<Habitacion>;
    /// Da de baja la habitación sin borrar su registro (baja lógica).
    fn desactivar(&self, id: i64) -> AppResult<()>;
    /// Cambia el estado operativo de la habitación.
    fn cambiar_estado(&self, id: i64, estado: EstadoHabitacion) -> AppResult<Habitacion>;
}

/// Acceso a datos de los clientes (DNI único).
pub trait ClienteRepository {
    fn crear(&self, cliente: &Cliente) -> AppResult<Cliente>;
    fn obtener_por_dni(&self, dni: &str) -> AppResult<Cliente>;
    fn listar(&self) -> AppResult<Vec<Cliente>>;
}

/// Acceso a datos de las estancias.
pub trait EstanciaRepository {
    fn crear(&self, estancia: &Estancia) -> AppResult<Estancia>;
    fn listar_por_cliente(&self, cliente_id: i64) -> AppResult<Vec<Estancia>>;
    fn listar_activas(&self) -> AppResult<Vec<Estancia>>;
    /// Cierra la estancia con la salida real e importe definitivo.
    fn cerrar(&self, id: i64, salida_real: FechaHora, importe: f64) -> AppResult<Estancia>;
}

/// Acceso a datos de las transacciones de pago (pila LIFO de recientes).
pub trait TransaccionRepository {
    /// Registra una transacción de pago.
    fn crear(
        &self,
        estancia_id: i64,
        cliente_id: i64,
        importe: f64,
        fecha: FechaHora,
        metodo_pago: MetodoPago,
    ) -> AppResult<Transaccion>;
    /// Devuelve la transacción más reciente (tope de la pila LIFO).
    fn ultima(&self) -> AppResult<Option<Transaccion>>;
    /// Devuelve las últimas N transacciones, la más reciente primero.
    fn listar_recientes(&self, limite: usize) -> AppResult<Vec<Transaccion>>;
    /// Anula una transacción con su motivo (nunca borra el registro).
    fn anular(&self, id: i64, motivo: &str) -> AppResult<Transaccion>;
}

/// Acceso a datos de la cola FIFO de limpieza.
pub trait ColaLimpiezaRepository {
    fn encolar(&self, entrada: &EntradaColaLimpieza) -> AppResult<EntradaColaLimpieza>;
    /// Siguiente habitación a limpiar: la que lleva más tiempo esperando.
    fn siguiente_pendiente(&self) -> AppResult<Option<EntradaColaLimpieza>>;
    /// Marca la entrada como atendida con su fecha.
    fn marcar_atendida(&self, id: i64, fecha_atencion: FechaHora)
        -> AppResult<EntradaColaLimpieza>;
}

/// Acceso al registro de auditoría (solo escritura y consulta).
pub trait AuditoriaRepository {
    fn registrar(&self, entrada: &EntradaAuditoria) -> AppResult<EntradaAuditoria>;
    fn listar(&self, limite: usize) -> AppResult<Vec<EntradaAuditoria>>;
}

/// Configuración del hostal (una fila de preferencias vigentes).
pub trait ConfiguracionRepository {
    /// Devuelve la configuración actual o `None` si el onboarding no empezó.
    fn obtener(&self) -> AppResult<Option<Configuracion>>;
    /// Guarda (o reemplaza) la configuración vigente.
    fn guardar(&self, configuracion: &Configuracion) -> AppResult<Configuracion>;
}
