#!/usr/bin/env python3
"""Pin the inputs of the panic-variant filename clobber (issue #359 WP-1, design §5/§6.7).

`scripts/run-release-workspace-tests.sh` explains at length why `--release --workspace
--all-targets` overrides `[profile.release].panic` back to `unwind`: three packages carry a
`cdylib`/`staticlib` lib target, whose crate-type outputs are NOT hashed into their filenames the
way an rlib's are, so building both panic variants of one of those libs in the same invocation
lets the second clobber the first on disk and a downstream unit link whichever landed last. That
override is a workaround for a fact about which packages carry `cdylib`/`staticlib` and about
`[profile.release]` -- and this script pins exactly that fact, not the workaround. It does not
build or link anything itself: `cargo metadata` is read-only, and the root manifest is parsed as
text. If either input drifts (a new shipped cdylib, a changed profile), the override's reasoning
should be re-derived, not silently kept valid by a stale pin.

The five packages below are exactly the shipped-cdylib/staticlib set as of #359: `capi` (rlib +
staticlib + cdylib, the native C ABI), `effect-package` and `host-web` (rlib + cdylib), and the two
wasm guest test cdylibs `wasm-gate-guest`/`wasm-console-guest` (their own target triple, so they do
not actually clobber, but they are still exactly-pinned membership, not "cdylib crates in general").

By default this runs `cargo metadata --format-version 1 --locked --no-deps` itself. `--metadata
<file>` accepts a saved `cargo metadata` JSON document instead, so the crate-type assertion can be
tested hermetically without a Rust toolchain; `--self-test` needs neither cargo nor a file and
proves the checker rejects each protected mutation from synthetic metadata/manifest text held only
in memory.
"""
from __future__ import annotations

import argparse
import json
import pathlib
import subprocess
import sys
import tempfile
import tomllib

EXPECTED_CDYLIB_OR_STATICLIB = frozenset({
    "capi",
    "effect-package",
    "host-web",
    "wasm-gate-guest",
    "wasm-console-guest",
})
NATIVE_LIB_CRATE_TYPES = frozenset({"cdylib", "staticlib"})


class Invalid(RuntimeError):
    pass


def crate_type_packages(metadata: object) -> set[str]:
    """The set of package names carrying a cdylib/staticlib lib target, per `cargo metadata`."""
    if not isinstance(metadata, dict):
        raise Invalid("cargo metadata output must be a JSON object")
    packages = metadata.get("packages")
    if not isinstance(packages, list):
        raise Invalid("cargo metadata: missing or malformed 'packages' list")
    found: set[str] = set()
    for package in packages:
        if not isinstance(package, dict):
            raise Invalid("cargo metadata: malformed package entry")
        name = package.get("name")
        targets = package.get("targets")
        if not isinstance(name, str) or not isinstance(targets, list):
            raise Invalid("cargo metadata: malformed package entry")
        for target in targets:
            if not isinstance(target, dict):
                continue
            kinds = set(target.get("kind") or []) | set(target.get("crate_types") or [])
            if kinds & NATIVE_LIB_CRATE_TYPES:
                found.add(name)
                break
    return found


def check_crate_types(metadata: object) -> set[str]:
    actual = crate_type_packages(metadata)
    if actual != EXPECTED_CDYLIB_OR_STATICLIB:
        missing = EXPECTED_CDYLIB_OR_STATICLIB - actual
        extra = actual - EXPECTED_CDYLIB_OR_STATICLIB
        detail = []
        if missing:
            detail.append(f"missing: {', '.join(sorted(missing))}")
        if extra:
            detail.append(f"unexpected: {', '.join(sorted(extra))}")
        raise Invalid(
            "cdylib/staticlib package set drifted from the pinned clobber inputs "
            f"({'; '.join(detail)})"
        )
    return actual


def check_release_profile(cargo_toml_text: str) -> None:
    try:
        document = tomllib.loads(cargo_toml_text)
    except tomllib.TOMLDecodeError as error:
        raise Invalid(f"root Cargo.toml is not valid TOML: {error}") from error
    profile = document.get("profile")
    release = profile.get("release") if isinstance(profile, dict) else None
    if not isinstance(release, dict):
        raise Invalid("root Cargo.toml is missing a [profile.release] table")
    if release.get("panic") != "abort":
        raise Invalid(
            f"[profile.release].panic must be \"abort\" (the D12 shipped-codegen pin), "
            f"got {release.get('panic')!r}"
        )
    if release.get("lto") != "fat":
        raise Invalid(f"[profile.release].lto must be \"fat\", got {release.get('lto')!r}")
    if release.get("codegen-units") != 1:
        raise Invalid(
            f"[profile.release].codegen-units must be 1, got {release.get('codegen-units')!r}"
        )


def load_metadata(metadata_path: pathlib.Path | None, root: pathlib.Path) -> object:
    if metadata_path is not None:
        try:
            text = metadata_path.read_text(encoding="utf-8")
        except OSError as error:
            raise Invalid(f"cannot read --metadata file: {error}") from error
        try:
            return json.loads(text)
        except json.JSONDecodeError as error:
            raise Invalid(f"--metadata file is not valid JSON: {error}") from error
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--locked", "--no-deps"],
        cwd=root,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        raise Invalid(f"cargo metadata failed: {result.stderr.decode(errors='replace').strip()}")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise Invalid(f"cargo metadata output is not valid JSON: {error}") from error


def check(root: pathlib.Path, metadata_path: pathlib.Path | None) -> set[str]:
    metadata = load_metadata(metadata_path, root)
    actual = check_crate_types(metadata)
    manifest = root / "Cargo.toml"
    try:
        cargo_toml_text = manifest.read_text(encoding="utf-8")
    except OSError as error:
        raise Invalid(f"cannot read {manifest}: {error}") from error
    check_release_profile(cargo_toml_text)
    return actual


# --------------------------------------------------------------------------------------------
# --self-test: synthetic metadata/manifest text held only in memory, no cargo, no filesystem
# fixture beyond one throwaway temp file used to exercise --metadata's own JSON-parsing path.
# --------------------------------------------------------------------------------------------

def _pkg(name: str, crate_types: list[str]) -> dict:
    return {
        "name": name,
        "targets": [{
            "name": name.replace("-", "_"),
            "kind": list(crate_types),
            "crate_types": list(crate_types),
        }],
    }


def _valid_metadata() -> dict:
    return {
        "packages": [
            _pkg("capi", ["rlib", "staticlib", "cdylib"]),
            _pkg("effect-package", ["rlib", "cdylib"]),
            _pkg("host-web", ["rlib", "cdylib"]),
            _pkg("wasm-gate-guest", ["cdylib"]),
            _pkg("wasm-console-guest", ["cdylib"]),
            _pkg("engine", ["lib"]),
            _pkg("lane", ["lib"]),
            _pkg("math", ["lib"]),
        ]
    }


VALID_CARGO_TOML = (
    "[workspace]\n"
    "members = []\n"
    "\n"
    "[workspace.package]\n"
    "license = \"Apache-2.0\"\n"
    "\n"
    "[profile.release]\n"
    "lto = \"fat\"\n"
    "codegen-units = 1\n"
    "panic = \"abort\"\n"
    "debug = 1\n"
)


def _expect_invalid(label: str, fn, *args, needle: str | None = None) -> None:
    try:
        fn(*args)
    except Invalid as error:
        if needle is not None and needle not in str(error):
            raise AssertionError(f"{label}: wrong rejection reason: {error}") from error
        return
    raise AssertionError(f"{label}: accepted an invalid input")


def self_test() -> None:
    # A fully valid pair passes and reports the exact pinned set.
    actual = check_crate_types(_valid_metadata())
    assert actual == EXPECTED_CDYLIB_OR_STATICLIB, actual
    check_release_profile(VALID_CARGO_TOML)

    # A sixth cdylib crate is rejected.
    sixth = _valid_metadata()
    sixth["packages"].append(_pkg("sixth-shipped-cdylib", ["cdylib"]))
    _expect_invalid("sixth cdylib crate", check_crate_types, sixth, needle="unexpected")

    # A missing expected crate is rejected.
    missing = _valid_metadata()
    missing["packages"] = [
        package for package in missing["packages"] if package["name"] != "wasm-console-guest"
    ]
    _expect_invalid("missing expected crate", check_crate_types, missing, needle="missing")

    # panic = "unwind" is rejected.
    _expect_invalid(
        "panic=unwind",
        check_release_profile,
        VALID_CARGO_TOML.replace('panic = "abort"', 'panic = "unwind"'),
        needle="panic",
    )

    # A missing [profile.release] table is rejected.
    _expect_invalid(
        "missing profile.release",
        check_release_profile,
        "[workspace]\nmembers = []\n",
        needle="profile.release",
    )

    # Malformed JSON fed through the --metadata file-loading path is rejected.
    with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as handle:
        handle.write("{not valid json")
        temp_path = pathlib.Path(handle.name)
    try:
        _expect_invalid(
            "malformed metadata JSON",
            load_metadata, temp_path, pathlib.Path("."),
            needle="JSON",
        )
    finally:
        temp_path.unlink()

    # Two more mutations beyond the required five, for the other two pinned profile fields.
    _expect_invalid(
        "lto not fat",
        check_release_profile,
        VALID_CARGO_TOML.replace('lto = "fat"', 'lto = "thin"'),
        needle="lto",
    )
    _expect_invalid(
        "codegen-units not 1",
        check_release_profile,
        VALID_CARGO_TOML.replace("codegen-units = 1", "codegen-units = 16"),
        needle="codegen-units",
    )

    print("check-release-shape self-test: ok")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path,
                        default=pathlib.Path(__file__).resolve().parent.parent)
    parser.add_argument("--metadata", type=pathlib.Path,
                        help="a saved `cargo metadata --format-version 1` JSON document, "
                             "for hermetic testing without a Rust toolchain")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        self_test()
        return 0

    try:
        actual = check(args.root.resolve(), args.metadata)
    except Invalid as error:
        print(f"release shape check failed: {error}", file=sys.stderr)
        return 1
    print("release shape: cdylib/staticlib packages = " + ", ".join(sorted(actual)))
    print("release shape: [profile.release] panic=\"abort\" lto=\"fat\" codegen-units=1: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
