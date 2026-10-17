# MEMORY.md — Hostal Cúpido

Memoria del proyecto entre sesiones. Máximo ~50 líneas: resume o elimina lo que ya no aporte.

## Estado actual
- Fase 0 (fundaciones) casi completa: la app arranca en Windows (`pnpm tauri dev`), ventana "Hostal Cúpido" 1280x800 y pantalla de bienvenida con Tailwind.
- Tooling listo: Tailwind v4 con tokens de `DESIGN.md`, ESLint 10 + Prettier, Vitest con smoke test de App, rustfmt/clippy sin warnings.
- CI creado (`.github/workflows/ci.yml`): job quality (lint + formato + tests + clippy) y job build (`tauri build --no-bundle`) en windows-latest.
- Primer run del CI falló en `prettier --check`: `core.autocrlf=true` da CRLF en fresh checkouts de Windows y Prettier exige LF. Corregido con `.gitattributes` (`* text=auto eol=lf`) + renormalize del índice y Node 24 en el workflow; ver run más reciente con `gh run list`.
- SQLite con `rusqlite` aún sin implementar; la Fase 1 trae el dominio en Rust.
- Fase 0: arquitectura de capas implementada — `domain/` puro, `repository/` (traits por entidad + `sqlite/` único con SQL), `commands/` finos, `error.rs` (`AppError` → `{code,message}`), `state.rs`/`db.rs` (pool r2d2, WAL+FK). Arquetipo `habitaciones` con tabla bootstrap y tests cargo (4 verdes). No entra aquí: esquema completo, migraciones versionadas y demás repos (véase `docs/architecture.md` y `docs/plans/2026-10-17-arquitectura-capas-backend.md`).

## Decisiones (y por qué)
- `eslint-plugin-import-x` en lugar de `eslint-plugin-import` — el clásico es incompatible con ESLint 10 (APIs de sourceCode eliminadas).
- Markdown y configs exentos de Prettier en `.prettierignore` — PLAN.md/README se editan a mano y no debe reformatear generados.
- Builds de postinstall aprobados vía `allowBuilds` en `pnpm-workspace.yaml` (pnpm ≥11 renombró el setting).
- CI solo en runners Windows — plataforma objetivo del hostal; evita instalar webkit2gtk para Linux.
- `tauri build --no-bundle` en CI — empaquetado y firma llegan en Fase 8.
- Sombra/estados de runner: Node 24 y pnpm leído de `packageManager` (12.14.0).

## Aprendizajes y errores a evitar
- En shells cmd de Windows los heredocs de bash fallan (`git commit -F - <<EOF` no sirve): escribir el mensaje en un archivo temporal y usar `git commit -F archivo`.
- El checkout de CI y los clones en Windows dan CRLF sin `.gitattributes` (`* text=auto eol=lf`); mantenerlo y renormalizar si un archivo nuevo entra con CRLF.
- Al formatear todo con Prettier revisar `git status`: puede tocar configuraciones y generados ajenos; mantener `.prettierignore` al día.
- Los tests corren en jsdom sin CSS procesado: no hacer assertions visuales por ahora.
- Con `cargo clippy -D warnings`, los `pub` items dentro de módulos raíz privados disparan `dead_code` aunque no se usen: los módulos de la lib (`domain`, `repository`, etc.) se declaran `pub` en `lib.rs`.

## Próximos pasos
- Verificar que el run del CI tras la corrección CRLF pasa en verde.
- Tarea de base de datos: esquema completo de las 7 tablas (PLAN §5) y sistema de migraciones versionadas reemplazando el bootstrap de `db.rs`.
- Fase 1: dominio puro en Rust con tests — cálculo de tarifas, estados de habitación, cola FIFO de limpieza y pila LIFO de transacciones.
