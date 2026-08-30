# PLAN.md


# Sistema de Gestión Hotelera Hostal Cúpido
## 1\. Resumen
Aplicación de escritorio multiplataforma para administrar la operación diaria de un pequeño hostal: habitaciones, clientes, estancias, check-in, check-out, limpieza, pagos, transacciones, auditoría, backups y actualizaciones.

**Repositorio:** [https://github.com/Johan409IA/hostal-cupido](https://github.com/Johan409IA/hostal-cupido)
## 2\. Stack técnico
*   **Desktop:** Tauri v2.
*   **Frontend:** React + TypeScript + Vite.
*   **Backend:** Rust.
*   **Base de datos:** SQLite local, usando `rusqlite` con SQLite embebido.
*   **Estilos:** Tailwind CSS.
*   **Distribución:** GitHub Releases, con `tauri-plugin-updater`.
*   **CI/CD:** GitHub Actions.
## 3\. Alcance funcional
*   Un único rol: administrador del hostal.
*   Login local con contraseña protegida mediante Argon2id.
*   Configuración inicial en el primer arranque.
*   CRUD de habitaciones: número, tipo normal o matrimonial y tarifas.
*   Estados de habitación: Disponible, Ocupada y Sucia.
*   Registro y búsqueda de clientes por DNI único.
*   Historial completo de estancias por cliente.
*   Check-in para día completo de 24 horas o estancia momentánea de 4 horas.
*   Cálculo automático de hora de salida e importe.
*   Check-out explícito con cálculo de excedentes.
*   Cola FIFO de limpieza.
*   Registro de pagos y pila LIFO de transacciones recientes.
*   Anulación y corrección de la última transacción, con trazabilidad.
*   Dashboard operativo.
*   Reporte de caja diaria.
*   Backup automático, exportación manual y restauración.
*   Empaquetado para Windows.
*   Actualizaciones automáticas firmadas.
## 4\. Reglas de negocio principales
### Habitaciones
*   `Disponible → Ocupada` al confirmar un check-in.
*   `Ocupada → Sucia` al confirmar un check-out.
*   `Sucia → Disponible` al finalizar la limpieza.
*   No se permiten otras transiciones.
*   Una habitación ocupada no puede darse de baja.
### Estancias
*   Día completo: duración de 24 horas.
*   Momentánea: duración de 4 horas.
*   El backend calcula el importe definitivo; el frontend solo muestra una previsualización.
*   El check-out siempre es explícito.
*   Si se supera la hora prevista, se cobra un excedente o se condona dejando un motivo.
### Limpieza
*   Cada check-out agrega la habitación a una cola FIFO persistida.
*   Se atiende primero la habitación que lleva más tiempo esperando.
*   Una limpieza finalizada cambia el estado a Disponible.
### Transacciones
*   Cada pago conserva importe, fecha, cliente, estancia y método de pago.
*   La transacción más reciente se mantiene en el tope de una pila LIFO.
*   Solo la última transacción puede anularse o corregirse.
*   Nunca se borran físicamente pagos ni auditorías.
## 5\. Modelo de datos inicial
*   `administrador`: usuario, hash de contraseña, fecha de último acceso.
*   `habitaciones`: id, número único, tipo, tarifa de día, tarifa momentánea, estado, activa.
*   `clientes`: id, DNI único, nombres, apellidos, teléfono, fecha de registro.
*   `estancias`: cliente, habitación, modalidad, ingreso, salida prevista, salida real, importe y estado.
*   `transacciones`: estancia, cliente, importe, fecha, método de pago, anulación y motivo.
*   `cola_limpieza`: habitación, fecha de encolado y fecha de atención.
*   `auditoria`: acción, entidad, entidad afectada, detalle antes/después y fecha.
*   `configuracion`: datos del hostal, estado del onboarding y preferencias operativas.

Índices mínimos: DNI, estado de habitación y estado de estancia. Activar foreign keys y WAL en SQLite.
## 6\. Plan por fases
## Fase 0 · Fundaciones
Lista: 901716460957 ([https://app.clickup.com/90171378301/v/li/901716460957](https://app.clickup.com/90171378301/v/li/901716460957))

Objetivo: dejar lista la base técnica del proyecto.
- [ ] Private ([https://app.clickup.com/t/86e316yx7](https://app.clickup.com/t/86e316yx7)) | 31 ago - 1 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316yxa](https://app.clickup.com/t/86e316yxa)) | 1 - 2 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316yxt](https://app.clickup.com/t/86e316yxt)) | 2 - 4 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316yxx](https://app.clickup.com/t/86e316yxx)) | 3 - 5 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316yy2](https://app.clickup.com/t/86e316yy2)) | 4 - 5 sep | Normal
- [ ] Private ([https://app.clickup.com/t/86e316yyx](https://app.clickup.com/t/86e316yyx)) | 5 - 6 sep | Alta

Entregable: aplicación vacía ejecutable, base local inicializable, migraciones y pipeline de calidad funcionando.
## Fase 1 · Núcleo de dominio
Lista: 901716460962 ([https://app.clickup.com/90171378301/v/li/901716460962](https://app.clickup.com/90171378301/v/li/901716460962))

Objetivo: implementar las reglas de negocio puras en Rust antes de construir la interfaz.
- [ ] Private ([https://app.clickup.com/t/86e316yzz](https://app.clickup.com/t/86e316yzz)) | 7 - 8 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316z1v](https://app.clickup.com/t/86e316z1v)) | 8 - 10 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316z3c](https://app.clickup.com/t/86e316z3c)) | 9 - 11 sep | Alta
- [ ] Private ([https://app.clickup.com/t/86e316z4n](https://app.clickup.com/t/86e316z4n)) | 10 - 12 sep | Alta
- [ ] Private ([https://app.clickup.com/t/86e316z66](https://app.clickup.com/t/86e316z66)) | 11 - 13 sep | Alta
- [ ] Private ([https://app.clickup.com/t/86e316z7d](https://app.clickup.com/t/86e316z7d)) | 12 - 13 sep | Alta

Entregable: dominio probado con cálculo de tarifas, estados, cola FIFO, pila LIFO y árbol de clientes.
## Fase 2 · Autenticación y administrador
Lista: 901716460965 ([https://app.clickup.com/90171378301/v/li/901716460965](https://app.clickup.com/90171378301/v/li/901716460965))

Objetivo: proteger el acceso con un único rol administrativo y soportar el primer arranque.
- [ ] Private ([https://app.clickup.com/t/86e316z8u](https://app.clickup.com/t/86e316z8u)) | 14 - 16 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316za6](https://app.clickup.com/t/86e316za6)) | 15 - 17 sep | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zbe](https://app.clickup.com/t/86e316zbe)) | 17 - 19 sep | Normal
- [ ] Private ([https://app.clickup.com/t/86e316zcn](https://app.clickup.com/t/86e316zcn)) | 18 - 20 sep | Normal

Entregable: instalación inicial, login, sesión protegida, bloqueo por inactividad y recuperación offline.
## Fase 3 · Habitaciones
Lista: 901716460969 ([https://app.clickup.com/90171378301/v/li/901716460969](https://app.clickup.com/90171378301/v/li/901716460969))

Objetivo: administrar el inventario físico y sus estados operativos.
- [ ] Private ([https://app.clickup.com/t/86e316zdv](https://app.clickup.com/t/86e316zdv)) | 21 - 23 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316zf6](https://app.clickup.com/t/86e316zf6)) | 22 - 24 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316zgb](https://app.clickup.com/t/86e316zgb)) | 24 - 26 sep | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zj5](https://app.clickup.com/t/86e316zj5)) | 25 - 27 sep | Normal

Entregable: inventario configurable, estados consistentes y grilla operativa para recepción.
## Fase 4 · Clientes
Lista: 901716460971 ([https://app.clickup.com/90171378301/v/li/901716460971](https://app.clickup.com/90171378301/v/li/901716460971))

Objetivo: registrar huéspedes y consultar rápidamente sus antecedentes.
- [ ] Private ([https://app.clickup.com/t/86e316zm2](https://app.clickup.com/t/86e316zm2)) | 28 - 30 sep | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316zng](https://app.clickup.com/t/86e316zng)) | 29 sep - 1 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zq2](https://app.clickup.com/t/86e316zq2)) | 1 - 3 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zr8](https://app.clickup.com/t/86e316zr8)) | 2 - 4 oct | Normal

Entregable: alta de clientes, búsqueda por DNI, historial y listado con ordenamientos.
## Fase 5 · Check-in y check-out
Lista: 901716460972 ([https://app.clickup.com/90171378301/v/li/901716460972](https://app.clickup.com/90171378301/v/li/901716460972))

Objetivo: cubrir el ciclo completo de una estancia.
- [ ] Private ([https://app.clickup.com/t/86e316zt1](https://app.clickup.com/t/86e316zt1)) | 5 - 8 oct | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316zt4](https://app.clickup.com/t/86e316zt4)) | 7 - 9 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316ztb](https://app.clickup.com/t/86e316ztb)) | 9 - 13 oct | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316ztk](https://app.clickup.com/t/86e316ztk)) | 13 - 16 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zty](https://app.clickup.com/t/86e316zty)) | 15 - 18 oct | Normal

Entregable: check-in atómico, check-out explícito, excedentes, alertas, extensiones y cambios de habitación.
## Fase 6 · Limpieza y transacciones
Lista: 901716460973 ([https://app.clickup.com/90171378301/v/li/901716460973](https://app.clickup.com/90171378301/v/li/901716460973))

Objetivo: cerrar la operación diaria y asegurar la trazabilidad financiera.
- [ ] Private ([https://app.clickup.com/t/86e316zu2](https://app.clickup.com/t/86e316zu2)) | 19 - 21 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zu8](https://app.clickup.com/t/86e316zu8)) | 20 - 22 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zuc](https://app.clickup.com/t/86e316zuc)) | 21 - 24 oct | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316zuu](https://app.clickup.com/t/86e316zuu)) | 23 - 26 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zv0](https://app.clickup.com/t/86e316zv0)) | 26 - 29 oct | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zvc](https://app.clickup.com/t/86e316zvc)) | 28 - 31 oct | Normal
- [ ] Private ([https://app.clickup.com/t/86e316zvh](https://app.clickup.com/t/86e316zvh)) | 29 oct - 1 nov | Normal

Entregable: limpieza FIFO, pagos, pila de transacciones, anulación controlada, auditoría y caja diaria.
## Fase 7 · Dashboard y UX
Lista: 901716460974 ([https://app.clickup.com/90171378301/v/li/901716460974](https://app.clickup.com/90171378301/v/li/901716460974))

Objetivo: hacer la aplicación rápida y clara para el trabajo de recepción.
- [ ] Private ([https://app.clickup.com/t/86e316zvn](https://app.clickup.com/t/86e316zvn)) | 2 - 5 nov | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zvt](https://app.clickup.com/t/86e316zvt)) | 4 - 6 nov | Normal
- [ ] Private ([https://app.clickup.com/t/86e316zvv](https://app.clickup.com/t/86e316zvv)) | 5 - 8 nov | Normal

Entregable: dashboard operativo, navegación rápida y UX resistente a errores.
## Fase 8 · Backup, empaquetado y release
Lista: 901716460975 ([https://app.clickup.com/90171378301/v/li/901716460975](https://app.clickup.com/90171378301/v/li/901716460975))

Objetivo: proteger los datos y publicar una versión instalable y actualizable.
- [ ] Private ([https://app.clickup.com/t/86e316zvz](https://app.clickup.com/t/86e316zvz)) | 9 - 12 nov | Urgente
- [ ] Private ([https://app.clickup.com/t/86e316zw4](https://app.clickup.com/t/86e316zw4)) | 11 - 14 nov | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zwe](https://app.clickup.com/t/86e316zwe)) | 13 - 17 nov | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zx1](https://app.clickup.com/t/86e316zx1)) | 16 - 19 nov | Alta
- [ ] Private ([https://app.clickup.com/t/86e316zxb](https://app.clickup.com/t/86e316zxb)) | 18 - 21 nov | Normal
- [ ] Private ([https://app.clickup.com/t/86e316zxf](https://app.clickup.com/t/86e316zxf)) | 20 - 22 nov | Normal

Entregable: backups verificables, restauración probada, instaladores firmados, updater automático, release automatizado y manual.
## 7\. Criterios de aceptación globales
*   La aplicación funciona sin internet para la operación diaria.
*   SQLite se crea automáticamente en el primer arranque.
*   Ninguna operación crítica deja datos parcialmente actualizados.
*   Las contraseñas nunca se guardan en texto plano.
*   El administrador puede completar check-in y check-out sin salir del flujo.
*   Una habitación no puede quedar ocupada sin estancia activa ni sucia sin entrada en la cola.
*   Los pagos anulados permanecen visibles y auditables.
*   Se puede restaurar un backup válido y recuperar la operación.
*   Existe al menos un instalador probado en una máquina limpia.
*   Una instalación anterior puede actualizarse mediante GitHub Releases.
*   El pipeline bloquea cambios con tests, lint o formato fallidos.
## 8\. Riesgos y decisiones
### Backup local
El backup automático debe guardarse en una ubicación configurable. Recomendada: unidad externa o carpeta sincronizada por el sistema operativo. La aplicación debe indicar claramente cuándo el último backup falló.
### Actualizaciones
El updater y la firma deben probarse temprano, no solo durante el release final. Si la clave de firma se configura mal, las instalaciones existentes no podrán actualizarse de forma segura.
### Seguridad
No hay multirol en el alcance actual: solo existe el administrador. Aun así, las acciones sensibles requieren contraseña o confirmación reforzada y quedan en auditoría.
### Consistencia
Check-in, check-out, pagos, cambios de habitación, limpieza y restauración deben ejecutarse en transacciones SQLite cuando modifiquen más de una tabla.
## 9\. Orden recomendado de ejecución
1. Fundaciones y migraciones.
2. Dominio y tests.
3. Login y onboarding.
4. Habitaciones.
5. Clientes.
6. Check-in y check-out.
7. Limpieza y transacciones.
8. Dashboard y UX.
9. Backup, empaquetado, updater y release.

La configuración de firma y updater debe validarse antes de cerrar las funcionalidades, aunque su entrega formal permanezca en Fase 8.