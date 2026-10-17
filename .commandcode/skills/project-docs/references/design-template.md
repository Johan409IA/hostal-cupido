# DESIGN.md — plantilla

Responde "cómo se ve y cómo se construye la interfaz". Es el sistema de diseño visual del proyecto, escrito para que los agentes que generan UI mantengan la misma identidad en todas las pantallas. Vive en la **raíz del proyecto**, junto a `AGENTS.md` (no dentro de `docs/`).

Sigue el formato abierto de Google Stitch (versión `alpha`): [documentación](https://stitch.withgoogle.com/docs/design-md/overview/) · [especificación](https://github.com/google-labs-code/design.md). Al ser un documento de referencia no tiene un límite estricto de líneas (como `docs/architecture.md`), pero cada sección debe ser corta y enfocada: 3-6 viñetas o un párrafo breve.

Los **encabezados de sección van en inglés, exactamente como en la spec** (las herramientas compatibles los reconocen por nombre). La prosa se escribe en el idioma del proyecto.

````markdown
---
version: alpha
name: [Nombre del proyecto]
description: [Una frase: qué tipo de interfaz es y qué sensación transmite.]

# Paleta. Los hex van entre comillas: sin ellas YAML interpreta "#" como comentario.
colors:
  primary: "#RRGGBB"        # Acción principal y elementos interactivos clave
  on-primary: "#RRGGBB"     # Texto/íconos sobre `primary`
  secondary: "#RRGGBB"      # Acciones de apoyo, bordes, texto secundario
  tertiary: "#RRGGBB"       # Acento / estado interactivo (hover, foco, enlaces)
  surface: "#RRGGBB"        # Fondo de páginas, tarjetas e inputs
  on-surface: "#RRGGBB"     # Texto principal sobre `surface`
  error: "#RRGGBB"          # Validaciones y acciones destructivas

# Escala tipográfica. Los nombres son libres; usa nombres semánticos.
typography:
  h1:
    fontFamily: [Fuente de títulos]
    fontSize: 2.5rem
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: -0.02em
  h2:
    fontFamily: [Fuente de títulos]
    fontSize: 1.75rem
    fontWeight: 600
    lineHeight: 1.3
  body-md:
    fontFamily: [Fuente de texto]
    fontSize: 1rem
    fontWeight: 400
    lineHeight: 1.5
  label-md:
    fontFamily: [Fuente de texto]
    fontSize: 0.875rem
    fontWeight: 500
    lineHeight: 1.25

# Radios de esquina
rounded:
  sm: 4px
  md: 8px
  lg: 16px
  full: 9999px

# Escala de espaciado (padding, margin, gap)
spacing:
  xs: 4px
  sm: 8px
  md: 16px
  lg: 24px
  xl: 32px

# Componentes base. Referencian tokens con {ruta.al.token}.
# Las variantes (hover, active, disabled) son entradas aparte con sufijo.
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.on-primary}"
    typography: "{typography.label-md}"
    rounded: "{rounded.md}"
    padding: "12px 20px"
  button-primary-hover:
    backgroundColor: "{colors.tertiary}"
    textColor: "{colors.on-primary}"
  button-secondary:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.secondary}"
    rounded: "{rounded.md}"
    padding: "12px 20px"
  input:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    padding: "{spacing.sm}"
  input-error:
    textColor: "{colors.error}"
  card:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.on-surface}"
    rounded: "{rounded.lg}"
    padding: "{spacing.lg}"
---

## Overview
[Identidad visual en pocas líneas: personalidad de la marca, público objetivo, tono (serio/juguetón, denso/espacioso) y la emoción que debe provocar la interfaz. Sirve de criterio para decisiones que ningún token cubre.]

## Colors
- `primary`: [para qué se usa y cuándo; ej. "solo la acción más importante de cada pantalla"].
- `secondary`: [rol].
- `tertiary`: [rol].
- `surface` / `on-surface`: [fondos y texto base].
- `error`: [cuándo aparece].
- [Regla de contraste: texto sobre fondo ≥ 4.5:1 (WCAG AA).]

## Typography
[Fuentes usadas y por qué se emparejan, jerarquía (h1 → body → label), pesos permitidos y reglas de uso. Menciona los tokens por nombre (`h1`, `body-md`).]

## Layout
[Modelo de layout (grid fluido, ancho máximo fijo, márgenes), uso de la escala `spacing` y principios de agrupación. Comportamiento responsive: breakpoints y qué cambia en cada uno.]

## Elevation & Depth
[Cómo se comunica la jerarquía visual: sombras (blur, spread, color) y en qué niveles se usan; o, si el diseño es plano, qué las reemplaza (bordes, contraste de color).]

## Shapes
[Dónde se usa cada radio de `rounded` (botones, inputs, tarjetas, avatares) y la regla de consistencia.]

## Components
[Patrones de los componentes clave: tamaños, padding y estados (hover, foco, deshabilitado, error). Indica qué componente existente reutilizar antes de crear uno nuevo y dónde vive en el código.]

## Do's and Don'ts
- ✅ [Regla concreta, ej. "Usa `primary` una sola vez por pantalla".]
- ✅ [Regla concreta.]
- 🚫 [Error común a evitar, ej. "No mezcles esquinas redondeadas y rectas en la misma vista".]
- 🚫 [Error común a evitar.]

<!-- Secciones extra OPCIONALES: van siempre después de "Do's and Don'ts". Borra las que no apliquen. -->

## Iconography & Imagery
[Librería de íconos, tamaño y grosor de trazo, estilo de ilustraciones/fotografía.]

## Motion
[Duraciones y curvas de easing, qué se anima y qué no, respeto a `prefers-reduced-motion`.]

## Accessibility
[Contraste mínimo, foco visible, tamaños mínimos de toque, soporte de modo oscuro si existe.]
````

## Qué hace cada bloque

- **Bloque `---` (frontmatter YAML)**: son los *tokens*, los valores normativos que un agente copia tal cual al generar UI. Va siempre al inicio del archivo, entre dos líneas `---`.
  - `version` / `name` / `description`: identifican el sistema de diseño. Solo `name` es obligatorio; `version` es hoy `alpha`.
  - `colors`: la paleta con su rol. Debe existir `primary`, o el linter avisa (`missing-primary`).
  - `typography`: la escala tipográfica. `lineHeight` es mejor como multiplicador sin unidad (`1.5`).
  - `rounded` y `spacing`: las escalas de radios y de espaciado que usarán los componentes.
  - `components`: combina los tokens anteriores en componentes concretos con la sintaxis `{colors.primary}`. Si una referencia no existe, el linter da error (`broken-ref`).
- **Cuerpo markdown (`## Overview` … `## Do's and Don'ts`)**: explica el *porqué* de los tokens. Los tokens dicen *qué* valor, la prosa dice *cuándo y por qué* usarlo; por eso la prosa cita los tokens por nombre en vez de repetir los hex.
- **Secciones extra (`Iconography & Imagery`, `Motion`, `Accessibility`)**: no son parte del orden canónico. La spec las conserva sin error, pero deben ir después de `Do's and Don'ts` para no disparar el aviso `section-order`.

## Reglas al actualizar

- **Solo para proyectos con interfaz** (web, móvil, escritorio). Si el proyecto es una API, CLI o librería sin UI, no crees este archivo.
- Saca los valores del código real: configuración de Tailwind, variables CSS (`:root`, `globals.css`), tema de la librería de UI (shadcn `components.json`, MUI, Chakra), fuentes cargadas, exportaciones de Figma/DTCG. No inventes colores ni fuentes. Si el proyecto aún no tiene identidad visual definida, propón valores y márcalos en el reporte como "propuesta, confirmar".
- Si ya existe un `DESIGN.md`, léelo y actualízalo en sitio; no lo sobrescribas. Si el código cambió (nuevo color, otra fuente), actualiza los tokens para que no se desfasen.
- Los valores viven **solo en el frontmatter**; la prosa no redefine un token con otro valor.
- Orden canónico: Overview → Colors → Typography → Layout → Elevation & Depth → Shapes → Components → Do's and Don'ts. Se pueden omitir secciones, pero no reordenarlas ni duplicar un encabezado (un encabezado duplicado es error). Alias aceptados: `Brand & Style` (Overview), `Layout & Spacing` (Layout), `Elevation` (Elevation & Depth).
- Borra de la plantilla lo que el proyecto no use (tokens, componentes, secciones extra): el linter avisa de colores que ningún componente referencia (`orphaned-tokens`).
- Valida con `npx @google/design.md lint DESIGN.md`. Corrige los errores y los avisos razonables (`missing-primary`, `contrast-ratio`, `section-order`, `missing-typography`).
- Nunca escribas datos sensibles (claves, tokens, URLs privadas) en este archivo.
