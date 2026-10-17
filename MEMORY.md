# MEMORY.md — Hostal Cúpido

Memoria del proyecto entre sesiones. Máximo ~50 líneas: resume o elimina lo que ya no aporte.

## Estado actual
- Fase 0 (fundaciones) casi completa: la app arranca en Windows (`pnpm tauri dev`), ventana "Hostal Cúpido" 1280x800 y pantalla de bienvenida con Tailwind.
- Tooling listo: Tailwind v4 con tokens de `DESIGN.md`, ESLint 10 + Prettier, Vitest con smoke test de App, rustfmt/clippy sin warnings.
- CI creado (`.github/workflows/ci.yml`): job quality (lint + formato + tests + clippy) y job build (`tauri build --no-bundle`) en windows-latest.
- PENDIENTE de commit y push: workflow CI, tests de Vitest (`src/App.test.tsx`, `src/test-setup.ts`, `vitest.config.ts`), script `test`, deps de testing y `.mcp.json`.
- SQLite con `rusqlite` aún sin implementar; la Fase 1 trae el dominio en Rust.

## Decisiones (y por qué)
- `eslint-plugin-import-x` en lugar de `eslint-plugin-import` — el clásico es incompatible con ESLint 10 (APIs de sourceCode eliminadas).
- Markdown y configs exentos de Prettier en `.prettierignore` — PLAN.md/README se editan a mano y no debe reformatear generados.
- Builds de postinstall aprobados vía `allowBuilds` en `pnpm-workspace.yaml` (pnpm ≥11 renombró el setting).
- CI solo en runners Windows — plataforma objetivo del hostal; evita instalar webkit2gtk para Linux.
- `tauri build --no-bundle` en CI — empaquetado y firma llegan en Fase 8.
- Sombra/estados de runner: Node 22 y pnpm leído de `packageManager` (12.14.0).

## Aprendizajes y errores a evitar
- En shells cmd de Windows los heredocs de bash fallan (`git commit -F - <<EOF` no sirve): escribir el mensaje en un archivo temporal y usar `git commit -F archivo`.
- Al formatear todo con Prettier revisar `git status`: puede tocar configuraciones y generados ajenos; mantener `.prettierignore` al día.
- Los tests corren en jsdom sin CSS procesado: no hacer assertions visuales por ahora.

## Próximos pasos
- Commitear y push del trabajo de CI/tests; verificar el primer run del workflow.
- Fase 1: dominio puro en Rust con tests — cálculo de tarifas, estados de habitación, cola FIFO de limpieza y pila LIFO de transacciones.
- Diseñar la arquitectura del backend (módulos de db/migraciones/comandos) antes de crear los commands de Tauri.
