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
* ``toolchain --commit COMMIT`` prints the Rust toolchain COMMIT pinned for its own CI build of the
  module: its `qualification.yml`'s workflow-level ``RUSTUP_TOOLCHAIN`` (which overrides
  `rust-toolchain.toml` in CI), else its `rust-toolchain.toml` channel. The base's module is built
  with the base's toolchain, never this run's: a toolchain change moves the module with no source
  change, and the report is the only place that change shows (#1061 attempt 1, finding 1).
* ``report --built SHA256 --toolchain NAME --twin MODULE [--base COMMIT --base-toolchain NAME
  --base-module MODULE]`` prints the Markdown report for the job summary. Its headline is ``ARTIFACT CHANGED`` or ``ARTIFACT UNCHANGED`` against
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
QUALIFICATION = ".github/workflows/qualification.yml"
TOOLCHAIN_FILE = "rust-toolchain.toml"
# The workflow-level `env:` block: `env:` at column 0, then its two-space-indented entries.
WORKFLOW_ENV = re.compile(r"^env:\n((?:  .*\n|\s*\n)*)", re.MULTILINE)
ENV_TOOLCHAIN = re.compile(r'^  RUSTUP_TOOLCHAIN: *"?([^"\s#]+)"?\s*(?:#.*)?$', re.MULTILINE)
CHANNEL = re.compile(r'^channel *= *"([^"]+)"\s*$', re.MULTILINE)
TOOLCHAIN_NAME = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]*")


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


def pinned_toolchain(root: pathlib.Path, commit: str) -> tuple[str, str]:
    """The toolchain `commit` pinned for its own CI build of the module, and where it was read."""
    workflow = show(root, commit, QUALIFICATION) or ""
    block = WORKFLOW_ENV.search(workflow)
    found = ENV_TOOLCHAIN.search(block.group(1)) if block else None
    if found is not None:
        name, source = found.group(1), f"`{QUALIFICATION}`'s `RUSTUP_TOOLCHAIN`"
    else:
        channel = CHANNEL.search(show(root, commit, TOOLCHAIN_FILE) or "")
        if channel is None:
            raise Usage(f"{commit} pins no toolchain: no workflow-level RUSTUP_TOOLCHAIN in "
                        f"{QUALIFICATION} and no channel in {TOOLCHAIN_FILE}")
        name, source = channel.group(1), f"`{TOOLCHAIN_FILE}`'s channel"
    if TOOLCHAIN_NAME.fullmatch(name) is None:
        raise Usage(f"{commit} pins a malformed toolchain name: {name!r}")
    return name, source


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


def report(root: pathlib.Path, built: str, toolchain: str, twin_module: pathlib.Path,
           base: str | None, base_toolchain: str | None,
           base_module: pathlib.Path | None) -> tuple[list[str], bool]:
    if SHA256.fullmatch(built) is None:
        raise Usage(f"--built is not a lowercase sha256: {built!r}")
    for name in (toolchain, base_toolchain):
        if name is not None and TOOLCHAIN_NAME.fullmatch(name) is None:
            raise Usage(f"not a toolchain name: {name!r}")
    if not twin_module.is_file():
        raise Usage(f"--twin is not a file: {twin_module}")
    if not (base is None) == (base_toolchain is None) == (base_module is None):
        raise Usage("--base, --base-toolchain and --base-module go together")
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
    if base is not None:
        moved = "" if base_toolchain == toolchain else " (a toolchain change)"
        lines.append(f"- Toolchains: this commit built with Rust `{toolchain}`, the base with its own "
                     f"pinned `{base_toolchain}`{moved}.")
    else:
        lines.append(f"- Toolchain: this commit built with Rust `{toolchain}`.")
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
    state = "are" if pin == built else "are not"
    if base is None:
        lines.append(f"- Release fingerprint not checked: with no base, this run cannot tell whether "
                     f"the change edits the pin or `{NPM_PUBLISH}`'s release identity. The committed "
                     f"pin is `{pin or 'missing'}`; the built bytes {state} the pinned module "
                     f"({RELEASE_DOC}).")
    elif edited:
        lines.append(f"- RELEASE: this change edits {' and '.join(edited)}, so the release "
                     f"fingerprint must describe the built bytes ({RELEASE_DOC}):")
        checked, release_ok = release_checks(root, built)
        lines += checked
        ok = ok and release_ok
    else:
        lines.append(f"- Release fingerprint not checked: this change does not edit the pin or "
                     f"`{NPM_PUBLISH}`'s release identity. The committed pin is "
                     f"`{pin or 'missing'}`; the built bytes {state} the pinned module "
                     f"({RELEASE_DOC}).")
    return lines, ok


# ---------------------------------------------------------------------------------------------
# Self-test: each rule is the red mutation of one fixture, in a scratch repository.


def self_test() -> None:
    scratch = pathlib.Path(tempfile.mkdtemp(prefix="web-audioworklet-identity-"))
    try:
        run_self_test(scratch)
    finally:
        shutil.rmtree(scratch)


# The fixture's commits must not depend on the caller's git configuration (identity, signing).
AUTHOR = ("-c", "user.name=identity-self-test", "-c", "user.email=identity-self-test@invalid",
          "-c", "commit.gpgsign=false")


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
               candidate: str = "1" * 40, extra: str | None = None, toolchain: str = "1.97.1",
               workflow_env: str | None = None) -> str:
        (repo / PIN).parent.mkdir(parents=True, exist_ok=True)
        (repo / NPM_PUBLISH).parent.mkdir(parents=True, exist_ok=True)
        (repo / RESULTS).parent.mkdir(parents=True, exist_ok=True)
        (repo / PIN).write_text(f"{pin}\n")
        if workflow_env is None:
            workflow_env = f"  CARGO_TERM_COLOR: always\n  RUSTUP_TOOLCHAIN: {toolchain}\n"
        (repo / QUALIFICATION).write_text(
            f"name: qualification\non: {{}}\nenv:\n{workflow_env}\njobs:\n  artifact:\n"
            "    env:\n      RUSTUP_TOOLCHAIN: 0.0.0-job-level\n")
        (repo / TOOLCHAIN_FILE).write_text(f'[toolchain]\nchannel = "{toolchain}"\n')
        (repo / NPM_PUBLISH).write_text(
            f'env:\n  PACKAGE_NAME: "@misofm/engine"\n  PACKAGE_VERSION: "{version}"\n'
            f'  EXPECTED_WORKLET_SHA256: "{expected}"\n')
        (repo / RESULTS).write_text(json.dumps(
            {"candidateCommit": candidate, "wasmSha256": recorded}, indent=2) + "\n")
        if extra is not None:
            (repo / "notes.txt").write_text(extra)
        must(git(repo, "add", "-A"))
        must(git(repo, *AUTHOR, "commit", "-q", "--allow-empty", "-m", message))
        return must(git(repo, "rev-parse", "HEAD"))

    must(git(repo, "init", "-q"))
    base = commit("base: release of module a", a, "1.0.0", a, a)
    failures = []

    def case(label: str, built: str, twin: str, base_module: str | None, ok: bool,
             needles: tuple[str, ...], toolchain: str = "1.97.1") -> None:
        try:
            lines, observed = report(repo, built, toolchain, modules[twin][0],
                                     None if base_module is None else base,
                                     None if base_module is None else pinned_toolchain(repo, base)[0],
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
                                          "does not edit the pin", "are the pinned module",
                                          "the base with its own pinned `1.97.1`."))
    case("changed", b, "b", "a", True, ("ARTIFACT CHANGED", f"`{a}` at base", f"-> `{b}`",
                                        "are not the pinned module"))
    case("not reproducible", b, "a", "a", False, ("NOT REPRODUCIBLE", f"produced `{a}`"))
    case("no base", b, "b", None, True, ("ARTIFACT BASE UNAVAILABLE", "with no base",
                                         "Toolchain: this commit built with Rust `1.97.1`"))
    # Finding 1 of attempt 1: a toolchain-only change. The base's toolchain is read from the base,
    # never from HEAD, so the base module is the one the base's CI built and the bytes that moved
    # with the toolchain report CHANGED.
    at(base)
    commit("toolchain 1.98.1, no source change", a, "1.0.0", a, a, toolchain="1.98.1")
    for ref, expected in ((base, "1.97.1"), ("HEAD", "1.98.1")):
        observed, _ = pinned_toolchain(repo, ref)
        if observed != expected:
            failures.append(f"toolchain of {ref}: expected {expected}, got {observed}")
    case("toolchain change", b, "b", "a", True,
         ("ARTIFACT CHANGED", "this commit built with Rust `1.98.1`, the base with its own pinned "
          "`1.97.1` (a toolchain change)"), toolchain="1.98.1")
    # The workflow's own level wins over rust-toolchain.toml; a job-level RUSTUP_TOOLCHAIN never
    # counts; with no workflow-level one the channel is read; with neither, or a malformed name,
    # the step fails rather than guessing.
    at(base)
    fallback = commit("no workflow-level toolchain", a, "1.0.0", a, a, toolchain="1.96.0",
                      workflow_env="  CARGO_TERM_COLOR: always\n")
    observed, source = pinned_toolchain(repo, fallback)
    if (observed, source) != ("1.96.0", f"`{TOOLCHAIN_FILE}`'s channel"):
        failures.append(f"fallback toolchain: got {observed} from {source}")
    at(base)
    commit("no workflow-level toolchain", a, "1.0.0", a, a,
           workflow_env="  CARGO_TERM_COLOR: always\n")
    must(git(repo, "rm", "-q", TOOLCHAIN_FILE))
    must(git(repo, *AUTHOR, "commit", "-q", "-m", "drop rust-toolchain.toml"))
    neither = must(git(repo, "rev-parse", "HEAD"))
    at(base)
    malformed = commit("malformed toolchain", a, "1.0.0", a, a,
                       workflow_env="  RUSTUP_TOOLCHAIN: 1.97.1;rm\n")
    for label, ref in (("no toolchain", neither), ("malformed toolchain", malformed)):
        try:
            pinned_toolchain(repo, ref)
            failures.append(f"{label} was accepted")
        except Usage:
            pass
    # A release change describing module b, and each way it can fail to.
    at(base)
    commit("release b", b, "1.1.0", b, b)
    case("release", b, "b", "a", True, ("ARTIFACT CHANGED", "RELEASE: this change edits",
                                        "the built bytes", "is the pin", "records the three"))
    # Finding 3 of attempt 1: with no base, the report says the release was not checked because
    # there is no base, never that the change does not edit the pin.
    case("release, no base", b, "b", None, True, ("with no base, this run cannot tell",))
    try:
        lines, _ = report(repo, b, "1.97.1", modules["b"][0], None, None, None)
        if any("does not edit" in line for line in lines):
            failures.append("a run with no base claimed the change does not edit the pin")
    except Usage as error:
        failures.append(f"release, no base: usage error {error}")
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
            report(repo, bad, "1.97.1", modules["a"][0], None, None, None)
            failures.append(f"malformed --built {bad!r} was accepted")
        except Usage:
            pass
    for label, arguments in (
        ("base without its toolchain", (base, None, modules["a"][0])),
        ("base toolchain without a base", (None, "1.97.1", None)),
        ("malformed base toolchain", (base, "1.97.1 --x", modules["a"][0])),
    ):
        try:
            report(repo, a, "1.97.1", modules["a"][0], *arguments)
            failures.append(f"{label} was accepted")
        except Usage:
            pass

    # The base: a pull request's merge commit compares against its first parent, a push against
    # the previous tip, and nothing else has a base.
    at(base)
    side = must(git(repo, "rev-parse", ordinary))
    must(git(repo, *AUTHOR, "merge", "-q", "--no-ff", "-m", "merge", side))
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
        (["report", "--built", a, "--toolchain", "1.97.1", "--twin", str(modules["a"][0]),
          "--base", base, "--base-toolchain", "1.97.1", "--base-module", str(modules["a"][0])], 0),
        (["report", "--built", b, "--toolchain", "1.97.1", "--twin", str(modules["a"][0])], 1),
        (["report", "--built", b, "--toolchain", "1.97.1", "--twin",
          str(scratch / "missing.wasm")], 2),
        (["report", "--built", b, "--toolchain", "1.97.1", "--twin", str(modules["b"][0]),
          "--base", base], 2),
        (["toolchain", "--commit", base], 0),
        (["toolchain", "--commit", neither], 2),
    ):
        result = subprocess.run([sys.executable, "-B", script, "--root", str(repo), *args],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                                check=False)
        if result.returncode != status:
            failures.append(f"CLI {args[:3]}: expected exit {status}, got {result.returncode}: "
                            f"{result.stderr}")
        elif args[0] == "toolchain" and status == 0 and result.stdout != "1.97.1\n":
            failures.append(f"CLI toolchain printed {result.stdout!r}, not the base's 1.97.1")

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
    toolchain = commands.add_parser("toolchain")
    toolchain.add_argument("--commit", required=True)
    check = commands.add_parser("report")
    check.add_argument("--built", required=True)
    check.add_argument("--toolchain", required=True)
    check.add_argument("--twin", required=True, type=pathlib.Path)
    check.add_argument("--base")
    check.add_argument("--base-toolchain")
    check.add_argument("--base-module", type=pathlib.Path)
    args = parser.parse_args()

    if args.self_test:
        if args.command is not None:
            parser.error("--self-test takes no command")
        self_test()
        print("web AudioWorklet identity self-test passed: changed, unchanged, no base, "
              "not reproducible, a toolchain-only change, the base toolchain's sources, and six "
              "release mutations")
        return 0
    if args.command == "base":
        commit_id, reason = base_commit(args.root, args.event, args.before)
        print(f"artifact base: {commit_id or 'none'} ({reason})", file=sys.stderr)
        if commit_id is not None:
            print(commit_id)
        return 0
    if args.command == "toolchain":
        try:
            name, source = pinned_toolchain(args.root, args.commit)
        except Usage as error:
            print(f"web-audioworklet-identity: {error}", file=sys.stderr)
            return 2
        print(f"artifact base toolchain: {name} (from {source} at {args.commit})", file=sys.stderr)
        print(name)
        return 0
    if args.command == "report":
        try:
            lines, ok = report(args.root, args.built, args.toolchain, args.twin, args.base or None,
                               args.base_toolchain or None, args.base_module)
        except Usage as error:
            print(f"web-audioworklet-identity: {error}", file=sys.stderr)
            return 2
        print("\n".join(lines))
        if not ok:
            print("web-audioworklet-identity: FAIL (see the report above)", file=sys.stderr)
        return 0 if ok else 1
    parser.error("a command (base, toolchain, report) or --self-test is required")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
