use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Entrada en la cola FIFO de limpieza.
///
/// Regla del dominio: cada check-out encola la habitación y la limpieza
/// se atiende por orden de llegada (la que lleva más tiempo esperando
/// sale primero). Al finalizar, la habitación pasa a Disponible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntradaColaLimpieza {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub habitacion_id: i64,
    pub fecha_encolado: FechaHora,
    pub fecha_atencion: Option<FechaHora>,
}

/// Cola FIFO de habitaciones pendientes de limpieza, en memoria.
///
/// Regla del dominio: al confirmar el check-out, la habitación pasa a
/// `Sucia` y se encola automáticamente en el fondo de la cola; se atiende
/// primero la habitación que lleva más tiempo esperando (orden de llegada
/// estricto). Cada habitación tiene a lo sumo una entrada pendiente. La
/// cola sobrevive al cierre de la app en la tabla `cola_limpieza` vía la
/// capa de repositorio, y se reconstruye al iniciar con
/// [`ColaLimpieza::desde_entradas`], que ordena por `fecha_encolado`.
#[derive(Debug, Clone)]
pub struct ColaLimpieza {
    cola: VecDeque<EntradaColaLimpieza>,
}

/// La habitación que se intentó encolar ya tiene una limpieza pendiente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HabitacionYaEnCola;

impl ColaLimpieza {
    /// Crea la cola vacía; el arranque la reconstruye con
    /// [`ColaLimpieza::desde_entradas`] y el check-out encola durante el
    /// flujo operativo con [`ColaLimpieza::encolar`].
    pub fn nueva() -> Self {
        Self {
            cola: VecDeque::new(),
        }
    }

    /// Reconstruye la cola para arrancar la app desde las entradas
    /// persistidas en la tabla `cola_limpieza`, ordenándolas por
    /// `fecha_encolado` (la más antigua al frente), sea cual sea el orden
    /// en que lleguen.
    pub fn desde_entradas(mut entradas: Vec<EntradaColaLimpieza>) -> Self {
        entradas.sort_by_key(|entrada| entrada.fecha_encolado);
        Self {
            cola: entradas.into(),
        }
    }

    /// Encola una habitación que acaba de pasar a `Sucia` tras su
    /// check-out, al fondo de la cola. Falla si ya tiene una limpieza
    /// pendiente; en ese caso la cola queda sin cambios.
    pub fn encolar(&mut self, entrada: EntradaColaLimpieza) -> Result<(), HabitacionYaEnCola> {
        if self
            .cola
            .iter()
            .any(|pendiente| pendiente.habitacion_id == entrada.habitacion_id)
        {
            return Err(HabitacionYaEnCola);
        }
        self.cola.push_back(entrada);
        Ok(())
    }

    /// Consulta la siguiente habitación a limpiar (la que lleva más
    /// tiempo esperando) sin retirarla de la cola.
    pub fn ver_siguiente(&self) -> Option<&EntradaColaLimpieza> {
        self.cola.front()
    }

    /// Atiende la siguiente habitación: la desencola y la devuelve con
    /// su `fecha_atencion` registrada; la capa de repositorio persiste
    /// esa marca sin borrar la entrada físicamente. Devuelve `None` si
    /// no hay pendientes.
    pub fn atender_siguiente(&mut self, fecha_atencion: FechaHora) -> Option<EntradaColaLimpieza> {
        let mut atendida = self.cola.pop_front()?;
        atendida.fecha_atencion = Some(fecha_atencion);
        Some(atendida)
    }

    /// Lista las habitaciones pendientes de limpieza en el orden en que
    /// deben atenderse.
    pub fn listar_pendientes(&self) -> Vec<EntradaColaLimpieza> {
        self.cola.iter().cloned().collect()
    }

    /// Cantidad de habitaciones pendientes de limpieza.
    pub fn tamano(&self) -> usize {
        self.cola.len()
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Entrada de prueba ya persistida (con id), lista para encolarse.
    fn entrada(id: i64, habitacion_id: i64, fecha_encolado: FechaHora) -> EntradaColaLimpieza {
        EntradaColaLimpieza {
            id: Some(id),
            habitacion_id,
            fecha_encolado,
            fecha_atencion: None,
        }
    }

    /// Ids de habitación de los pendientes, en el orden de atención.
    fn orden_de(cola: &[EntradaColaLimpieza]) -> Vec<i64> {
        cola.iter().map(|e| e.habitacion_id).collect()
    }

    /// Cola con tres habitaciones que hicieron check-out seguidas.
    fn cola_de_prueba() -> ColaLimpieza {
        let mut cola = ColaLimpieza::nueva();
        cola.encolar(entrada(1, 101, 1_700_000_010)).unwrap();
        cola.encolar(entrada(2, 102, 1_700_000_020)).unwrap();
        cola.encolar(entrada(3, 103, 1_700_000_030)).unwrap();
        cola
    }

    #[test]
    fn cola_vacia_no_atiende_nada_y_esta_en_cero() {
        let mut cola = ColaLimpieza::nueva();
        assert!(cola.ver_siguiente().is_none());
        assert!(cola.atender_siguiente(1_700_000_099).is_none());
        assert!(cola.listar_pendientes().is_empty());
        assert_eq!(cola.tamano(), 0);
    }

    #[test]
    fn encolar_agrega_al_fondo_y_ver_siguiente_no_desencola() {
        let cola = cola_de_prueba();

        let primera = cola
            .ver_siguiente()
            .expect("la cola no debería estar vacía");
        assert_eq!(primera.habitacion_id, 101);
        assert_eq!(primera.fecha_atencion, None);
        // Ver la siguiente no altera la cola: siguen los tres pendientes.
        assert_eq!(cola.tamano(), 3);
        assert_eq!(orden_de(&cola.listar_pendientes()), vec![101, 102, 103]);
    }

    #[test]
    fn atender_siguiente_respeta_el_orden_fifo_de_llegada() {
        let mut cola = cola_de_prueba();

        // Sale por el mismo orden en que se encolaron, cada una con su
        // fecha de atención registrada.
        let primera = cola
            .atender_siguiente(1_700_000_100)
            .expect("pendiente esperada");
        assert_eq!(primera.habitacion_id, 101);
        assert_eq!(primera.fecha_atencion, Some(1_700_000_100));
        assert_eq!(cola.tamano(), 2);

        let segunda = cola
            .atender_siguiente(1_700_000_110)
            .expect("pendiente esperada");
        assert_eq!(segunda.habitacion_id, 102);
        assert_eq!(cola.ver_siguiente().unwrap().habitacion_id, 103);

        let ultima = cola
            .atender_siguiente(1_700_000_120)
            .expect("pendiente esperada");
        assert_eq!(ultima.habitacion_id, 103);
        assert_eq!(cola.tamano(), 0);
        assert!(cola.atender_siguiente(1_700_000_130).is_none());
    }

    #[test]
    fn el_check_out_de_una_habitacion_limpia_vuelve_a_colarla() {
        // Ciclo real de una habitación: sucia → limpiada → sucia de nuevo.
        let mut cola = cola_de_prueba();
        cola.atender_siguiente(1_700_000_100)
            .expect("pendiente esperada");
        cola.atender_siguiente(1_700_000_110)
            .expect("pendiente esperada");

        cola.encolar(entrada(4, 102, 1_700_000_220)).unwrap();
        assert_eq!(orden_de(&cola.listar_pendientes()), vec![103, 102]);
        assert_eq!(cola.tamano(), 2);
    }

    #[test]
    fn encolar_rechaza_la_habitacion_que_ya_tiene_limpieza_pendiente() {
        let mut cola = ColaLimpieza::nueva();
        cola.encolar(entrada(1, 101, 1_700_000_010)).unwrap();

        assert_eq!(
            cola.encolar(entrada(2, 101, 1_700_000_050)),
            Err(HabitacionYaEnCola)
        );
        // El duplicado no entró: queda la única entrada original.
        assert_eq!(cola.tamano(), 1);
        assert_eq!(orden_de(&cola.listar_pendientes()), vec![101]);
    }

    #[test]
    fn desde_entradas_reconstruye_ordenando_por_fecha_de_encolado() {
        let mut cola = ColaLimpieza::nueva();
        cola.encolar(entrada(1, 101, 1_700_000_010)).unwrap();
        cola.encolar(entrada(2, 102, 1_700_000_020)).unwrap();
        cola.encolar(entrada(3, 103, 1_700_000_030)).unwrap();

        // Reconstrucción desde la tabla: entradas guardadas en cualquier
        // orden de inserción, pero con fecha_encolado única por fila.
        let mut recu = ColaLimpieza::desde_entradas(vec![
            entrada(2, 102, 1_700_000_020),
            entrada(4, 104, 1_700_000_100),
            entrada(1, 101, 1_700_000_010),
            entrada(3, 103, 1_700_000_030),
        ]);

        assert_eq!(
            orden_de(&recu.listar_pendientes()),
            vec![101, 102, 103, 104]
        );
        assert_eq!(recu.tamano(), 4);
        // Y la reconstruida respeta el mismo FIFO que la original.
        assert_eq!(
            recu.atender_siguiente(1_700_000_200).unwrap().habitacion_id,
            101
        );
    }
}
