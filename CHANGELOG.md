# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es/1.1.0/); versionado SemVer. Cambios de software en desarrollo van en `Unreleased` hasta el primer release.

## [Unreleased]

### Added
- Sistema de migraciones versionadas (`src-tauri/src/migrations.rs`): tabla `schema_version`, aplicación automática al iniciar dentro de una transacción por migración y rechazo de bases más nuevas que la app.
- Migración v1: tabla `habitaciones` (antes bootstrap en `db.rs`).
- Esquema base de PLAN §5 como migraciones v2..v7: `clientes`, `estancias`, `transacciones`, `cola_limpieza`, `auditoria` e índices mínimos (DNI, estado de habitación y estado de estancia).
- Validación de contigüidad de versiones al arrancar: un salto en la lista (v3 sin v2, p. ej.) detiene la app en vez de aplicar migraciones fuera de orden.

### Changed
- `db::crear_pool` aplica las migraciones al abrir la base; se eliminó `inicializar_esquema`.
- Tests del backend cubren WAL + `foreign_keys`, el esquema completo y las restricciones de referencia vía `crear_pool` sobre archivos temporales.
