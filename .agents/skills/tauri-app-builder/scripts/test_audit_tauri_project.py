#!/usr/bin/env python3
"""Regression tests for the read-only Tauri project auditor."""

from __future__ import annotations

import contextlib
import importlib.util
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("audit_tauri_project.py")
sys.dont_write_bytecode = True
SPEC = importlib.util.spec_from_file_location("audit_tauri_project", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("could not load audit_tauri_project.py")
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)
Audit = MODULE.Audit


class AuditTauriProjectTests(unittest.TestCase):
    def test_accepts_a_hardened_tray_popup_fixture(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.write_project(root)
            audit = Audit(root)

            with contextlib.redirect_stdout(io.StringIO()):
                exit_code = audit.run()
            messages = [finding.message for finding in audit.findings]

            self.assertEqual(exit_code, 0)
            self.assertFalse(
                any("first registered plugin" in message for message in messages)
            )
            self.assertFalse(any(finding.level == "FAIL" for finding in audit.findings))

    def test_rejects_a_directory_without_tauri(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            audit = Audit(Path(temporary))

            with contextlib.redirect_stdout(io.StringIO()):
                exit_code = audit.run()

            self.assertEqual(exit_code, 1)
            self.assertTrue(any(finding.level == "FAIL" for finding in audit.findings))

    @staticmethod
    def write_project(root: Path) -> None:
        (root / "src-tauri" / "src").mkdir(parents=True)
        (root / "src-tauri" / "capabilities").mkdir()
        (root / "src").mkdir()
        (root / "src-tauri" / "Cargo.toml").write_text(
            """
[package]
name = "fixture"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = "2"
tauri-plugin-single-instance = "2"

[profile.release]
lto = true
""".strip(),
            encoding="utf-8",
        )
        config = {
            "productName": "Fixture",
            "version": "0.1.0",
            "identifier": "dev.example.fixture",
            "build": {
                "beforeDevCommand": "npm run dev",
                "devUrl": "http://localhost:1420",
                "frontendDist": "../dist",
            },
            "app": {
                "windows": [
                    {
                        "label": "main",
                        "decorations": False,
                        "transparent": True,
                        "visible": False,
                        "skipTaskbar": True,
                        "dragDropEnabled": True,
                    }
                ],
                "security": {
                    "csp": "default-src 'self'; connect-src ipc: http://ipc.localhost"
                },
            },
        }
        (root / "src-tauri" / "tauri.conf.json").write_text(
            json.dumps(config), encoding="utf-8"
        )
        capability = {
            "identifier": "main",
            "windows": ["main"],
            "permissions": ["core:default", "core:window:allow-hide"],
        }
        (root / "src-tauri" / "capabilities" / "main.json").write_text(
            json.dumps(capability), encoding="utf-8"
        )
        (root / "package.json").write_text(
            json.dumps(
                {
                    "dependencies": {"@tauri-apps/api": "2.11.1"},
                    "devDependencies": {"@tauri-apps/cli": "2.11.4"},
                }
            ),
            encoding="utf-8",
        )
        (root / "src-tauri" / "src" / "lib.rs").write_text(
            """
#[tauri::command]
fn ping() {}

fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .invoke_handler(tauri::generate_handler![ping]);
}
""".strip(),
            encoding="utf-8",
        )
        (root / "src" / "input.ts").write_text(
            """
getCurrentWebview().onDragDropEvent((event) => {
  return ["enter", "over", "drop", "leave"].includes(event.payload.type);
});
""".strip(),
            encoding="utf-8",
        )
        (root / "src" / "app.css").write_text(
            "html, body { background: transparent; overflow: clip; }",
            encoding="utf-8",
        )


if __name__ == "__main__":
    unittest.main()
