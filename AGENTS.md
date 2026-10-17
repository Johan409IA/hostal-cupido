# AGENTS.md — Hostal Cúpido

Sistema de escritorio (Tauri v2) para administrar la operación diaria de un hostal pequeño: habitaciones, clientes, estancias y transacciones. Objetivo: app offline para Windows con SQLite local.

## Stack y estructura
- Frontend: React 19 + TypeScript + Vite 7 + Tailwind CSS v4 (tokens de diseño en `src/index.css`, fuente en `DESIGN.md`).
- Backend: Rust + Tauri v2 en `src-tauri/` (SQLite con `rusqlite` planificado, aún no implementado).
- Tooling: ESLint 10 + Prettier + Vitest (front), rustfmt + clippy (back), CI en `.github/workflows/ci.yml`. `PLAN.md` es el mapa de fases.

## Comandos
- Instalar: `pnpm install` · Arrancar app: `pnpm tauri dev` (solo frontend: `pnpm dev`).
- Quality gate: `pnpm check` (lint + formato + build) y `pnpm test` (Vitest).
- Backend (en `src-tauri/`): `cargo fmt --check` · `cargo clippy --all-targets --all-features -- -D warnings` · `cargo test`.
- Build sin instaladores: `pnpm tauri build --no-bundle`.

## Convenciones
- Identificadores en inglés; comentarios y textos de UI en español.
- Imports ordenados (`import-x/order`), formato por Prettier, Tailwind en vez de CSS plano.
- Usar `ctx7` para docs de tecnologías, GitHub CLI (`gh`) para CI, y las skills de tauri para la plataforma.
- Rol único en la app: administrador.

## Reglas de dominio (trampas)
- Estados de habitación solo: Disponible → Ocupada → Sucia → Disponible (ninguna otra transición).
- Solo la transacción en el tope de la pila LIFO se anula o corrige; nunca se borran pagos ni auditorías.
- Los importes los calcula siempre el backend; el frontend solo previsualiza.
- No tocar `src-tauri/gen/`, `dist/` ni `target/` (generados).

## Memoria
- Al empezar, lee `MEMORY.md` para conocer el estado del proyecto y las decisiones tomadas.
- Al terminar una tarea, actualízalo: estado actual, decisiones importantes (con su porqué) y errores a evitar.
- Mantenlo breve (máximo ~50 líneas): resume o elimina lo que ya no aporte.
- Si algo se convierte en una regla permanente, propón moverlo aquí en lugar de dejarlo en la memoria.
- No guardes nunca datos sensibles (claves, tokens, datos personales).

## Límites
- ✅ Siempre: actualizar `MEMORY.md` al terminar cada tarea y correr `pnpm check` + `pnpm test` antes de dar por terminado el trabajo.
- ⚠️ Pregunta antes: dependencias nuevas, archivos nuevos, cambios de esquema de datos, commits y push.
- 🚫 Nunca: force push, ni escribir secretos (claves, tokens) en el código o las docs.

## Verificación
- `pnpm check && pnpm test` local; el CI (`windows-latest`) corre lo mismo más clippy y `tauri build --no-bundle`.
- Manual: `pnpm tauri dev` y probar el flujo afectado.
