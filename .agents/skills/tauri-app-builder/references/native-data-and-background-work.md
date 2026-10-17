# Native data and background work

## Contents

- Managed state
- Platform directories
- Atomic configuration
- Staging stores
- Binary processing
- Preview persistence
- Blocking work
- Cleanup

## Managed state

Initialize stores during `setup` and register them with `app.manage`:

```rust
.setup(|app| {
    app.manage(ConfigStore::load(app.handle())?);
    app.manage(CacheStore::load(app.handle())?);
    Ok(())
})
```

Commands request `tauri::State<'_, Store>`. Prefer cloning immutable records out of a lock before
doing I/O. Never hold a mutex across network, process, image decode, or keyring operations.

Use a lock type that matches the access pattern. A standard `Mutex` is often sufficient for small
configuration snapshots; do not add async locks without an async critical section.

## Platform directories

Use:

- `app_config_dir()` for preferences and small metadata;
- `app_data_dir()` for durable app-owned files;
- `app_cache_dir()` for reproducible or disposable state.

Create directories explicitly and surface errors. Do not assume parent directories exist.

## Atomic configuration

Serialize to bytes, write a sibling temporary file, then rename:

```rust
let bytes = serde_json::to_vec_pretty(config)?;
let temporary = path.with_extension("json.tmp");
std::fs::write(&temporary, bytes)?;
std::fs::rename(&temporary, path)?;
```

For stronger durability guarantees, consider file and directory `sync_all` appropriate to the
platform. Add `#[serde(default)]` to new fields so older config versions remain readable. Introduce
an explicit schema version and migration path once changes cannot be represented by defaults.

Bound history and other append-only collections before persisting.

## Staging stores

Do not push large files through repeated frontend state or IPC after validation. A staging store can:

1. validate/decode bytes in Rust;
2. write app-owned bytes under an opaque UUID in cache;
3. retain only metadata and the managed path in a locked map;
4. return the UUID plus bounded preview metadata;
5. perform the final operation by UUID;
6. delete on success, replacement, discard, startup recovery, and exit.

Never trust a path sent back from the frontend as a staging record. Look up the opaque ID in managed
state and confirm its file still exists.

Clean stale staging files at startup. A prior process may have crashed before normal exit.

## Binary processing

Validate encoded data before persistence:

- reject input over a product-defined byte limit before decode;
- sniff the format from bytes instead of trusting extensions;
- decode to establish authoritative dimensions;
- reject zero dimensions;
- use checked arithmetic for raw pixel buffers;
- preserve formats only when downstream consumers reliably support them;
- transcode unsupported-but-decodable formats to a canonical format.

For RGBA:

```rust
let expected = (width as usize)
    .checked_mul(height as usize)
    .and_then(|pixels| pixels.checked_mul(4))
    .ok_or(Error::InvalidDimensions)?;
if width == 0 || height == 0 || rgba.len() != expected {
    return Err(Error::InvalidDimensions);
}
```

Protect against decompression bombs. Encoded-byte limits alone do not bound decoded memory; also
bound dimensions or total pixels for untrusted content.

## Preview persistence

Generate a bounded thumbnail, not a copy of the original:

- maximum width and height;
- canonical PNG or WebP encoding;
- a maximum stored/read byte size;
- opaque validated filename;
- separate app-data directory;
- atomic write;
- prune with the owning history records.

Send a `data:` URL only when the CSP permits it and the payload is small. For larger local assets,
use Tauri's asset protocol with a narrow scope.

Treat previews as sensitive derivatives. Document retention and remove them when the user deletes
their record or clears history.

## Blocking work

Tauri commands may be async while using blocking libraries. Wrap the blocking portion:

```rust
#[tauri::command]
async fn import_file(path: String) -> Result<Metadata, String> {
    tauri::async_runtime::spawn_blocking(move || process_file(path))
        .await
        .map_err(|error| format!("background task failed: {error}"))?
        .map_err(|error| error.to_string())
}
```

Candidates:

- filesystem reads/writes of meaningful size;
- image decode/encode;
- clipboard image access on Linux;
- credential vault calls;
- `Command::output`;
- archive operations;
- CPU-heavy parsing.

Do not return a Rust reference or hold Tauri `State` inside the blocking closure. Clone the needed
owned data first.

Provide cancellation or operation IDs in the frontend so stale operations cannot overwrite newer
state.

## Cleanup

Define cleanup for every durable or temporary artifact:

| Artifact | Replacement | Success | Failure | Startup | Exit |
|---|---:|---:|---:|---:|---:|
| staged input | delete | delete | retain or delete by policy | delete stale | delete |
| one-time socket | n/a | delete | delete | delete stale | delete |
| history preview | n/a | retain | delete partial | prune orphan | retain |
| temporary config | n/a | renamed | delete/recover | recover/delete | n/a |

Use RAII `Drop` for private sockets, helper threads, and temporary directories. Keep final application
cleanup in Rust because the webview may already be gone.
