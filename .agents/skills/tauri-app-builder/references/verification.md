# Verification matrix

## Contents

- Static gates
- Unit and integration tests
- Browser checks
- Native interaction matrix
- Linux desktop matrix
- Release checks
- Evidence rules

## Static gates

Run the bundled audit:

```bash
python3 ~/.codex/skills/tauri-app-builder/scripts/audit_tauri_project.py .
```

Then run project-native commands. A strong Rust gate:

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
```

Also run frontend lint, tests, typecheck, and production build. Treat warnings as failures unless a
documented external limitation makes that impossible.

Check source file sizes. Split application source files above roughly 500 lines into focused modules.

## Unit and integration tests

Unit-test pure boundaries:

- input/path/identifier validation;
- shell or protocol quoting;
- command argument ordering;
- icon byte conversion;
- decoded dimension arithmetic;
- maximum-size rejection;
- atomic config migration;
- bounded history/pruning;
- drag-event classification;
- theme preference resolution;
- user-facing error mapping.

Use real small files and encoded images in tests rather than fake success objects. Mark tests that
require an unlocked credential vault or graphical clipboard as explicit integration tests; never
silently skip them.

Test the IPC contract from both sides where feasible. Rust success alone does not detect a
TypeScript argument casing mismatch.

## Browser checks

Browser verification is useful for:

- DOM semantics;
- focus-visible states;
- layout and overflow;
- theme rendering;
- component workflows not requiring native APIs;
- console and network errors.

Test at least 320, 375, 420/414, 768, and a normal desktop width when the window can resize. For
compact popups, verify the configured minimum width and height.

Browser checks cannot prove:

- tray behavior;
- transparent compositor corners;
- window positioning;
- native focus;
- close-to-tray;
- file-system path drops;
- native clipboard;
- native dialogs;
- capabilities;
- package behavior.

## Native interaction matrix

### Startup and tray

- first launch creates exactly one tray icon;
- left click performs the primary action;
- right click opens the menu without triggering primary action;
- every menu item works;
- a second launch focuses the first instance;
- restarting does not leave a stale icon or service.

### Window

- starts hidden when tray-first;
- appears on the intended monitor;
- remains within every monitor edge;
- focuses after show;
- custom titlebar drags from intended regions;
- header buttons do not drag;
- rounded corners have no square bleed;
- taskbar/dock presence matches product intent;
- always-on-top does not trap unrelated work.

### Lifecycle

- Escape hides;
- close hides when configured;
- blur hides only when appropriate;
- explicit Quit exits;
- shutdown removes caches/sockets/services;
- OS logout/shutdown does not hang.

### Native interactions

- dialog select and cancel;
- clipboard image, text-only clipboard, empty clipboard, and large image;
- file drag: enter, multiple over events, drop, leave, drop outside;
- drag from every supported file manager;
- unsupported file and oversized file;
- operation replacement/cancellation does not show stale results.

### Theme and accessibility

- system/dark/light rendering;
- system theme changes while open;
- icons remain visible on light and dark panels;
- keyboard navigation and focus rings;
- reduced motion;
- screen reader labels for icon-only controls;
- no two-line critical button labels at minimum width.

## Linux desktop matrix

Test at least the environments the project claims to support:

- KDE Plasma;
- GNOME with the expected AppIndicator/StatusNotifier extension;
- Cinnamon/Xfce when they are target users;
- X11 and Wayland where available;
- fractional scaling;
- multiple monitors with negative/global offsets.

Record desktop, display protocol, scale factor, tray host/extension, and package type in bug reports.
Tray and focus behavior can differ by host even when the application code is unchanged.

## Release checks

For each bundle:

1. confirm it is freshly rebuilt from the tested source state;
2. record SHA-256;
3. inspect file type and architecture;
4. install or run it in a clean environment;
5. verify icons and desktop entry;
6. verify required system dependencies;
7. verify upgrade preserves configuration;
8. verify uninstall does not delete user data unexpectedly;
9. verify the minimum supported OS/glibc baseline;
10. scan logs for warnings.

## Evidence rules

Report separately:

- automated checks run and their counts;
- browser viewports and workflows verified;
- native actions physically performed;
- platform/desktop/package tested;
- behaviors not verified.

Never turn a code review or a browser screenshot into a claim that native tray, clipboard, focus, or
drag/drop behavior works. Say exactly what remains for a maintainer to try.
