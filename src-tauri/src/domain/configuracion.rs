use serde::{Deserialize, Serialize};

/// Configuración del hostal: onboarding inicial y preferencias operativas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuracion {
    pub onboarding_completado: bool,
    /// Datos del hostal: nombre, dirección y datos de contacto.
    pub datos_hostal: String,
    /// Preferencias operativas almacenadas como texto JSON.
    pub preferencias: String,
}
