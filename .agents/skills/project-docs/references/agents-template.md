# AGENTS.md — [Nombre del proyecto]

Plantilla base. Mantener el archivo completo en ~30-40 líneas: es releído en cada sesión de agente, así que cada línea de más es costo recurrente.

```markdown
# AGENTS.md — [Nombre del proyecto]
[Una o dos frases: qué es, para quién y cuál es su objetivo.]

## Stack y estructura
- Tecnologías y versiones clave.
- Qué hay en cada carpeta o archivo importante (solo lo que no es obvio).

## Comandos
- Cómo ejecutar, probar, hacer lint y compilar (comandos exactos, copiables).

## Convenciones
- Estilo de código, nombres, idioma de comentarios y textos.
- Patrones que hay que seguir (y cuál es el archivo de referencia).

## Reglas de dominio / trampas conocidas
- Lo que es fácil hacer mal y el agente no puede deducir leyendo el código.

## Forma de trabajar
- Cuándo planificar antes de tocar código, tamaño de los cambios, qué explicar al terminar.

## Memoria
- Al empezar, lee `MEMORY.md` para conocer el estado del proyecto y las decisiones tomadas.
- Al terminar una tarea, actualízalo: estado actual, decisiones importantes (con su porqué) y errores a evitar.
- Mantenlo breve (máximo ~50 líneas): resume o elimina lo que ya no aporte.
- Si algo se convierte en una regla permanente, propón moverlo aquí en lugar de dejarlo en la memoria.
- No guardes nunca datos sensibles (claves, tokens, datos personales).

## Límites
- ✅ Siempre: lo que debe hacer sin preguntar, incluyendo actualizar `MEMORY.md` al terminar cada tarea.
- ⚠️ Pregunta antes: dependencias nuevas, archivos nuevos, cambios en el formato de datos…
- 🚫 Nunca: lo que no debe tocar bajo ningún concepto.

## Verificación
- Cómo comprobar que un cambio funciona antes de darlo por terminado.
```

## Qué incluir (resumen)

- Stack tecnológico
- Convenciones de código
- Patrones
- Prohibiciones
- Estructura de proyecto
- Flujo de trabajo
- Testing, CI/CD
- Estilo de commits y PRs

Referencia: https://agents.md
