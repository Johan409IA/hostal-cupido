#!/usr/bin/env python3
"""Read-only audit for production Tauri 2 desktop project fundamentals."""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass
class Finding:
    level: str
    message: str


class Audit:
    def __init__(self, root: Path) -> None:
        self.root = root.resolve()
        self.tauri = self.root / "src-tauri"
        self.findings: list[Finding] = []

    def add(self, level: str, message: str) -> None:
        self.findings.append(Finding(level, message))

    def read(self, path: Path) -> str:
        try:
            return path.read_text(encoding="utf-8")
        except (OSError, UnicodeError):
            return ""

    def source_text(self) -> str:
        roots = [self.root / "src", self.tauri / "src"]
        parts: list[str] = []
        for directory in roots:
            if not directory.is_dir():
                continue
            for path in sorted(directory.rglob("*")):
                if path.suffix in {".rs", ".ts", ".tsx", ".js", ".jsx", ".css", ".html"}:
                    parts.append(self.read(path))
        return "\n".join(parts)

    def load_json(self, path: Path) -> dict[str, Any] | None:
        try:
            value = json.loads(self.read(path))
        except json.JSONDecodeError as error:
            self.add("FAIL", f"{path.relative_to(self.root)} is invalid JSON: {error}")
            return None
        if not isinstance(value, dict):
            self.add("FAIL", f"{path.relative_to(self.root)} must contain a JSON object")
            return None
        return value

    def run(self) -> int:
        if not self.tauri.is_dir():
            self.add("FAIL", "src-tauri directory was not found")
            return self.report()

        config_path = self.tauri / "tauri.conf.json"
        cargo_path = self.tauri / "Cargo.toml"
        if not cargo_path.is_file():
            self.add("FAIL", "src-tauri/Cargo.toml was not found")
        else:
            cargo = self.read(cargo_path)
            if not re.search(r"(?m)^tauri\s*=", cargo):
                self.add("FAIL", "Cargo.toml does not declare the tauri crate")
            if "[profile.release]" not in cargo:
                self.add("WARN", "Cargo.toml has no explicit [profile.release] policy")

        if not config_path.is_file():
            self.add("FAIL", "src-tauri/tauri.conf.json was not found")
            return self.report()

        config = self.load_json(config_path)
        if config is None:
            return self.report()

        self.audit_config(config)
        self.audit_capabilities()
        self.audit_source(config)
        self.audit_versions()
        self.audit_file_sizes()
        return self.report()

    def audit_config(self, config: dict[str, Any]) -> None:
        identifier = config.get("identifier")
        if not isinstance(identifier, str) or identifier.count(".") < 2:
            self.add("WARN", "Use a stable reverse-domain application identifier")

        build = config.get("build", {})
        if not isinstance(build, dict) or not build.get("frontendDist"):
            self.add("FAIL", "build.frontendDist is missing")
        if isinstance(build, dict) and build.get("devUrl") and not build.get("beforeDevCommand"):
            self.add("WARN", "devUrl is configured without beforeDevCommand")

        app = config.get("app", {})
        security = app.get("security", {}) if isinstance(app, dict) else {}
        if not isinstance(security, dict) or not security.get("csp"):
            self.add("FAIL", "app.security.csp is missing")
        elif "*" in str(security.get("csp")):
            self.add("WARN", "CSP contains a wildcard; verify that it is strictly required")

        windows = app.get("windows", []) if isinstance(app, dict) else []
        if not isinstance(windows, list) or not windows:
            self.add("WARN", "No statically configured window was found")
            return

        labels: set[str] = set()
        for window in windows:
            if not isinstance(window, dict):
                continue
            label = str(window.get("label", "<unlabeled>"))
            if label in labels:
                self.add("FAIL", f"duplicate window label: {label}")
            labels.add(label)
            if window.get("transparent") is True and window.get("decorations") is not False:
                self.add("WARN", f"{label}: transparent window still has decorations")
            if window.get("visible") is False and window.get("skipTaskbar") is not True:
                self.add("WARN", f"{label}: hidden-at-start window is not configured to skip taskbar")
            if window.get("dragDropEnabled") is True:
                self.add("PASS", f"{label}: native drag/drop is enabled")

    def audit_capabilities(self) -> None:
        directory = self.tauri / "capabilities"
        files = sorted(directory.glob("*.json")) if directory.is_dir() else []
        if not files:
            self.add("FAIL", "No JSON capability file was found")
            return
        for path in files:
            value = self.load_json(path)
            if value is None:
                continue
            windows = value.get("windows", [])
            if "*" in windows:
                self.add("WARN", f"{path.name}: wildcard window scope")
            if value.get("remote"):
                self.add("WARN", f"{path.name}: remote webview capability needs threat-model review")
            permissions = value.get("permissions", [])
            permission_text = json.dumps(permissions)
            if "shell:default" in permission_text or '"shell:allow-execute"' in permission_text:
                self.add("WARN", f"{path.name}: frontend shell execution permission is enabled")

    def audit_source(self, config: dict[str, Any]) -> None:
        source = self.source_text()
        compact = source.replace(" ", "")
        if "tauri_plugin_single_instance" in source:
            builder_section = source[source.find("tauri::Builder") :]
            first_plugin = re.search(r"\.plugin\(([^\n]+)", builder_section)
            if first_plugin and "single_instance" not in first_plugin.group(1):
                self.add("WARN", "Single-instance does not appear to be the first registered plugin")

        windows = config.get("app", {}).get("windows", [])
        drag_enabled = any(
            isinstance(window, dict) and window.get("dragDropEnabled") is True
            for window in windows
        )
        if drag_enabled:
            if "onDragDropEvent" not in source:
                self.add("WARN", "Native drag/drop is enabled but no onDragDropEvent listener was found")
            for event_type in ("enter", "over", "drop", "leave"):
                if f'"{event_type}"' not in source and f"'{event_type}'" not in source:
                    self.add("WARN", f"Drag lifecycle does not visibly handle {event_type}")

        transparent = any(
            isinstance(window, dict) and window.get("transparent") is True
            for window in windows
        )
        if transparent:
            if "background:transparent" not in compact:
                self.add("WARN", "Transparent window has no obvious transparent document background")
            if "overflow:clip" not in compact:
                self.add("WARN", "Transparent window has no obvious overflow clipping for clean corners")

        if "onFocusChanged" in source and "suppress" not in source.lower():
            self.add("WARN", "Focus-change handling has no obvious native-interaction suppression")
        if "Command::new(\"sh\")" in source or "Command::new(\"bash\")" in source:
            self.add("WARN", "Rust launches a shell; audit all interpolation and quoting")
        if "#[tauri::command]" not in source:
            self.add("WARN", "No Rust Tauri command was found")
        if "spawn_blocking" not in source and (
            "std::fs::read" in source or ".output()" in source or "image::load" in source
        ):
            self.add("WARN", "Blocking native work exists without an obvious spawn_blocking boundary")

    def audit_versions(self) -> None:
        package_path = self.root / "package.json"
        if not package_path.is_file():
            self.add("WARN", "package.json was not found; skip JS Tauri version audit")
            return
        package = self.load_json(package_path)
        if package is None:
            return
        dependencies: dict[str, str] = {}
        for key in ("dependencies", "devDependencies"):
            value = package.get(key, {})
            if isinstance(value, dict):
                dependencies.update({str(name): str(version) for name, version in value.items()})
        api = dependencies.get("@tauri-apps/api")
        cli = dependencies.get("@tauri-apps/cli")
        if not api:
            self.add("WARN", "@tauri-apps/api is not declared")
        if not cli:
            self.add("WARN", "@tauri-apps/cli is not declared")
        if api and cli:
            api_major = re.search(r"\d+", api)
            cli_major = re.search(r"\d+", cli)
            if api_major and cli_major and api_major.group() != cli_major.group():
                self.add("FAIL", f"Tauri JS API ({api}) and CLI ({cli}) major versions differ")

    def audit_file_sizes(self) -> None:
        for directory in (self.root / "src", self.tauri / "src"):
            if not directory.is_dir():
                continue
            for path in directory.rglob("*"):
                if path.suffix not in {".rs", ".ts", ".tsx", ".js", ".jsx", ".css"}:
                    continue
                try:
                    with path.open(encoding="utf-8") as source_file:
                        lines = sum(1 for _ in source_file)
                except (OSError, UnicodeError):
                    continue
                if lines > 500:
                    self.add("WARN", f"{path.relative_to(self.root)} has {lines} lines")

    def report(self) -> int:
        order = {"FAIL": 0, "WARN": 1, "PASS": 2}
        for finding in sorted(self.findings, key=lambda item: (order[item.level], item.message)):
            print(f"[{finding.level}] {finding.message}")
        failures = sum(finding.level == "FAIL" for finding in self.findings)
        warnings = sum(finding.level == "WARN" for finding in self.findings)
        passes = sum(finding.level == "PASS" for finding in self.findings)
        print(f"\nSummary: {failures} failure(s), {warnings} warning(s), {passes} noted pass(es)")
        return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("project", nargs="?", default=".", help="Tauri project root")
    args = parser.parse_args()
    return Audit(Path(args.project)).run()


if __name__ == "__main__":
    sys.exit(main())
