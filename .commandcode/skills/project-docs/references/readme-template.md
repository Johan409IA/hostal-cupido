# README.md — plantilla

El README es para humanos nuevos en el proyecto. Responde: ¿qué es esto? ¿cómo lo levanto? ¿cómo lo uso? Verifica cada comando contra el `package.json`/`Makefile`/`pyproject.toml`/etc. real del proyecto antes de escribirlo — nunca asumas nombres de scripts.

```markdown
# [Nombre del proyecto]

[Una o dos frases: qué hace el proyecto y para quién.]

## Requisitos

- [Runtime/lenguaje y versión]
- [Otras dependencias del sistema, si aplica]

## Instalación

```sh
[comando real de instalación]
```

## Uso / cómo levantarlo

```sh
[comando real para correr en desarrollo]
```

[Variables de entorno necesarias, si las hay, y dónde documentarlas (.env.example, etc.)]

## Tests

```sh
[comando real de test]
```

## Documentación adicional

- Arquitectura: `docs/architecture.md`
- API (OpenAPI + Scalar): `[URL, p. ej. http://localhost:PORT/docs]`
- Reglas para agentes: `AGENTS.md`
- Historial de cambios: `CHANGELOG.md`
```

Mantén el README enfocado en el "cómo"; detalles de diseño van en `docs/architecture.md`, y reglas para agentes van en `AGENTS.md`/`MEMORY.md`.
