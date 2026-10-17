# Upstream research sources

Research snapshot: 2026-07-26. Re-check these sources before changing APIs, permissions, packaging,
or platform claims.

## Tauri 2

- System tray:
  https://v2.tauri.app/learn/system-tray/
  - Native menus and `show_menu_on_left_click(false)`.
  - Official Linux note: tray mouse events are unsupported even though icons and context menus work.
- Window customization:
  https://v2.tauri.app/learn/window-customization/
  - Custom titlebars, direct-only `data-tauri-drag-region`, manual `startDragging`.
- Window JavaScript API:
  https://v2.tauri.app/reference/javascript/api/namespacewindow/
  - Window options and platform caveats for transparency, shadows, effects, taskbar, focus, sizes.
- Webview/window drag events:
  https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/
  - `onDragDropEvent` and listener cleanup.
- Configuration:
  https://v2.tauri.app/reference/config/
  - Window config, CSP, bundle targets, and platform-specific configuration merging.
- Capabilities:
  https://v2.tauri.app/reference/acl/capability/
  - Exact window/webview boundaries, permission merging, remote-source warnings.
- Core permissions:
  https://v2.tauri.app/reference/acl/core-permissions/
  - Current permission identifiers.
- Security:
  https://v2.tauri.app/security/
  - Webview/IPC trust model and capability boundary.
- Runtime authority:
  https://v2.tauri.app/security/runtime-authority/
  - How origin, capability, permissions, and scopes are enforced.
- Single instance:
  https://v2.tauri.app/plugin/single-instance/
  - Must register first; Linux D-Bus behavior; Snap/Flatpak declarations.
- Positioner:
  https://v2.tauri.app/plugin/positioner/
  - Tray event forwarding and constrained/common window positions.
- Plugin catalog:
  https://v2.tauri.app/plugin/
- App icons:
  https://v2.tauri.app/develop/icons/
  - `tauri icon` and required platform icon outputs.
- Distribution:
  https://v2.tauri.app/distribute/
- AppImage:
  https://v2.tauri.app/distribute/appimage/
  - old-base-system compatibility, typical size, ARM limitations, GStreamer, PATH warning.
- Linux signing:
  https://v2.tauri.app/distribute/sign/linux/

## Linux StatusNotifierItem

- `ksni::Tray`:
  https://docs.rs/ksni/latest/ksni/trait.Tray.html
  - `activate`, `secondary_activate`, menus, tooltips, watcher lifecycle, ARGB32 icon pixmaps.
- `ksni::TrayServiceBuilder`:
  https://docs.rs/ksni/latest/ksni/struct.TrayServiceBuilder.html
  - sandbox D-Bus-name behavior and watcher availability.
- Freedesktop StatusNotifierItem specification:
  https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/

## Research discipline

Prefer official Tauri guides/reference, current docs.rs for the installed Rust crate, and the
freedesktop specification. Do not base production behavior on a single old issue comment or blog
post. When implementation and current documentation disagree, inspect the installed type
declarations/source and reproduce behavior on the target desktop.
