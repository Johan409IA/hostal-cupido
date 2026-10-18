use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::domain::FechaHora;

/// Cliente registrado del hostal, identificado de forma única por su DNI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    /// Identificador asignado por la base de datos (`None` hasta persistir).
    pub id: Option<i64>,
    pub dni: String,
    pub nombres: String,
    pub apellidos: String,
    pub telefono: String,
    pub fecha_registro: FechaHora,
}

/// Árbol binario de búsqueda (ABB) de clientes, indexado por DNI.
///
/// Regla del dominio: cada cliente registrado se inserta en el árbol,
/// que arranca vacío, se reconstruye en memoria al iniciar la app desde
/// la tabla `clientes` y se mantiene sincronizado con cada alta. La
/// búsqueda por DNI y el recorrido in-order (clientes ordenados por DNI)
/// cuestan O(altura); los listados por apellidos y por antigüedad de
/// registro se derivan aplicando el orden correspondiente sobre la
/// in-order.
///
/// El DNI es la clave del árbol y es único: insertar un duplicado falla
/// con [`DniDuplicado`]. Al ser un ABB simple (sin autobalanceo), la
/// altura depende del orden de inserción, así que la carga inicial desde
/// la base debería intercalar el orden de los DNI.
#[derive(Debug, Clone)]
pub struct ArbolClientes {
    raiz: Option<Box<NodoClientes>>,
}

#[derive(Debug, Clone)]
struct NodoClientes {
    cliente: Cliente,
    izquierda: Option<Box<NodoClientes>>,
    derecha: Option<Box<NodoClientes>>,
}

/// El DNI que se intentó insertar ya pertenece a un cliente del árbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DniDuplicado;

impl ArbolClientes {
    /// Crea el árbol vacío, listo para reconstruirse desde la base al iniciar.
    pub fn nueva() -> Self {
        Self { raiz: None }
    }

    /// Inserta un cliente en el árbol según su DNI. Falla si el DNI ya
    /// está registrado; en ese caso el árbol queda sin cambios.
    pub fn insertar(&mut self, cliente: Cliente) -> Result<(), DniDuplicado> {
        insertar_en(&mut self.raiz, cliente)
    }

    /// Búsqueda por DNI. Iterativa y descendente, en O(altura).
    pub fn buscar_por_dni(&self, dni: &str) -> Option<&Cliente> {
        let mut nodo = self.raiz.as_deref();
        while let Some(actual) = nodo {
            match dni.cmp(actual.cliente.dni.as_str()) {
                Ordering::Less => nodo = actual.izquierda.as_deref(),
                Ordering::Equal => return Some(&actual.cliente),
                Ordering::Greater => nodo = actual.derecha.as_deref(),
            }
        }
        None
    }

    /// Recorrido in-order: los clientes ordenados ascendentemente por DNI.
    pub fn listar_por_dni(&self) -> Vec<Cliente> {
        let mut clientes = Vec::new();
        recorrido_in_order(&self.raiz, &mut clientes);
        clientes
    }

    /// El mismo conjunto, ordenado alfabéticamente por apellidos; los
    /// nombres y, en última instancia, el DNI desempatan.
    ///
    /// Nota: usa la comparación lexicográfica de Unicode; no aplica una
    /// colación específica del español (no se introducen dependencias).
    pub fn listar_por_apellidos(&self) -> Vec<Cliente> {
        let mut clientes = self.listar_por_dni();
        clientes.sort_by(|a, b| {
            a.apellidos
                .to_lowercase()
                .cmp(&b.apellidos.to_lowercase())
                .then_with(|| a.nombres.to_lowercase().cmp(&b.nombres.to_lowercase()))
                .then_with(|| a.dni.cmp(&b.dni))
        });
        clientes
    }

    /// El mismo conjunto, ordenado por antigüedad de registro: primero
    /// los clientes más antiguos, y por DNI si empatan en fecha.
    pub fn listar_por_antiguedad(&self) -> Vec<Cliente> {
        let mut clientes = self.listar_por_dni();
        clientes.sort_by(|a, b| {
            a.fecha_registro
                .cmp(&b.fecha_registro)
                .then_with(|| a.dni.cmp(&b.dni))
        });
        clientes
    }
}

fn insertar_en(
    nivel: &mut Option<Box<NodoClientes>>,
    cliente: Cliente,
) -> Result<(), DniDuplicado> {
    match nivel {
        None => {
            *nivel = Some(Box::new(NodoClientes {
                cliente,
                izquierda: None,
                derecha: None,
            }));
            Ok(())
        }
        Some(nodo) => match cliente.dni.as_str().cmp(nodo.cliente.dni.as_str()) {
            Ordering::Less => insertar_en(&mut nodo.izquierda, cliente),
            Ordering::Greater => insertar_en(&mut nodo.derecha, cliente),
            Ordering::Equal => Err(DniDuplicado),
        },
    }
}

fn recorrido_in_order(nodo: &Option<Box<NodoClientes>>, salida: &mut Vec<Cliente>) {
    if let Some(actual) = nodo {
        recorrido_in_order(&actual.izquierda, salida);
        salida.push(actual.cliente.clone());
        recorrido_in_order(&actual.derecha, salida);
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Cliente de prueba listo para insertarse en el árbol.
    fn cliente(dni: &str, nombres: &str, apellidos: &str, fecha: FechaHora) -> Cliente {
        Cliente {
            id: None,
            dni: dni.to_string(),
            nombres: nombres.to_string(),
            apellidos: apellidos.to_string(),
            telefono: String::new(),
            fecha_registro: fecha,
        }
    }

    /// DNI del conjunto, en el orden del recorrido, para pinnear el orden.
    fn orden_de(cola: &[Cliente]) -> Vec<String> {
        cola.iter().map(|c| c.dni.clone()).collect()
    }

    /// Árbol con cinco clientes, insertados sin que degenere en una lista.
    fn arbol_de_prueba() -> ArbolClientes {
        let mut arbol = ArbolClientes::nueva();
        arbol
            .insertar(cliente("71234567", "Ana", "Torres Vega", 1_700_000_001))
            .unwrap();
        arbol
            .insertar(cliente("40112233", "Bruno", "Alvarado Ruiz", 1_700_000_000))
            .unwrap();
        arbol
            .insertar(cliente("53098765", "Clara", "Salazar Diaz", 1_700_000_002))
            .unwrap();
        // Empata la fecha de registro con Ana: el DNI lo desempata.
        arbol
            .insertar(cliente("59911122", "Alberto", "Torres Vega", 1_700_000_001))
            .unwrap();
        // El más antiguo de todos los registrados.
        arbol
            .insertar(cliente(
                "88888888",
                "Dario",
                "Bustamante Cruz",
                1_690_000_000,
            ))
            .unwrap();
        arbol
    }

    #[test]
    fn arbol_vacio_no_encuentra_y_los_recorridos_salen_vacios() {
        let arbol = ArbolClientes::nueva();
        assert!(arbol.buscar_por_dni("40112233").is_none());
        assert!(arbol.listar_por_dni().is_empty());
        assert!(arbol.listar_por_apellidos().is_empty());
        assert!(arbol.listar_por_antiguedad().is_empty());
    }

    #[test]
    fn insertar_y_buscar_cliente_existente_e_inexistente() {
        let arbol = arbol_de_prueba();

        let bruno = arbol
            .buscar_por_dni("40112233")
            .expect("debería encontrar a Bruno");
        assert_eq!(bruno.nombres, "Bruno");
        assert_eq!(bruno.apellidos, "Alvarado Ruiz");

        let ana = arbol
            .buscar_por_dni("71234567")
            .expect("debería encontrar a Ana");
        assert_eq!(ana.apellidos, "Torres Vega");
        assert_eq!(ana.fecha_registro, 1_700_000_001);

        // Un DNI que nadie registró no devuelve nada.
        assert!(arbol.buscar_por_dni("10000000").is_none());
    }

    #[test]
    fn insertar_rechaza_dni_duplicado_y_deja_el_arbol_intacto() {
        let mut arbol = arbol_de_prueba();

        let rechazado = cliente("53098765", "Veronica", "Salazar Diaz", 1_790_000_000);
        assert_eq!(arbol.insertar(rechazado), Err(DniDuplicado));

        // El duplicado no entró: siguen los cinco clientes originales.
        assert_eq!(
            orden_de(&arbol.listar_por_dni()),
            vec![
                "40112233".to_string(),
                "53098765".to_string(),
                "59911122".to_string(),
                "71234567".to_string(),
                "88888888".to_string(),
            ]
        );
    }

    #[test]
    fn recorrido_in_order_entrega_los_clientes_ordenados_por_dni() {
        let arbol = arbol_de_prueba();
        assert_eq!(
            orden_de(&arbol.listar_por_dni()),
            vec![
                "40112233".to_string(),
                "53098765".to_string(),
                "59911122".to_string(),
                "71234567".to_string(),
                "88888888".to_string(),
            ]
        );
    }

    #[test]
    fn recorrido_alfabetico_ordena_por_apellidos_y_desempata_por_nombres() {
        let arbol = arbol_de_prueba();
        // Alvarado < Bustamante < Salazar < Torres (dos clientes):
        // entre los Torres gana el de nombres alfabéticamente menor.
        assert_eq!(
            orden_de(&arbol.listar_por_apellidos()),
            vec![
                "40112233".to_string(),
                "88888888".to_string(),
                "53098765".to_string(),
                "59911122".to_string(),
                "71234567".to_string(),
            ]
        );
    }

    #[test]
    fn recorrido_por_antiguedad_pone_lo_mas_antiguo_primero() {
        let arbol = arbol_de_prueba();
        // Orden ascendente por fecha; Ana y Alberto empatan y el DNI desempata.
        assert_eq!(
            orden_de(&arbol.listar_por_antiguedad()),
            vec![
                "88888888".to_string(),
                "40112233".to_string(),
                "59911122".to_string(),
                "71234567".to_string(),
                "53098765".to_string(),
            ]
        );
    }
}
