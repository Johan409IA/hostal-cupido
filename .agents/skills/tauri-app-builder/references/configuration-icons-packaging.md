# Configuration, icons, and packaging

## Contents

- Version alignment
- Tauri configuration
- Plugins
- Capabilities
- Icon pipeline
- Linux prerequisites
- Bundles
- Release compatibility

## Version alignment

Tauri core, CLI, JavaScript API, and plugins release separately. Before editing:

1. inspect `package.json`, `Cargo.toml`, and lockfiles;
2. fetch current upstream docs for the installed major version;
3. avoid assuming every package shares the same patch version;
4. run the CLI's version/mismatch checks during build;
5. update both JavaScript and Rust halves of a plugin together.

## Tauri configuration

Keep:

- stable reverse-domain `identifier`;
- product name and semantic version;
- explicit `beforeDevCommand`, `devUrl`, `beforeBuildCommand`, and `frontendDist`;
- one clearly named window label per security boundary;
- a CSP under `app.security`;
- explicit bundle targets and metadata.

Use platform-specific configuration files when behavior diverges:

- `tauri.linux.conf.json`
- `tauri.windows.conf.json`
- `tauri.macos.conf.json`

Do not fill the common configuration with brittle cross-platform conditionals.

For a local Vite frontend:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  }
}
```

Use a strict fixed development port so Tauri never opens a different or stale server.

## Plugins

Register only what the product uses. Common tray utility plugins:

- single-instance;
- positioner;
- dialog;
- clipboard manager;
- autostart when explicitly requested;
- updater when release infrastructure exists.

Register single-instance first. It must run before other plugins can interfere. Its callback should
show and focus the existing window, and may process second-launch arguments.

On Linux, the single-instance plugin uses D-Bus. Snap and Flatpak need explicit D-Bus ownership/talk
rules for the generated service name.

The positioner plugin can place windows at common or tray-relative positions. Tray-relative use
requires forwarding Tauri tray events to the plugin. Use a constrained position method where
available.

## Capabilities

Capability files live under `src-tauri/capabilities`. All files there may be enabled automatically
unless the app explicitly lists capability identifiers. Keep one readable file per privilege set.

Audit every frontend import from `@tauri-apps/api/*` and `@tauri-apps/plugin-*` against capability
permissions. A plugin registered in Rust is not necessarily callable from the webview until its
permission is granted.

## Icon pipeline

Start from a square high-resolution source, preferably at least 1024×1024:

```bash
npm run tauri icon path/to/app-icon.png
```

Inspect:

- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.png`
- `icon.ico`
- `icon.icns`

Keep all generated platform formats even for a platform-first project when cross-platform support is
plausible.

Tray icons need separate inspection at 16, 22, 24, and 32 px:

- simplify line work;
- preserve a transparent background;
- use consistent optical padding;
- avoid thin semitransparent strokes;
- prepare light and dark variants where the host does not apply template coloring.

Do not assume the package icon is a good tray icon or in-app logo. They have different visual
constraints.

## Linux prerequisites

Current Tauri 2 builds require WebKitGTK 4.1 development packages and normal native build tooling.
Use the official prerequisites page for the target distribution rather than copying a stale apt
line.

GUI applications on Linux and macOS do not necessarily inherit shell startup `$PATH`. If the product
launches user-installed command-line tools, resolve this deliberately or use the maintained
`fix-path-env` approach. Never assume `.bashrc` has run.

## Bundles

Tauri supports Linux DEB, RPM, AppImage, Snap, Flatpak, and AUR workflows. A practical direct-release
set is:

```json
{
  "bundle": {
    "active": true,
    "targets": ["deb", "appimage", "rpm"],
    "category": "Utility",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

AppImages bundle more dependencies and commonly exceed 70 MB. Their `linuxdeploy` phase can be much
slower than Rust compilation, especially on mounted or slow filesystems.

Do not cross-compile ARM AppImages; build them on ARM hardware/runners. Enable bundled media
framework support only when the app actually needs GStreamer and licensing has been reviewed.

## Release compatibility

Build Linux packages on the oldest supported base distribution that still offers Tauri 2's required
WebKitGTK 4.1 packages. glibc compatibility flows from the build environment; a package built on a
new system can fail on older distributions.

Official guidance identifies Ubuntu 22.04 and Debian 12 as suitable baseline examples at the time
this reference was researched. Re-check before establishing a release pipeline.

After building:

- verify file type, size, timestamp, and checksum;
- install the DEB/RPM in a disposable environment;
- run the AppImage directly;
- confirm desktop entry, icon, tray, and uninstall behavior;
- sign packages when the release policy requires it;
- test startup on the minimum supported distribution.

AppImage signing can use GPG environment configuration, but CI secrets must be handled by the CI
vault and signing failures should fail the build when authenticity is required.
