# CHANGELOG.md — plantilla

Sigue la convención [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/): más reciente arriba, agrupado por versión, con una sección `Unreleased` al tope para trabajo en curso que aún no se publicó.

```markdown
# Changelog

Todos los cambios notables de este proyecto se documentan aquí.
El formato sigue [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/).

## [Unreleased]

### Added
- [Nueva funcionalidad aún no liberada]

### Changed
- [Cambio de comportamiento existente]

### Fixed
- [Corrección de bug]

### Removed
- [Funcionalidad eliminada]

## [1.1.0] - YYYY-MM-DD

### Added
- [...]

## [1.0.0] - YYYY-MM-DD

### Added
- Versión inicial.
```

## Reglas al actualizar

- Añade entradas bajo `Unreleased` a medida que se completan tareas, no solo al momento de publicar.
- Solo crea una nueva sección con versión y fecha cuando el usuario indique que se está cortando un release.
- Usa solo las categorías que apliquen (`Added`, `Changed`, `Fixed`, `Removed`, `Deprecated`, `Security`); omite las vacías.
- Una línea por cambio, en lenguaje de usuario/desarrollador, no un resumen del diff técnico.
