use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Transacción de pago asociada a una estancia y su cliente.
///
/// Regla del dominio: solo la transacción en el tope de la pila de
/// recientes (la más reciente) puede anularse o corregirse, y nunca
/// se borra físicamente (trazabilidad auditada).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaccion {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub estancia_id: i64,
    pub cliente_id: i64,
    pub importe: f64,
    pub fecha: FechaHora,
    pub metodo_pago: MetodoPago,
    pub estado: EstadoTransaccion,
    /// Motivo requerido cuando la transacción queda `Anulada`.
    pub motivo_anulacion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetodoPago {
    Efectivo,
    Tarjeta,
    Transferencia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoTransaccion {
    Vigente,
    Anulada,
}

/// Pila LIFO de las transacciones más recientes, en memoria.
///
/// Regla del dominio: cada pago registrado se apila; la transacción del
/// tope se consulta con [`PilaTransaccionesRecientes::ver_ultima`] y se
/// anula con [`PilaTransaccionesRecientes::deshacer`], que la retira de
/// la pila y la devuelve ya marcada como `Anulada` para que la capa de
/// repositorio persista esa marca (nunca se borra físicamente, para
/// conservar trazabilidad). Al superar el límite configurable en
/// memoria, la transacción más antigua sale de la pila y solo vive en
/// base de datos.
#[derive(Debug, Clone)]
pub struct PilaTransaccionesRecientes {
    /// Cantidad máxima de transacciones conservadas en memoria.
    limite: usize,
    /// Transacciones en memoria, con el tope (la más reciente) al final.
    transacciones: Vec<Transaccion>,
}

impl PilaTransaccionesRecientes {
    /// Crea la pila vacía, con espacio en memoria para `limite` transacciones.
    pub fn nueva(limite: usize) -> Self {
        Self {
            limite,
            transacciones: Vec::new(),
        }
    }

    /// Apila un pago recién registrado en el tope de la pila. Si se supera
    /// el límite, la transacción más antigua sale de memoria y queda solo
    /// en base de datos.
    pub fn apilar(&mut self, transaccion: Transaccion) {
        self.transacciones.push(transaccion);
        let exceso = self.transacciones.len().saturating_sub(self.limite);
        self.transacciones.drain(..exceso);
    }

    /// Consulta la transacción del tope sin retirarla.
    pub fn ver_ultima(&self) -> Option<&Transaccion> {
        self.transacciones.last()
    }

    /// Retira la última transacción de la pila y la devuelve ya marcada
    /// como `Anulada`, con su motivo: el repositorio debe persistir esa
    /// marca sin borrar el registro físico. Devuelve `None` si la pila
    /// está vacía.
    pub fn deshacer(&mut self, motivo: &str) -> Option<Transaccion> {
        let mut deshecha = self.transacciones.pop()?;
        deshecha.estado = EstadoTransaccion::Anulada;
        deshecha.motivo_anulacion = Some(motivo.to_string());
        Some(deshecha)
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Pago de prueba ya persistido (con id), listo para apilarse.
    fn pago(id: i64, importe: f64) -> Transaccion {
        Transaccion {
            id: Some(id),
            estancia_id: 10,
            cliente_id: 20,
            importe,
            fecha: 1_700_000_000 + id,
            metodo_pago: MetodoPago::Efectivo,
            estado: EstadoTransaccion::Vigente,
            motivo_anulacion: None,
        }
    }

    #[test]
    fn apilar_pone_el_pago_en_el_tope_y_ver_ultima_no_lo_retira() {
        let mut pila = PilaTransaccionesRecientes::nueva(10);
        pila.apilar(pago(1, 50.0));
        pila.apilar(pago(2, 30.0));

        let tope = pila.ver_ultima().expect("la pila no debería estar vacía");
        assert_eq!(tope.id, Some(2));
        assert_eq!(tope.importe, 30.0);
        // Ver el tope no altera la pila: siguen las dos transacciones en memoria.
        assert_eq!(pila.transacciones.len(), 2);
    }

    #[test]
    fn deshacer_sigue_el_orden_lifo() {
        let mut pila = PilaTransaccionesRecientes::nueva(10);
        pila.apilar(pago(1, 50.0));
        pila.apilar(pago(2, 30.0));
        pila.apilar(pago(3, 20.0));

        // El deshacer sale por el mismo orden inverso al que entraron.
        let tercera = pila.deshacer("error de digitación").expect("tope esperado");
        assert_eq!(tercera.id, Some(3));
        assert_eq!(tercera.estado, EstadoTransaccion::Anulada);
        assert_eq!(
            tercera.motivo_anulacion.as_deref(),
            Some("error de digitación")
        );
        assert_eq!(pila.ver_ultima().unwrap().id, Some(2));

        let segunda = pila.deshacer("error de digitación").expect("tope esperado");
        assert_eq!(segunda.id, Some(2));
        assert_eq!(pila.ver_ultima().unwrap().id, Some(1));

        // La transacción deshecha conserva sus datos (solo cambió su estado):
        // nunca se borra físicamente, se anula.
        assert_eq!(segunda.importe, 30.0);
    }

    #[test]
    fn deshacer_en_pila_vacia_devuelve_none_y_deja_la_pila_igual() {
        let mut pila = PilaTransaccionesRecientes::nueva(5);
        assert!(pila.ver_ultima().is_none());
        assert!(pila.deshacer("error").is_none());

        pila.apilar(pago(1, 50.0));
        pila.deshacer("error").unwrap();

        // Tras deshacer la única transacción, la pila vuelve a estar vacía
        // y un segundo deshacer tampoco altera nada.
        assert!(pila.ver_ultima().is_none());
        assert!(pila.deshacer("error").is_none());
        assert_eq!(pila.transacciones.len(), 0);
    }

    #[test]
    fn el_limite_solo_conserva_en_memoria_las_transacciones_mas_recientes() {
        let mut pila = PilaTransaccionesRecientes::nueva(2);
        pila.apilar(pago(1, 50.0)); // sale de memoria: solo vive en base de datos
        pila.apilar(pago(2, 30.0));
        pila.apilar(pago(3, 20.0));

        assert_eq!(pila.transacciones.len(), 2);
        assert_eq!(pila.ver_ultima().unwrap().id, Some(3));
        // Y el deshacer sigue respetando el orden LIFO dentro del límite.
        assert_eq!(pila.deshacer("error").unwrap().id, Some(3));
        assert_eq!(pila.ver_ultima().unwrap().id, Some(2));
    }

    #[test]
    fn apilar_con_limite_1_conserva_solo_la_mas_reciente() {
        let mut pila = PilaTransaccionesRecientes::nueva(1);
        pila.apilar(pago(1, 50.0));
        pila.apilar(pago(2, 30.0));

        // Con límite 1, cada apilar conserva únicamente la más reciente.
        assert_eq!(pila.transacciones.len(), 1);
        assert_eq!(pila.ver_ultima().unwrap().id, Some(2));
    }
}
