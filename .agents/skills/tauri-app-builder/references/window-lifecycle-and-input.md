# Window lifecycle and native input

## Contents

- Popup configuration
- Rounded transparent shells
- Custom dragging
- Hide, close, and exit
- Blur suppression
- File drag/drop
- Native dialogs
- Dropdowns and detached surfaces
- Clipboard
- Themes

## Popup configuration

A tray popup is not a normal document window. Configure its product contract explicitly:

```json
{
  "label": "main",
  "width": 420,
  "height": 700,
  "minWidth": 320,
  "minHeight": 520,
  "resizable": true,
  "maximizable": false,
  "minimizable": false,
  "decorations": false,
  "transparent": true,
  "visible": false,
  "alwaysOnTop": true,
  "skipTaskbar": true,
  "focus": true,
  "dragDropEnabled": true
}
```

Change dimensions and resizability for the actual product. Do not automatically make every Tauri
window always-on-top, hidden, or transparent.

Transparent windows have platform caveats:

- Tauri's `shadow` setting is unsupported on Linux.
- macOS transparency can require private APIs that prevent App Store acceptance.
- Windows transparency may need `noRedirectionBitmap` to reduce white flashes.
- native effects and radius support are platform-specific.

## Rounded transparent shells

Round and clip every layer that can paint:

```css
html,
body,
#root {
  width: 100%;
  height: 100%;
  margin: 0;
  overflow: clip;
  background: transparent;
  border-radius: var(--window-radius);
}

.app-shell {
  width: 100%;
  height: 100%;
  overflow: clip;
  background: var(--surface);
  border: 1px solid var(--rule);
  border-radius: var(--window-radius);
}
```

Use `overflow: clip`, not only `hidden`, to prevent corner bleed without creating a scroll
container. Remove CSS box shadows that extend into the transparent region when they show as square
pixels at the corners. Verify the actual compiled window because browser screenshots do not model
compositor transparency.

## Custom dragging

`data-tauri-drag-region` applies only to the element carrying it; it is not inherited. Mark each
noninteractive header region that should drag. Leave buttons, selects, inputs, and menus unmarked.

```tsx
<header data-tauri-drag-region>
  <div data-tauri-drag-region>Product</div>
  <button type="button">Settings</button>
</header>
```

Grant `core:window:allow-start-dragging` only if JavaScript calls `startDragging()`. Static
`data-tauri-drag-region` does not justify unrelated window permissions.

## Hide, close, and exit

Intercept close requests in Rust so a frontend crash cannot accidentally destroy a tray-resident
window:

```rust
.on_window_event(|window, event| {
    if window.label() == "main" {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = window.hide();
        }
    }
})
```

At application run time:

- prevent an unqualified exit request and hide the popup;
- allow `app.exit(code)` from the explicit Quit command;
- clean temporary data on `ExitRequested` and `Exit`;
- shut down native services only on final `Exit`.

Do not register contradictory close logic in Rust and the frontend without a reason. If the frontend
also listens to close requests, always call the returned unlisten function during teardown.

## Blur suppression

Native file drags and dialogs can make the webview report a focus loss. A naive `onFocusChanged`
handler hides the window in the middle of the interaction.

Maintain one shared mutable suppression flag:

```ts
const suppressAutoHide = useRef(false);
```

All focus-stealing operations set it before they begin and release it after a short grace period.
The blur handler:

1. cancels the prior timer;
2. returns immediately when focus is restored;
3. waits roughly 150–250 ms;
4. checks the shared suppression flag;
5. optionally checks whether the global pointer is still inside `outerPosition()`/`outerSize()`;
6. hides only when both checks say the interaction is outside.

Add capabilities for `cursorPosition`, `outerPosition`, `outerSize`, `setFocus`, and `hide` only when
the frontend actually calls them.

After opening the popup from native code, emit a narrow `popup-opened` event and suppress blur for
the first few hundred milliseconds. Some window managers produce a transient focus change during
show/reposition/focus.

## File drag/drop

Use `getCurrentWebview().onDragDropEvent`. The event payload types are:

- `enter`
- `over`
- `drop`
- `leave`

Treat both `enter` and `over` as active:

```ts
export function isActiveDragEvent(type: DragDropEvent["type"]) {
  return type === "enter" || type === "over";
}
```

Recommended state machine:

```text
idle
  └─ enter/over → suppress hide, focus window, show drag UI
dragging
  ├─ over       → refresh session, keep suppression
  ├─ drop       → hide drag UI, import first accepted path, refocus, delayed release
  └─ leave      → hide drag UI, delayed release
```

Do not classify every non-`over` event as completion; the first `enter` would end the session and can
hide the popup. Do not release suppression before an asynchronous path import finishes.

```ts
const unlisten = await getCurrentWebview().onDragDropEvent((event) => {
  if (event.payload.type === "enter" || event.payload.type === "over") {
    beginDragSession();
    setDragging(true);
  } else if (event.payload.type === "drop") {
    beginDragSession();
    setDragging(false);
    const [path] = event.payload.paths;
    void importPath(path).finally(() => {
      void getCurrentWindow().setFocus();
      endDragSessionAfterGracePeriod();
    });
  } else {
    setDragging(false);
    endDragSessionAfterGracePeriod();
  }
});
```

Clear release timers and call `unlisten()` when the component/hook unmounts. Add a pure unit test
asserting `enter` and `over` are active while `drop` and `leave` are not.

Native `dragDropEnabled` can interfere with browser HTML5 drop handlers. Choose one event system for
external files and test the target operating system.

## Native dialogs

Set suppression before calling a native dialog and release it in `finally`:

```ts
suppressAutoHide.current = true;
try {
  const selected = await open({ multiple: false, directory: false });
  if (selected) await importPath(selected);
} finally {
  window.setTimeout(() => {
    suppressAutoHide.current = false;
  }, 250);
}
```

Handle cancellation as a normal result. Never interpret an empty selection as an error.

## Dropdowns and detached surfaces

Native `<select>` menus, operating-system pickers, and some portal-backed popovers can render in a
separate surface. On a transparent auto-hide popup, opening that surface may make the main webview
report focus loss and disappear while the detached menu remains.

Test every dropdown in the compiled app. If the desktop treats it as a detached surface:

- suppress auto-hide from pointer/key activation until selection or dismissal;
- or replace it with an accessible in-webview segmented control/listbox whose popup remains inside
  the window;
- keep Escape semantics unambiguous between closing the menu and hiding the application;
- verify keyboard navigation, outside click, and screen-reader roles.

Do not paper over the problem by disabling auto-hide globally. The suppression must belong to the
interaction that temporarily owns focus.

## Clipboard

Support both:

- a webview `paste` event for image `ClipboardItem`/`File` payloads;
- the native clipboard plugin's image API for an explicit “Paste” button.

Linux clipboard backends can block. Read native images in a blocking worker task, validate RGBA
dimensions with checked multiplication, encode to a standard format, and return a bounded preview.

Clipboard reads should be initiated by explicit user action. Report “clipboard contains no readable
image” distinctly from decode and permission errors.

## Themes

Represent preference as `system | dark | light`. Store the preference, not only the resolved value.
Set:

- `data-theme` to the preference;
- `data-resolved-theme` to `dark` or `light`;
- `color-scheme` to the resolved theme.

Listen to `prefers-color-scheme` changes only when the stored preference is `system`. Use separate
high-contrast logo assets when a single transparent mark cannot work on both surfaces.
