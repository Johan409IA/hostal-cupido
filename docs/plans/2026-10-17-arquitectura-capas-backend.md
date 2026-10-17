# 2026-10-17 — Arquitectura de capas del backend en Rust

> Estado: implementado y verificado (clippy, tests y app ejecutándose). Este plan es la decisión vigente documentada también en `docs/architecture.md`.

## Objetivo

Separación de responsabilidades del backend Rust antes de escribir lógica de negocio: capas con dependencias hacia adentro, contrato de errores unificado y estado compartido con pool de conexiones.

## Capas

- `domain/` — entidades, enums y reglas de negocio puras. Sin `tauri`, `rusqlite` ni `r2d2`.
- `repository/` — traits por entidad; `repository/sqlite/` es el único lugar con SQL.
- `commands/` — `#[tauri::command]` finos que validan, orquestan y devuelven `Result<T, AppError>`.
- `error.rs` — `AppError` (thiserror) serializable como `{code, message}`.
- `state.rs` + `db.rs` — `AppState` con pool r2d2 SQLite (`tauri::State`), PRAGMAs WAL + foreign keys.

## Decisiones con su porqué

- Pool r2d2 → pedido explícito; transacciones por request sin bloquear el hilo IPC.
- rusqlite `bundled` → SQLite embebido, app offline sin dependencias del sistema.
- Fechas `i64` epoch (`FechaHora`) → sin `chrono` por ahora; re-evaluable en Fase 1.
- Arquetipo `habitaciones` de extremo a extremo (trait + SQLite + command + tests) → plantilla literal para las demás entidades.
- Módulos raíz `pub` en la lib → evita falsos `dead_code` con `-D warnings` por entidades aún no usadas.
- Bootstrap de solo la tabla `habitaciones` → el esquema completo y migraciones son la tarea aparte de base de datos.

## No entra en este plan

- Reglas de negocio (transiciones, tarifas, FIFO/LIFO) — Fase 1.
- Esquema completo de las 7 tablas ni sistema de migraciones.
- Implementaciones de los otros 6 repositorios (solo traits).

## Pasos ejecutados

1. Deps: `rusqlite` (bundled), `r2d2`, `r2d2_sqlite`, `thiserror`.
2. `error.rs`, `state.rs`, `db.rs` (pool + PRAGMAs + bootstrap).
3. `domain/`: 7 módulos de entidades del PLAN.md §5.
4. `repository/mod.rs`: traits por entidad; `sqlite/habitaciones.rs` arquetipo con tests.
5. `commands/habitaciones.rs` (crear/listar) y `lib.rs` reescrito (setup con pool, sin `greet`).
6. Docs y verificación: clippy, `cargo test` (4 verdes), `pnpm check && pnpm test`, `tauri dev`.
