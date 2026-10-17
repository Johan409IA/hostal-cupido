# Security and IPC

## Contents

- Threat model
- Capabilities
- CSP
- Commands
- Secrets
- Child processes
- Paths and deletion
- Error handling

## Threat model

Model four boundaries:

1. untrusted or compromiseable webview content;
2. Tauri runtime authority and IPC;
3. privileged Rust application code;
4. operating-system services, files, keyrings, processes, and network.

Capabilities constrain frontend access to registered commands and plugins. They do not make unsafe
Rust code safe and do not replace validation inside commands.

## Capabilities

Use exact window or webview labels:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "main-window",
  "description": "Permissions required by the primary desktop window.",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-hide",
    "dialog:allow-open"
  ]
}
```

Rules:

- Start with no plugin permission, add one for each actual frontend API call.
- Prefer exact command permissions over a plugin `default` set when practical.
- Avoid `windows: ["*"]`.
- Separate capabilities for windows with different privilege.
- Remember that permissions merge when a window matches multiple capabilities.
- Avoid remote capability URLs. On Linux, embedded iframes cannot be distinguished from their
  containing window for this boundary.
- Do not expose window creation to lower-privileged windows.
- Review generated `gen/schemas` when permission identifiers are uncertain.

## CSP

Begin with:

```text
default-src 'self'; connect-src ipc: http://ipc.localhost
```

Then add only used sources. Typical local media additions:

```text
img-src 'self' asset: data: blob:;
style-src 'self';
font-src 'self';
```

Do not add `unsafe-eval`, wildcard network origins, or remote scripts to silence errors. When remote
content is truly required, isolate it into a lower-privileged webview and define explicit
navigation/origin policy.

## Commands

Expose product operations, not primitives:

Good:

- `load_preferences`
- `select_and_import_document`
- `render_preview`
- `save_account`

Dangerous:

- `run_shell(command: String)`
- `read_any_file(path: String)`
- `write_any_file(path: String, bytes: Vec<u8>)`

Validate in Rust:

- maximum lengths and byte sizes;
- enum values;
- identifier character sets;
- path roots and canonical ownership;
- record existence and relationship;
- authorization for destructive operations.

Use serializable domain error messages. Limit child-process stderr returned to the frontend so a
hostile or broken process cannot flood IPC or logs.

Keep TypeScript wrappers centralized:

```ts
export function savePreferences(input: Preferences): Promise<Preferences> {
  return invoke<Preferences>("save_preferences", { input });
}
```

Test argument casing. Tauri command parameters and `serde` field casing can fail independently.

## Secrets

Use an OS credential vault:

- Linux Secret Service via a maintained keyring library;
- macOS Keychain;
- Windows Credential Manager.

Store only a stable account/service key and a boolean indicating whether a value exists. Do not
store the value in app JSON.

Use `zeroize::Zeroizing` around secret strings/bytes where practical. Reject empty, oversized,
multiline, and NUL-containing secrets when the downstream protocol expects one line.

Be precise in product copy: recoverable secrets are encrypted and access-controlled by the OS
vault, but a process controlling the same unlocked desktop account may be able to request them.

## Child processes and secret delivery

Prefer a typed Rust `std::process::Command`:

```rust
let mut command = std::process::Command::new("tool");
command.arg("--fixed-option").arg(validated_user_value);
```

Never concatenate user input into a shell command. If a remote shell protocol necessarily accepts a
command string:

- restrict input to a conservative grammar;
- quote it with one audited function;
- unit-test quotes, leading hyphens, parent traversal, wildcards, spaces, and metacharacters;
- keep program options before destination/positional arguments.

Do not pass secrets in arguments. Environment variables are inherited and inspectable, so do not put
the secret value there either. If a tool requires an AskPass helper, one robust pattern is:

1. create a `0700` temporary directory;
2. bind a private Unix socket;
3. generate a random one-time token;
4. point the tool's helper variable to the current executable;
5. put only socket path and token in the child environment;
6. let the helper authenticate to the socket and receive the secret once;
7. bound the session lifetime;
8. zeroize buffers and remove socket/directory on drop.

Never disable host verification or certificate checks as an authentication workaround.

## Paths and deletion

For app-owned local files:

- generate opaque IDs;
- validate IDs before joining to a managed directory;
- reject separators and traversal;
- confirm the target is a file;
- bound reads.

For user-selected files, use native dialog scopes and still validate metadata/size in Rust.

For destructive remote or external operations, re-establish the trusted root immediately before
deletion and require a managed filename prefix or database ownership record. A history record alone
is not proof that an arbitrary path is safe to delete.

## Error handling

Use a domain error enum with stable user-facing categories. Preserve enough diagnostic detail to
debug without leaking secrets or unlimited stderr.

Do not use `unwrap()` on user-controlled paths, external services, or IPC payloads. `expect()` is
acceptable for invariant failures such as a poisoned internal lock when the application cannot
recover safely, but document that choice.

Keep explicit errors for:

- missing or invalid input;
- unsupported format;
- size limits;
- permission/vault errors;
- background task join errors;
- missing managed records;
- unsafe path refusal;
- external command failure.
