---
name: project-docs
description: Create or update a project's documentation set — README.md, AGENTS.md, MEMORY.md, CHANGELOG.md, docs/architecture.md, docs/plans/, and API docs (OpenAPI + Scalar). Use this whenever the user asks to "document my project", "documenta mi proyecto", "crea/actualiza el AGENTS.md o MEMORY.md", "genera el README", "actualiza el CHANGELOG", "escribe el architecture.md", or mentions keeping agent instructions/memory/plans in sync after finishing work — even if they only name one file, since these docs are interdependent and should be checked together.
---

# Project Docs

Helps create and keep up to date the full documentation set of a software project, following a fixed structure and a fixed relationship between `AGENTS.md` and `MEMORY.md`. Use this any time the user wants documentation generated from scratch, refreshed after changes, or kept consistent across files.

## Target structure

Every project this skill manages should converge toward this layout:

```
mi-proyecto/
├── README.md          # qué es, cómo levantarlo
├── AGENTS.md           # reglas e instrucciones para agentes
├── MEMORY.md            # decisiones vigentes y lo descartado
├── CHANGELOG.md          # qué cambió entre versiones
└── docs/
    ├── architecture.md
    ├── plans/            # planes de los agentes, uno por archivo
    └── (OpenAPI + Scalar servido desde la API en /docs)
```

If the project already has some of these files, read them first and update in place — don't blindly overwrite. If a file is missing, create it following the matching template below.

## Workflow

1. **Survey what exists.** List the project root and `docs/` to see which files are already there. Read any existing `AGENTS.md`/`MEMORY.md`/`CHANGELOG.md`/`README.md`/`docs/architecture.md` in full — they're short by design.
2. **Figure out scope.** If the user names a specific file ("actualiza el CHANGELOG"), focus there, but still check whether that change should ripple into `MEMORY.md` or `AGENTS.md` (see the propagation rule below). If the user says something general like "documenta mi proyecto" or "actualiza la documentación", go through every file in the structure above.
3. **Gather the facts** you need from the codebase itself (package manifests, entry points, scripts, routes, recent commits) rather than guessing. Don't invent commands, stack details, or endpoints — verify them by reading the code or config files.
4. **Write or update each file** using the matching reference template. Keep the strict length limits on `AGENTS.md` (~30-40 lines) and `MEMORY.md` (~50 lines) — these are load-bearing constraints, not suggestions, because agents re-read these files every session and bloated versions stop being useful.
5. **Report back** which files were created vs. updated, and flag anything you couldn't verify (e.g. "no encontré un comando de test, confírmalo").

## The documents

### README.md
Answers "what is this and how do I run it" for a human. See `references/readme-template.md`. Pull the run/install/test commands straight from the project's actual scripts (`package.json`, `Makefile`, `pyproject.toml`, etc.) — never guess at command names.

### AGENTS.md
Instructions for coding agents working in this repo. See `references/agents-template.md` for the exact sections and ordering. Hard limit: ~30-40 lines total. If it's already longer, your job when updating it is as much about *trimming* as adding — move anything that reads like a transient decision or an in-progress note into `MEMORY.md` instead, and keep only durable rules here.

### MEMORY.md
Session-to-session memory: what's true right now, why key decisions were made, what was tried and rejected, what's next. See `references/memory-template.md`. Hard limit: ~50 lines. When updating, actively prune — delete or compress entries that no longer matter rather than letting the file grow forever.

**How AGENTS.md and MEMORY.md relate** (this is the core convention — get it right):
- `AGENTS.md` must contain a short "Memoria" section telling agents to read `MEMORY.md` at the start of a task and update it at the end. Use this exact wording as a base, adapting only if the project already phrases it differently:
  ```markdown
  ## Memoria
  - Al empezar, lee `MEMORY.md` para conocer el estado del proyecto y las decisiones tomadas.
  - Al terminar una tarea, actualízalo: estado actual, decisiones importantes (con su porqué) y errores a evitar.
  - Mantenlo breve (máximo ~50 líneas): resume o elimina lo que ya no aporte.
  - Si algo se convierte en una regla permanente, propón moverlo a `AGENTS.md` en lugar de dejarlo en la memoria.
  - No guardes nunca datos sensibles (claves, tokens, datos personales).
  ```
- `AGENTS.md`'s "Límites" section must include `✅ Siempre: actualizar MEMORY.md al terminar cada tarea.`
- **Propagation rule**: `MEMORY.md` holds things that are true *for now* — current state, decisions with their reasoning, lessons learned, next steps. `AGENTS.md` holds things that are true *always* — permanent rules, conventions, stack, limits. When something in `MEMORY.md` stops being a one-off decision and becomes a standing rule (e.g. "we decided to never use X" turns into "never use X, period"), move it up into `AGENTS.md` and remove it from `MEMORY.md`. When updating either file, always check whether this promotion should happen.
- Never write secrets, tokens, or personal data into either file.

### CHANGELOG.md
Answers "what changed between versions." Follow the [Keep a Changelog](https://keepachangelog.com) convention (`Added`/`Changed`/`Fixed`/`Removed` under each version, newest on top, with an `Unreleased` section at the top for work in progress). See `references/changelog-template.md`. When updating, add entries under `Unreleased` as work happens, and only cut a new dated version section when the user indicates a release.

### docs/architecture.md
Answers "how is this built" — components, data flow, key design decisions and why, notable constraints. See `references/architecture-template.md`. This is the one doc that can run longer than the others since it's a reference doc, not something re-read every session — but still favor diagrams and short sections over long prose. Use a mermaid diagram for the main component/data-flow view when it helps.

### docs/plans/
When an agent produces a plan for a non-trivial task (a multi-step implementation plan, a migration plan, a refactor plan), save it here as its own file instead of only leaving it in chat. Create the directory if it doesn't exist. Name files `YYYY-MM-DD-short-slug.md` (e.g. `2026-10-03-add-payment-webhook.md`) so they sort chronologically. Each plan file should be self-contained: goal, approach, steps, open questions. Don't delete old plans after they're done — they're a historical record; if the project wants a "done" marker, add a one-line status note at the top of the file instead of removing it.

### API docs — OpenAPI + Scalar
These are **not** a static markdown file — they're served live from the API, typically at a `/docs` or `/reference` route via [Scalar](https://scalar.com)'s API reference renderer reading an OpenAPI spec. When asked to set up or update API docs:
1. Check whether the project already generates an OpenAPI spec (framework decorators/annotations, a `openapi.yaml`/`openapi.json` file, or a spec-generation library already in dependencies).
2. If the spec exists but isn't served via Scalar yet, wire up the Scalar middleware/handler for the project's framework (Scalar has official integrations for most common backend frameworks — check current package names/APIs rather than assuming, since this ecosystem moves fast) and mount it at `/docs`.
3. If no spec exists yet, generate one from the existing routes/handlers (most frameworks have a library that derives OpenAPI from route definitions/types — prefer that over hand-writing YAML) before wiring up Scalar.
4. Never hand-maintain a giant static OpenAPI YAML by hand if the framework can derive it from code — that drifts out of sync immediately. Prefer code-first generation.
5. Note the `/docs` URL in `README.md` once it's set up, so humans know where to find it.

## Checklist before finishing

- [ ] `AGENTS.md` is ≤ ~40 lines and has a "Memoria" section pointing at `MEMORY.md`
- [ ] `MEMORY.md` is ≤ ~50 lines and nothing permanent is stranded there that belongs in `AGENTS.md`
- [ ] `README.md` commands were verified against actual project config, not assumed
- [ ] `CHANGELOG.md` follows Keep a Changelog format with an `Unreleased` section
- [ ] `docs/architecture.md` reflects the current structure, not a stale one
- [ ] New agent plans are saved under `docs/plans/` with a dated filename
- [ ] Nothing sensitive (keys, tokens, personal data) was written into any doc
