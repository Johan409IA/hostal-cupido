# Hostal Cúpido — Sistema de Gestión Hotelera

Sistema de escritorio para la administración integral de un pequeño hotel u hostal. Diseñado para optimizar las operaciones diarias, ofrecer un control rápido y fiable de habitaciones, clientes y transacciones, y proporcionar una experiencia ágil tanto para el personal como para la gerencia.

> Proyecto en desarrollo: la Fase 0 (fundaciones) está completa. El plano completo de fases está en [`PLAN.md`](./PLAN.md).

---

## Características principales

### Gestión de habitaciones
- **Registro de habitaciones**: Alta de nuevas habitaciones con número, tipo (normal o matrimonial) y tarifa asociada.
- **Consulta de disponibilidad**: Visualización en tiempo real del estado de cada habitación.
- **Control de estados**: Cada habitación puede estar en uno de tres estados:
  - **Disponible** – lista para asignar a un cliente.
  - **Ocupada** – actualmente en uso.
  - **Sucia** – pendiente de limpieza tras un check-out.

### Gestión de clientes
- **Registro por DNI**: Cada cliente se identifica de forma única mediante su documento de identidad.
- **Historial de estancias**: Se mantiene un registro completo de todas las visitas del cliente para agilizar futuros check‑in.

### Registro de ingresos (check‑in)
- **Asignación de habitación**: El sistema selecciona una habitación disponible y la asigna al cliente.
- **Dos modalidades de estancia**:
  - **Día completo** (24 horas)
  - **Estancia momentánea** (4 horas)
- **Cálculo automático**: Se calcula la hora de salida y el importe total en función de la tarifa y la modalidad elegida.

### Gestión de limpieza
- **Cola FIFO**: Cuando una habitación queda sucia, se encola automáticamente. La limpieza se atiende por orden de llegada (la que lleva más tiempo esperando es la primera en limpiarse).
- **Actualización de estado**: Al finalizar la limpieza, la habitación vuelve a estar disponible.

### Control de transacciones y pagos
- **Registro de pagos**: Cada transacción se almacena con su importe, fecha y cliente asociado.
- **Pila de últimas transacciones**: Se mantiene un historial de las transacciones recientes para facilitar la revisión.
- **Corrección y deshacer**: Posibilidad de consultar, modificar o anular la última transacción registrada, con total trazabilidad.

### Historial y búsqueda de clientes
- **Búsqueda rápida por DNI**: Localiza a un cliente en segundos.
- **Visualización del árbol de clientes**: El sistema puede mostrar la lista de clientes en diferentes ordenes (por ejemplo, alfabético o por antigüedad) para una mejor comprensión de los datos.

### Estado general del hotel
- **Panel de control visual**: Muestra de forma clara y resumida:
  - Habitaciones disponibles
  - Habitaciones ocupadas
  - Habitaciones pendientes de limpieza
- Permite tomar decisiones operativas de un vistazo.

---

## Requisitos

- **Node.js** 22+
- **pnpm** 12 (fijado en `package.json` vía `packageManager`; con `corepack enable` se usa la versión correcta)
- **Rust** (canal stable) para el backend
- **WebView2** (solo Windows; viene preinstalado en Windows 10/11)

## Instalación

```sh
git clone https://github.com/Johan409IA/hostal-cupido.git
cd hostal-cupido
pnpm install
```

## Desarrollo

```sh
pnpm tauri dev   # app de escritorio completa (frontend + backend Rust)
pnpm dev         # solo el frontend con recarga en caliente, sin backend
```

La base de datos SQLite se creará automáticamente al primer arranque de la app (implementación planificada).

## Tests y calidad

```sh
pnpm test        # tests del frontend (Vitest)
pnpm check       # lint + formato + build (quality gate local)
```

Comandos del backend, dentro de `src-tauri/`:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## CI

GitHub Actions corre en cada push a `main` y en cada pull request (`windows-latest`): lint, formato, tests del frontend, `rustfmt`/`clippy`/tests del backend y un build completo con `tauri build --no-bundle`. Ver `.github/workflows/ci.yml`.

## Build

```sh
pnpm tauri build --no-bundle   # binario de la app sin instaladores
```

Los instaladores firmados (MSI/NSIS) y las actualizaciones automáticas llegarán en la Fase 8.

---

## Requisitos del sistema (app instalada)

- **Sistema operativo**: Windows 10/11 (el empaquetado está centrado en Windows; macOS y Linux son objetivos futuros).
- **Espacio en disco**: ~100 MB.
- **Memoria RAM**: Mínimo 512 MB (recomendado 1 GB).

---

## Documentación adicional

- Roadmap por fases: [`PLAN.md`](./PLAN.md)
- Sistema de diseño: [`DESIGN.md`](./DESIGN.md)
- Reglas e instrucciones para agentes: [`AGENTS.md`](./AGENTS.md)
- Memoria del proyecto entre sesiones: [`MEMORY.md`](./MEMORY.md)
