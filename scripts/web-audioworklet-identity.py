#!/usr/bin/env python3
"""The shipped AudioWorklet module's identity, reported on every change (issue #1061).

Owner decision 5 (`docs/rulings/engine-footprint-2026-09-28.md`): every PR builds the shipped
module and runs every artifact gate against those bytes, and reports whether the module changed;
only a release change must match the recorded fingerprint and carry a fresh three-browser
qualification. `qualification.yml`'s `artifact-identity` job runs this script:

* ``base --event EVENT [--before SHA]`` prints the commit this run's module is compared against,
  or nothing, with the reason on stderr. A pull request run checks out GitHub's merge commit, whose
  first parent is the base-branch tip the change is merged onto, so the comparison shows only what
  the change itself does to the module (the merge base would also count what the base branch did
  since the change branched). A `main` push compares against the previous tip, `--before`.
* ``report --built SHA256 --twin MODULE [--base COMMIT --base-module MODULE]`` prints the Markdown
  report for the job summary. Its headline is ``ARTIFACT CHANGED`` or ``ARTIFACT UNCHANGED`` against
  the base's module (``ARTIFACT BASE UNAVAILABLE`` when the run has no base), and an issue gate that
  claims "shipped artifact unchanged" cites that line. It fails (exit 1) when
  - the twin module, built from another checkout path and ``CARGO_HOME``, differs from the bytes the
    `artifact` job built: the build is not a function of the source alone; or
  - the change is a release change -- it edits the committed pin, or `npm-publish.yml`'s
    ``PACKAGE_VERSION`` or ``EXPECTED_WORKLET_SHA256`` -- and the pin, ``EXPECTED_WORKLET_SHA256``
    or the recorded browser qualification (`results.json`'s ``wasmSha256``, with a canonical
    ``candidateCommit``) does not describe the built bytes. `docs/RELEASE.md` is the procedure.
* ``--self-test`` proves every rule above goes red on its own mutation, in a scratch repository.

The script builds nothing and reads only git objects and the modules it is handed.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent.parent
PIN = "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256"
NPM_PUBLISH = ".github/workflows/npm-publish.yml"
RESULTS = "hosts/host-web/qualification/results.json"
RELEASE_DOC = "docs/RELEASE.md"
SHA256 = re.compile(r"[0-9a-f]{64}")
COMMIT = re.compile(r"[0-9a-f]{40}")
RELEASE_LINE = re.compile(r'^  (PACKAGE_VERSION|EXPECTED_WORKLET_SHA256): ".*"$', re.MULTILINE)
EXPECTED_WORKLET = re.compile(r'^  EXPECTED_WORKLET_SHA256: "([^"]*)"$', re.MULTILINE)


class Usage(RuntimeError):
    pass


def git(root: pathlib.Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(["git", "-C", str(root), *args], stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, text=True, check=False)


def show(root: pathlib.Path, commit: str, path: str) -> str | None:
    result = git(root, "show", f"{commit}:{path}")
    return result.stdout if result.returncode == 0 else None


def digest(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def base_commit(root: pathlib.Path, event: str, before: str) -> tuple[str | None, str]:
    """The commit a run's module is compared against, or None and the reason."""
    if event == "pull_request":
        parents = git(root, "rev-list", "--parents", "-n", "1", "HEAD").stdout.split()
        if len(parents) != 3:
            return None, "HEAD is not the pull request's merge commit"
        return parents[1], "the base-branch tip the merge commit sits on"
    if event == "push":
        if (COMMIT.fullmatch(before) is None or set(before) == {"0"}
                or git(root, "cat-file", "-e", f"{before}^{{commit}}").returncode != 0):
            return None, f"the push names no previous tip in this clone ({before or 'none'})"
        return before, "the previous tip of the pushed branch"
    return None, f"a {event or 'unnamed'} run has no base"


def release_identity(text: str | None) -> list[str] | None:
    """`npm-publish.yml`'s release identity: its `PACKAGE_VERSION` and `EXPECTED_WORKLET_SHA256`."""
    return None if text is None else sorted(m.group(0) for m in RELEASE_LINE.finditer(text))


def release_change(root: pathlib.Path, base: str) -> list[str]:
    """What this change edits of the release fingerprint, relative to its base."""
    edited = []
    if show(root, base, PIN) != show(root, "HEAD", PIN):
        edited.append(f"`{PIN}`")
    if release_identity(show(root, base, NPM_PUBLISH)) != release_identity(
            show(root, "HEAD", NPM_PUBLISH)):
        edited.append(f"`{NPM_PUBLISH}`'s `PACKAGE_VERSION` or `EXPECTED_WORKLET_SHA256`")
    return edited


def release_checks(root: pathlib.Path, built: str) -> tuple[list[str], bool]:
    lines, ok = [], True
    pin = (show(root, "HEAD", PIN) or "").strip()
    if pin == built:
        lines.append(f"  - the pin `{PIN}` is `{pin}`: the built bytes")
    else:
        ok = False
        lines.append(f"  - **FAIL** the pin `{PIN}` is `{pin or 'missing'}`, not the built `{built}`")
    match = EXPECTED_WORKLET.search(show(root, "HEAD", NPM_PUBLISH) or "")
    expected = match.group(1) if match else None
    if expected == pin:
        lines.append(f"  - `{NPM_PUBLISH}`'s `EXPECTED_WORKLET_SHA256` is the pin")
    else:
        ok = False
        lines.append(f"  - **FAIL** `{NPM_PUBLISH}`'s `EXPECTED_WORKLET_SHA256` is "
                     f"`{expected or 'missing'}`, not the pin `{pin or 'missing'}`")
    try:
        results = json.loads(show(root, "HEAD", RESULTS) or "null")
    except json.JSONDecodeError:
        results = None
    results = results if isinstance(results, dict) else {}
    recorded = results.get("wasmSha256")
    candidate = results.get("candidateCommit")
    if recorded == built:
        lines.append(f"  - `{RESULTS}` records the three-browser qualification of the built bytes")
    else:
        ok = False
        lines.append(f"  - **FAIL** artifact-lineage: `{RESULTS}` records a qualification of "
                     f"`{recorded}`, not of the built `{built}`: re-record it "
                     "(`npm run qualify -- --browser all --record-matrix`)")
    if isinstance(candidate, str) and COMMIT.fullmatch(candidate):
        lines.append(f"  - its `candidateCommit` is `{candidate}`")
    else:
        ok = False
        lines.append(f"  - **FAIL** candidate-lineage: `{RESULTS}`'s `candidateCommit` "
                     f"`{candidate}` is not a canonical lowercase 40-hex commit")
    return lines, ok


def report(root: pathlib.Path, built: str, twin_module: pathlib.Path, base: str | None,
           base_module: pathlib.Path | None) -> tuple[list[str], bool]:
    if SHA256.fullmatch(built) is None:
        raise Usage(f"--built is not a lowercase sha256: {built!r}")
    if not twin_module.is_file():
        raise Usage(f"--twin is not a file: {twin_module}")
    if (base is None) != (base_module is None):
        raise Usage("--base and --base-module go together")
    if base_module is not None and not base_module.is_file():
        raise Usage(f"--base-module is not a file: {base_module}")
    head = git(root, "rev-parse", "HEAD").stdout.strip()
    ok = True
    lines = ["### Shipped AudioWorklet artifact", ""]
    if base is None:
        lines.append(f"ARTIFACT BASE UNAVAILABLE: `{built}` at `{head}`; this run has no base to "
                     "compare against")
    else:
        was = digest(base_module)
        if was == built:
            lines.append(f"ARTIFACT UNCHANGED: `{built}` at `{head}` is the base `{base}`'s module")
        else:
            lines.append(f"ARTIFACT CHANGED: `{was}` at base `{base}` -> `{built}` at `{head}`")
    lines += ["", f"- Every artifact gate in this run read `{built}`: the `artifact` job built it, "
              "and each job that reads it checks its download against that digest."]
    twin = digest(twin_module)
    if twin == built:
        lines.append("- Reproducible: a second build from another checkout path and `CARGO_HOME` "
                     "produced the same bytes.")
    else:
        ok = False
        lines.append(f"- **FAIL** NOT REPRODUCIBLE: a second build from another checkout path and "
                     f"`CARGO_HOME` produced `{twin}`. The module is not a function of the source "
                     "alone; an absolute path is the usual cause (`build-web-audioworklet.sh` "
                     "remaps `CARGO_HOME` and the repository root, not `env!` values).")
    edited = release_change(root, base) if base is not None else []
    pin = (show(root, "HEAD", PIN) or "").strip()
    if edited:
        lines.append(f"- RELEASE: this change edits {' and '.join(edited)}, so the release "
                     f"fingerprint must describe the built bytes ({RELEASE_DOC}):")
        checked, release_ok = release_checks(root, built)
        lines += checked
        ok = ok and release_ok
    else:
        state = "are" if pin == built else "are not"
        lines.append(f"- Release fingerprint not checked: this change does not edit the pin or "
                     f"`{NPM_PUBLISH}`'s release identity. The pin is `{pin or 'missing'}`; the "
                     f"built bytes {state} the pinned release module ({RELEASE_DOC}).")
    return lines, ok


# ---------------------------------------------------------------------------------------------
# Self-test: each rule is the red mutation of one fixture, in a scratch repository.


def self_test() -> None:
    scratch = pathlib.Path(tempfile.mkdtemp(prefix="web-audioworklet-identity-"))
    try:
        run_self_test(scratch)
    finally:
        shutil.rmtree(scratch)


def run_self_test(scratch: pathlib.Path) -> None:
    repo = scratch / "repo"
    repo.mkdir()
    modules = {}
    for name in ("a", "b", "c"):
        path = scratch / f"{name}.wasm"
        path.write_bytes(f"module {name}\n".encode())
        modules[name] = (path, digest(path))
    a, b = modules["a"][1], modules["b"][1]

    def must(result: subprocess.CompletedProcess[str]) -> str:
        if result.returncode != 0:
            raise AssertionError(f"fixture git failed: {result.stderr}")
        return result.stdout.strip()

    def commit(message: str, pin: str, version: str, expected: str, recorded: str,
               candidate: str = "1" * 40, extra: str | None = None) -> str:
        (repo / PIN).parent.mkdir(parents=True, exist_ok=True)
        (repo / NPM_PUBLISH).parent.mkdir(parents=True, exist_ok=True)
        (repo / RESULTS).parent.mkdir(parents=True, exist_ok=True)
        (repo / PIN).write_text(f"{pin}\n")
        (repo / NPM_PUBLISH).write_text(
            f'env:\n  PACKAGE_NAME: "@misofm/engine"\n  PACKAGE_VERSION: "{version}"\n'
            f'  EXPECTED_WORKLET_SHA256: "{expected}"\n')
        (repo / RESULTS).write_text(json.dumps(
            {"candidateCommit": candidate, "wasmSha256": recorded}, indent=2) + "\n")
        if extra is not None:
            (repo / "notes.txt").write_text(extra)
        must(git(repo, "add", "-A"))
        must(git(repo, "-c", "user.name=identity-self-test", "-c",
                 "user.email=identity-self-test@invalid", "commit", "-q", "--allow-empty",
                 "-m", message))
        return must(git(repo, "rev-parse", "HEAD"))

    must(git(repo, "init", "-q"))
    base = commit("base: release of module a", a, "1.0.0", a, a)
    failures = []

    def case(label: str, built: str, twin: str, base_module: str | None, ok: bool,
             needles: tuple[str, ...]) -> None:
        try:
            lines, observed = report(repo, built, modules[twin][0],
                                     None if base_module is None else base,
                                     None if base_module is None else modules[base_module][0])
        except Usage as error:
            failures.append(f"{label}: usage error {error}")
            return
        text = "\n".join(lines)
        if observed != ok:
            failures.append(f"{label}: expected ok={ok}, got ok={observed}\n{text}")
        for needle in needles:
            if needle not in text:
                failures.append(f"{label}: {needle!r} missing from\n{text}")

    def at(ref: str) -> None:
        must(git(repo, "checkout", "-q", ref))

    # An ordinary change: no release edit, so the pin is reported and never checked.
    ordinary = commit("an ordinary change", a, "1.0.0", a, a, extra="edited\n")
    case("unchanged", a, "a", "a", True, ("ARTIFACT UNCHANGED", "Reproducible",
                                          "not checked", "are the pinned release module"))
    case("changed", b, "b", "a", True, ("ARTIFACT CHANGED", f"`{a}` at base", f"-> `{b}`",
                                        "are not the pinned release module"))
    case("not reproducible", b, "a", "a", False, ("NOT REPRODUCIBLE", f"produced `{a}`"))
    case("no base", b, "b", None, True, ("ARTIFACT BASE UNAVAILABLE",))
    # A release change describing module b, and each way it can fail to.
    at(base)
    commit("release b", b, "1.1.0", b, b)
    case("release", b, "b", "a", True, ("ARTIFACT CHANGED", "RELEASE: this change edits",
                                        "the built bytes", "is the pin", "records the three"))
    case("release built other bytes", modules["c"][1], "c", "a", False,
         ("**FAIL** the pin", "**FAIL** artifact-lineage"))
    at(base)
    commit("version bump without a re-pin", a, "1.1.0", a, a)
    case("release without a re-pin", b, "b", "a", False,
         ("PACKAGE_VERSION", "**FAIL** the pin", "**FAIL** artifact-lineage"))
    at(base)
    commit("re-pin without npm-publish", b, "1.0.0", a, b)
    case("release, EXPECTED_WORKLET_SHA256 stale", b, "b", "a", False,
         (f"`{PIN}`", "**FAIL** `.github/workflows/npm-publish.yml`'s `EXPECTED_WORKLET_SHA256`",))
    at(base)
    commit("release without a fresh qualification", b, "1.1.0", b, a)
    case("release, results.json stale", b, "b", "a", False, ("**FAIL** artifact-lineage",))
    at(base)
    commit("release with a malformed candidate", b, "1.1.0", b, b, candidate="0" * 39)
    case("release, candidate malformed", b, "b", "a", False, ("**FAIL** candidate-lineage",))
    for bad in (("A" * 64), ("0" * 63), ""):
        try:
            report(repo, bad, modules["a"][0], None, None)
            failures.append(f"malformed --built {bad!r} was accepted")
        except Usage:
            pass

    # The base: a pull request's merge commit compares against its first parent, a push against
    # the previous tip, and nothing else has a base.
    at(base)
    side = must(git(repo, "rev-parse", ordinary))
    must(git(repo, "-c", "user.name=identity-self-test", "-c",
             "user.email=identity-self-test@invalid", "merge", "-q", "--no-ff", "-m", "merge",
             side))
    merge_parent = must(git(repo, "rev-parse", "HEAD^1"))
    expectations = (
        ("pull_request", "", merge_parent),
        ("push", base, base),
        ("push", "0" * 40, None),
        ("push", "f" * 40, None),
        ("push", "", None),
        ("workflow_dispatch", base, None),
    )
    for event, before, expected in expectations:
        observed, reason = base_commit(repo, event, before)
        if observed != expected:
            failures.append(f"base {event} {before!r}: expected {expected}, got {observed} "
                            f"({reason})")
    at(ordinary)
    observed, _ = base_commit(repo, "pull_request", "")
    if observed is not None:
        failures.append("a pull_request run whose HEAD is not a merge commit found a base")

    # The command line: exit 1 on a failed rule, 0 otherwise, 2 on a usage error.
    at(ordinary)
    script = str(pathlib.Path(__file__).resolve())
    for args, status in (
        (["report", "--built", a, "--twin", str(modules["a"][0]), "--base", base,
          "--base-module", str(modules["a"][0])], 0),
        (["report", "--built", b, "--twin", str(modules["a"][0])], 1),
        (["report", "--built", b, "--twin", str(scratch / "missing.wasm")], 2),
        (["report", "--built", b, "--twin", str(modules["b"][0]), "--base", base], 2),
    ):
        result = subprocess.run([sys.executable, "-B", script, "--root", str(repo), *args],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                                check=False)
        if result.returncode != status:
            failures.append(f"CLI {args[:3]}: expected exit {status}, got {result.returncode}: "
                            f"{result.stderr}")

    if failures:
        raise AssertionError("\n".join(failures))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--root", type=pathlib.Path, default=ROOT)
    parser.add_argument("--self-test", action="store_true")
    commands = parser.add_subparsers(dest="command")
    base = commands.add_parser("base")
    base.add_argument("--event", required=True)
    base.add_argument("--before", default="")
    check = commands.add_parser("report")
    check.add_argument("--built", required=True)
    check.add_argument("--twin", required=True, type=pathlib.Path)
    check.add_argument("--base")
    check.add_argument("--base-module", type=pathlib.Path)
    args = parser.parse_args()

    if args.self_test:
        if args.command is not None:
            parser.error("--self-test takes no command")
        self_test()
        print("web AudioWorklet identity self-test passed: changed, unchanged, no base, "
              "not reproducible, and five release mutations")
        return 0
    if args.command == "base":
        commit_id, reason = base_commit(args.root, args.event, args.before)
        print(f"artifact base: {commit_id or 'none'} ({reason})", file=sys.stderr)
        if commit_id is not None:
            print(commit_id)
        return 0
    if args.command == "report":
        try:
            lines, ok = report(args.root, args.built, args.twin, args.base or None,
                               args.base_module)
        except Usage as error:
            print(f"web-audioworklet-identity: {error}", file=sys.stderr)
            return 2
        print("\n".join(lines))
        if not ok:
            print("web-audioworklet-identity: FAIL (see the report above)", file=sys.stderr)
        return 0 if ok else 1
    parser.error("a command (base, report) or --self-test is required")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
