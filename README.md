#  Sistema de Gestión Hotelera

Sistema de escritorio para la administración integral de un pequeño hotel u hostal. Diseñado para optimizar las operaciones diarias, ofrecer un control rápido y fiable de habitaciones, clientes y transacciones, y proporcionar una experiencia ágil tanto para el personal como para la gerencia.

---

##  Características principales

###  Gestión de habitaciones
- **Registro de habitaciones**: Alta de nuevas habitaciones con número, tipo (normal o matrimonial) y tarifa asociada.
- **Consulta de disponibilidad**: Visualización en tiempo real del estado de cada habitación.
- **Control de estados**: Cada habitación puede estar en uno de tres estados:
  - **Disponible** – lista para asignar a un cliente.
  - **Ocupada** – actualmente en uso.
  - **Sucia** – pendiente de limpieza tras un check-out.

###  Gestión de clientes
- **Registro por DNI**: Cada cliente se identifica de forma única mediante su documento de identidad.
- **Historial de estancias**: Se mantiene un registro completo de todas las visitas del cliente para agilizar futuros check‑in.

###  Registro de ingresos (check‑in)
- **Asignación de habitación**: El sistema selecciona una habitación disponible y la asigna al cliente.
- **Dos modalidades de estancia**:
  - **Día completo** (24 horas)
  - **Estancia momentánea** (4 horas)
- **Cálculo automático**: Se calcula la hora de salida y el importe total en función de la tarifa y la modalidad elegida.

###  Gestión de limpieza
- **Cola FIFO**: Cuando una habitación queda sucia, se encola automáticamente. La limpieza se atiende por orden de llegada (la que lleva más tiempo esperando es la primera en limpiarse).
- **Actualización de estado**: Al finalizar la limpieza, la habitación vuelve a estar disponible.

###  Control de transacciones y pagos
- **Registro de pagos**: Cada transacción se almacena con su importe, fecha y cliente asociado.
- **Pila de últimas transacciones**: Se mantiene un historial de las transacciones recientes para facilitar la revisión.
- **Corrección y deshacer**: Posibilidad de consultar, modificar o anular la última transacción registrada, con total trazabilidad.

###  Historial y búsqueda de clientes
- **Búsqueda rápida por DNI**: Localiza a un cliente en segundos.
- **Visualización del árbol de clientes**: El sistema puede mostrar la lista de clientes en diferentes ordenes (por ejemplo, alfabético o por antigüedad) para una mejor comprensión de los datos.

###  Estado general del hotel
- **Panel de control visual**: Muestra de forma clara y resumida:
  - Habitaciones disponibles
  - Habitaciones ocupadas
  - Habitaciones pendientes de limpieza
- Permite tomar decisiones operativas de un vistazo.

---

##  ¿Cómo empezar?

1. **Instalación**  
   Descarga el instalador desde la sección de [Releases]() y ejecútalo en tu PC.

2. **Configuración inicial**  
   Al primer inicio, el sistema creará automáticamente una base de datos local. Solo necesitas registrar tus habitaciones y ya estás listo para operar.

3. **Uso diario**  
   - Realiza check‑in y check‑out con pocos clics.
   - Consulta el estado del hotel en tiempo real.
   - Gestiona limpiezas y transacciones desde una única interfaz.

---

##  Actualizaciones automáticas

El sistema incluye un mecanismo de actualización integrado. Cuando se publique una nueva versión, recibirás una notificación dentro de la aplicación y podrás actualizar con un solo clic, sin necesidad de descargar manualmente el instalador.

---

##  Requisitos del sistema

- **Sistema operativo**: Windows 10/11, macOS o Linux (distribuciones principales).
- **Espacio en disco**: ~100 MB.
- **Memoria RAM**: Mínimo 512 MB (recomendado 1 GB).

