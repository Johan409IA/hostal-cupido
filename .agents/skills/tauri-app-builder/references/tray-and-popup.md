# Tray and popup engineering

## Contents

- Decision tree
- Tauri tray baseline
- Linux StatusNotifierItem path
- Popup positioning
- Dynamic menus and events
- Service lifetime
- Failure modes

## Decision tree

1. Does the app need only a tray context menu?
   Use Tauri `TrayIconBuilder`.
2. Does it need dependable left-click activation on Windows/macOS?
   Use Tauri tray events and `show_menu_on_left_click(false)`.
3. Does it need dependable left-click activation separate from a right-click context menu on Linux?
   Use a Linux StatusNotifierItem implementation such as `ksni`.
4. Is the package sandboxed by Flatpak or Snap?
   Model D-Bus ownership/talk permissions and test inside the sandbox. Do not assume host behavior.

Official Tauri documentation currently states that tray mouse events are unsupported on Linux. The
icon and context menu still work. This is a platform support gap, not an application bug.

## Tauri tray baseline

Enable the `tray-icon` feature when required by the installed Tauri version, construct native menu
items in Rust, and disable menu-on-left-click when activation should open a window.

```rust
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn install_tray(app: &mut tauri::App) -> tauri::Result<()> {
    let open = MenuItemBuilder::with_id("open", "Open").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&open, &quit]).build()?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().expect("bundle icon").clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => show_popup(app, None),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_popup(tray.app_handle(), None);
            }
        })
        .build(app)?;
    Ok(())
}
```

Do not depend on the `on_tray_icon_event` branch for Linux unless current upstream docs and actual
desktop testing confirm it.

## Linux StatusNotifierItem path

`ksni` maps the freedesktop StatusNotifierItem D-Bus interface into a Rust `Tray` trait:

- `activate(x, y)` normally represents primary activation;
- `secondary_activate(x, y)` represents secondary activation;
- `menu()` provides the D-Bus menu;
- `icon_pixmap()` supplies ARGB32 pixels;
- `tool_tip()` supplies optional title, description, and icon;
- the activation coordinates are global screen-coordinate hints.

Keep the service handle alive in Tauri managed state and shut it down on final application exit.

```rust
use std::sync::Mutex;
use ksni::blocking::{Handle, TrayMethods};

pub struct TrayService {
    handle: Mutex<Option<Handle<AppTray>>>,
}

impl TrayService {
    pub fn shutdown(&self) {
        if let Some(handle) = self.handle.lock().expect("tray lock").take() {
            handle.shutdown().wait();
        }
    }
}
```

Use `MENU_ON_ACTIVATE = false` when primary activation must run `activate` rather than open the menu.
Keep menu callbacks fast. Long work freezes the native menu; hand it to app state, a channel, or an
async/background task.

For Flatpak, `ksni` may need `disable_dbus_name(true)` because owning a well-known D-Bus name can be
restricted. For desktops that start the app before the notifier watcher, `assume_sni_available(true)`
can turn watcher absence into a recoverable callback. Both options change failure behavior and must
be tested deliberately.

## Icon pixel format

StatusNotifierItem icon pixmaps are ARGB32. Libraries may document the byte layout differently from
the decoded image's RGBA order. A common little-endian conversion for an RGBA byte vector is:

```rust
for pixel in rgba.chunks_exact_mut(4) {
    pixel.rotate_right(1); // RGBA -> ARGB byte sequence
}
```

Do not cargo-cult this. Check the selected crate's current `Icon` contract and inspect the tray on
the target desktop. Incorrect ordering produces blue/red swaps or broken transparency.

Supply at least a crisp 32 or 64 px pixmap. A family of sizes may improve host selection.

## Popup positioning

When activation coordinates are available:

1. Call `monitor_from_point`.
2. Read the window outer size.
3. Clamp X between monitor edges plus a small margin.
4. Place below the activation point when it is in the monitor's upper half.
5. Place above it when it is in the lower half.
6. Clamp Y as well when panels, fractional scaling, or unusual monitor layouts are possible.
7. Show, then focus.

Use physical positions consistently. Do not mix logical window sizes with physical global
coordinates without applying the scale factor.

```rust
use tauri::{Manager, PhysicalPosition};

fn show_popup(app: &tauri::AppHandle, point: Option<(i32, i32)>) {
    let Some(window) = app.get_webview_window("main") else { return };

    if let Some((x, y)) = point {
        if let (Ok(Some(monitor)), Ok(size)) = (
            window.monitor_from_point(x as f64, y as f64),
            window.outer_size(),
        ) {
            let origin = monitor.position();
            let area = monitor.size();
            let margin = 8;
            let max_x = origin.x + area.width as i32 - size.width as i32 - margin;
            let px = (x - size.width as i32 / 2).clamp(origin.x + margin, max_x);
            let py = if y < origin.y + area.height as i32 / 2 {
                y + margin
            } else {
                y - size.height as i32 - margin
            };
            let _ = window.set_position(PhysicalPosition::new(px, py));
        }
    }
    let _ = window.show();
    let _ = window.set_focus();
}
```

When coordinates are unavailable, the positioner plugin can place the window at a stable monitor
corner. For Tauri tray events, pass events through `tauri_plugin_positioner::on_tray_event` before
using tray-relative positions. Prefer `move_window_constrained` for tray-relative positions.

## Dynamic menus and native events

Generate menu state from a cheap in-memory snapshot. Use stable item IDs. If a menu action changes
frontend-visible state, emit a narrow application event after the native state succeeds.

Show the window before emitting a navigation event so the webview exists and is ready to receive it.
Frontend listeners must validate event payloads rather than trusting arbitrary strings.

## Service lifetime

- Create the tray once in `setup`.
- Store its handle in managed state.
- Do not let the handle drop after setup.
- Hide the window instead of destroying it for normal tray use.
- Explicit Quit calls `app.exit(0)`.
- Shut down the tray service during final `RunEvent::Exit`.
- Ensure second instances never create another tray owner.

## Failure modes

- **Left click opens the context menu:** menu-on-left-click was not disabled, or Linux tray mouse
  events are unavailable.
- **Duplicate icons:** single-instance registration is late or two tray implementations are active.
- **Popup appears on the wrong monitor:** activation coordinates were ignored or logical/physical
  coordinates were mixed.
- **Popup falls offscreen:** coordinates were used without monitor clamping.
- **Tray disappears after startup:** the tray handle was dropped.
- **Colors are wrong:** RGBA bytes were passed where ARGB32 was expected.
- **Menu freezes:** callback performed blocking work.
- **No tray on GNOME:** the desktop may require an AppIndicator/StatusNotifier extension.
