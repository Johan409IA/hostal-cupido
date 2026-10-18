# MEMORY.md — Hostal Cúpido

Memoria del proyecto entre sesiones. Máximo ~50 líneas: resume o elimina lo que ya no aporte.

## Estado actual
- Fase 0 completa: app arranca en Windows con `pnpm tauri dev`, CI en verde y base SQLite con migraciones versionadas.
- Backend por capas: `domain/` puro, `repository/` (traits + `sqlite/` con SQL), `commands/` finos, `migrations.rs`, `db.rs` (pool r2d2, WAL+FK vía `with_init`). Arquetipo `habitaciones` end-to-end.
- Fase 1 en curso en `feature/estructura-de-datos`: entidades del dominio declaradas (solo structs/enums, aún sin reglas) y pila LIFO de transacciones implementada (`domain/transaccion.rs::PilaTransaccionesRecientes` con `apilar`/`ver_ultima`/`deshacer(motivo)`) — 17 tests cargo + 1 Vitest en verde.
- Migraciones: `schema_version` (una fila por migración); v1..v7 crean el esquema base; cada migración aplica SQL + registro de versión en una transacción; base más nueva que la app no arranca; dev-bases convergen solas.

## Decisiones (y por qué)
- Pila LIFO pura en dominio: `deshacer(motivo)` devuelve la transacción ya marcada `Anulada` (con su motivo) y el borrado físico nunca existe — el repositorio que persiste esa marca llega en Fase 6, siguiendo el trait `TransaccionRepository::anular(id, motivo)`.
- Migraciones como lista `const` en `migrations::LISTA` en vez de librería externa: sin dependencias nuevas y suficiente para el alcance.
- Guard "base más nueva" con `AppError::Validacion` en vez de un código de error nuevo: el contrato con el frontend no cambia.
- Una fila por migración en `schema_version`, con fecha epoch i64 como en `domain::FechaHora`.
- Índice explícito sobre `clientes.dni` aunque el UNIQUE ya crea autoíndice: lo exigió la tarea y el test de índices lo pina por nombre.
- `eslint-plugin-import-x` en lugar de `eslint-plugin-import` — el clásico es incompatible con ESLint 10.
- Markdown y configs exentos de Prettier en `.prettierignore` — PLAN.md/README se editan a mano.
- Builds de postinstall aprobados vía `allowBuilds` en `pnpm-workspace.yaml` (pnpm ≥11 renombró el setting).
- CI solo en runners Windows — plataforma objetivo del hostal; `tauri build --no-bundle` hasta Fase 8.

## Aprendizajes y errores a evitar
- En shells cmd de Windows los heredocs de bash fallan: escribir el mensaje en un archivo temporal y usar `git commit -F archivo`.
- Checkout/clones en Windows dan CRLF sin `.gitattributes` (`* text=auto eol=lf`); mantenerlo y renormalizar si entra un archivo nuevo con CRLF.
- Al formatear con Prettier revisar `git status`: puede tocar configs y generados ajenos.
- Tests corren en jsdom sin CSS procesado: no hacer assertions visuales.
- Con `cargo clippy -D warnings`, los `pub` items en módulos raíz privados disparan `dead_code`: los módulos de la lib se declaran `pub` en `lib.rs`.
- Un pool construido a mano en tests NO pasa por `with_init`: para verificar WAL/FK usar `db::crear_pool` con archivo temporal, no un builder manual.
- Vitest a veces flaquea al spawnear el worker tras una compilación pesada de Cargo (timeout): reintentar antes de investigar.

## Próximos pasos
- Fase 1: dominio puro en Rust con tests — cálculo de tarifas, estados de habitación, cola FIFO y pila LIFO.
- `administrador` y `configuracion` (PLAN §5) llegan en Fase 2 como migraciones v8+.
- Verificar que el run del CI pasa en verde tras estos cambios.
