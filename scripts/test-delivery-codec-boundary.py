#!/usr/bin/env python3
"""Mutation coverage for the retired delivery-codec Cargo boundary."""

from __future__ import annotations

import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
CHECKER = ROOT / "scripts" / "check-delivery-codec-boundary.py"


def run(root: Path, expected: int, diagnostic: str | None = None) -> None:
    result = subprocess.run(["python3", "-B", str(CHECKER), "--root", str(root)], capture_output=True, text=True)
    if result.returncode != expected:
        raise SystemExit(f"checker returned {result.returncode}, expected {expected}: {result.stdout}{result.stderr}")
    if diagnostic is not None and diagnostic not in f"{result.stdout}{result.stderr}":
        raise SystemExit(f"checker did not name {diagnostic!r}: {result.stdout}{result.stderr}")


def fixture(root: Path) -> None:
    (root / "crates" / "engine" / "src").mkdir(parents=True)
    (root / "Cargo.toml").write_text('[workspace]\nmembers = ["crates/engine"]\nresolver = "3"\n', encoding="utf-8")
    (root / "crates" / "engine" / "Cargo.toml").write_text(
        '[package]\nname = "engine"\nversion = "0.1.0"\nedition = "2024"\n', encoding="utf-8"
    )
    (root / "crates" / "engine" / "src" / "lib.rs").write_text('', encoding="utf-8")
    subprocess.run(["cargo", "generate-lockfile"], cwd=root, check=True, capture_output=True, text=True)


with tempfile.TemporaryDirectory() as temporary:
    root = Path(temporary)
    fixture(root)
    run(root, 0)

    (root / "sidecars" / "example" / "src").mkdir(parents=True)
    (root / "sidecars" / "example" / "Cargo.toml").write_text(
        '[package]\nname = "example"\nversion = "0.1.0"\nedition = "2024"\n', encoding="utf-8"
    )
    (root / "sidecars" / "example" / "src" / "lib.rs").write_text('', encoding="utf-8")
    (root / "Cargo.toml").write_text(
        '[workspace]\nmembers = ["crates/engine", "sidecars/example"]\nresolver = "3"\n', encoding="utf-8"
    )
    subprocess.run(["cargo", "generate-lockfile"], cwd=root, check=True, capture_output=True, text=True)
    run(root, 0)

    manifest = root / "Cargo.toml"
    manifest.write_text('[workspace]\nmembers = ["crates/engine", "sidecars/flac-decoder"]\nresolver = "3"\n', encoding="utf-8")
    (root / "sidecars" / "flac-decoder" / "src").mkdir(parents=True)
    (root / "sidecars" / "flac-decoder" / "Cargo.toml").write_text(
        '[package]\nname = "flac-decoder"\nversion = "0.1.0"\nedition = "2024"\n', encoding="utf-8"
    )
    (root / "sidecars" / "flac-decoder" / "src" / "lib.rs").write_text('', encoding="utf-8")
    run(root, 1, "flac-decoder")

with tempfile.TemporaryDirectory() as temporary:
    root = Path(temporary)
    fixture(root)
    (root / "sidecars" / "flac-decoder" / "src").mkdir(parents=True)
    (root / "sidecars" / "flac-decoder" / "Cargo.toml").write_text(
        '[package]\nname = "example"\nversion = "0.1.0"\nedition = "2024"\n', encoding="utf-8"
    )
    (root / "sidecars" / "flac-decoder" / "src" / "lib.rs").write_text('', encoding="utf-8")
    run(root, 1, "flac-decoder")

with tempfile.TemporaryDirectory() as temporary:
    root = Path(temporary)
    fixture(root)
    (root / "Cargo.toml").write_text(
        '[workspace]\nmembers = ["crates/engine"]\nresolver = "3"\n\n[workspace.dependencies]\nflacenc = "0.5.1"\n',
        encoding="utf-8",
    )
    run(root, 1, "flacenc")

with tempfile.TemporaryDirectory() as temporary:
    root = Path(temporary)
    fixture(root)
    (root / "crates" / "engine" / "Cargo.toml").write_text(
        '[package]\nname = "engine"\nversion = "0.1.0"\nedition = "2024"\n\n[dependencies]\ndelivery = { package = "symphonia", version = "0.5.4" }\n',
        encoding="utf-8",
    )
    run(root, 1, "symphonia")

with tempfile.TemporaryDirectory() as temporary:
    root = Path(temporary)
    fixture(root)
    with (root / "Cargo.lock").open("a", encoding="utf-8") as lock:
        lock.write('\n[[package]]\nname = "symphonia"\nversion = "0.5.4"\n')
    run(root, 1, "symphonia")

print("delivery-codec Cargo boundary mutation tests: ok")
