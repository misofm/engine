#!/usr/bin/env python3
"""The shipped AudioWorklet module's identity, reported on every change (issue #1061).

Owner decision 5 (`docs/rulings/engine-footprint-2026-09-28.md`): every PR builds the shipped
module and runs every artifact gate against those bytes, and reports whether the module changed;
only a release change must match the recorded fingerprint and carry a fresh three-browser
qualification.

The record: on a push to `main`, `qualification.yml`'s `artifact-record` job (a push-only job that
checks out nothing and is the only one holding a write token) posts the module that commit's own
CI run built as the commit status ``audioworklet-sha256`` on that commit, described
``<sha256> rustc <release>``. It is what later changes compare against, and it does not expire.
This script reads it:

* ``report --event E [--before SHA] --repository R --built D --rustc V --twin MODULE`` (the
  `artifact-identity` job) prints the Markdown report for the job summary. Its headline compares
  this run's module with **the digest its base's own CI run recorded**, never with a rebuild of the
  base: a base rebuilt inside this run's workflow is built in whatever environment the change
  gives it, so a toolchain bump, or one workflow-level cargo variable, moved the module and still
  read UNCHANGED (#1061 attempts 1 and 2). The headline is one of:
  - ``ARTIFACT CHANGED`` or ``ARTIFACT UNCHANGED``, which an issue gate claiming "shipped artifact
    unchanged" cites;
  - ``ARTIFACT CANNOT TELL``, with the reason, when the run has no base (a manual run) or the base
    has no record (its run has not built the module yet, never ran, or ran before records existed).
    A base with no record whose difference from its nearest recorded first-parent ancestor is
    documentation only (the router's evidence class, which no build reads) uses that ancestor's
    record. CANNOT TELL does not fail the job.

  The base is the first parent of GitHub's merge commit for a pull request (the tip of the branch
  the change merges onto, so the line shows only the change's own effect) and ``--before`` for a
  push.

  The report fails (exit 1) when
  - the twin module, built in the same run from another checkout path and ``CARGO_HOME``, differs
    from the bytes the `artifact` job built: the build is not a function of the source alone; or
  - the change is a release change (relative to its base it edits the committed pin, or
    `npm-publish.yml`'s ``PACKAGE_VERSION`` or ``EXPECTED_WORKLET_SHA256``) and the pin,
    ``EXPECTED_WORKLET_SHA256``, `npm-publish.yml`'s ``RUSTUP_TOOLCHAIN`` (against the rustc
    release the `artifact` job built with) or the recorded browser qualification (`results.json`'s
    ``wasmSha256``, with a canonical ``candidateCommit``) does not describe the built bytes.
    `docs/RELEASE.md` is the procedure.

  A failed status lookup (the API, or ``gh`` itself) is an error (exit 2), not CANNOT TELL: it means
  the job is misconfigured, and a CANNOT TELL that never ends would hide that.
* ``--self-test`` proves every rule above in a scratch repository, with a fake ``gh`` on ``PATH``
  that answers from canned API responses and logs its arguments.

A record whose state is not ``success`` is not trusted: the report says CANNOT TELL.

The script builds nothing: it reads git objects, the module it is handed, and commit statuses.
"""
from __future__ import annotations

import argparse
import functools
import hashlib
import importlib.util
import json
import os
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
CONTEXT = "audioworklet-sha256"
SHA256 = re.compile(r"[0-9a-f]{64}")
COMMIT = re.compile(r"[0-9a-f]{40}")
RUSTC_RELEASE = re.compile(r"[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9.]+)?")
REPOSITORY = re.compile(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+")
RECORD = re.compile(r"([0-9a-f]{64}) rustc (\S+)")
RELEASE_LINE = re.compile(r'^  (PACKAGE_VERSION|EXPECTED_WORKLET_SHA256): ".*"$', re.MULTILINE)
EXPECTED_WORKLET = re.compile(r'^  EXPECTED_WORKLET_SHA256: "([^"]*)"$', re.MULTILINE)
NPM_TOOLCHAIN = re.compile(r'^  RUSTUP_TOOLCHAIN: "([^"]*)"$', re.MULTILINE)
# How far back a base with no record may look for a recorded ancestor it differs from only in
# documentation. A longer documentation-only stretch of `main` reports CANNOT TELL.
ANCESTOR_LIMIT = 20


class Usage(RuntimeError):
    pass


class LookupFailed(RuntimeError):
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


# ---------------------------------------------------------------------------------------------
# The record: a commit status on `main`'s commits, posted by that commit's own CI run
# (`qualification.yml`'s `artifact-record` job).


def gh_api(*args: str) -> str:
    try:
        result = subprocess.run(["gh", "api", *args], stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE, text=True, check=False)
    except OSError as error:
        raise LookupFailed(f"gh could not run: {error}") from error
    if result.returncode != 0:
        raise LookupFailed(f"gh api {' '.join(args)} exited {result.returncode}: "
                           f"{result.stderr.strip()}")
    return result.stdout


def recorded(repository: str, commit: str) -> tuple[dict | None, str]:
    """The record `commit`'s own CI run posted, or None and why there is none."""
    text = gh_api(f"repos/{repository}/commits/{commit}/status")
    try:
        combined = json.loads(text)
    except json.JSONDecodeError as error:
        raise LookupFailed(f"the status API returned no JSON for {commit}: {error}") from error
    statuses = combined.get("statuses") if isinstance(combined, dict) else None
    if not isinstance(statuses, list):
        raise LookupFailed(f"the status API returned no statuses list for {commit}")
    ours = [row for row in statuses if isinstance(row, dict) and row.get("context") == CONTEXT]
    if not ours:
        return None, f"`{commit}` has no `{CONTEXT}` status"
    # The combined status holds the latest status per context; a rerun re-posts the same bytes.
    row = ours[0]
    if row.get("state") != "success":
        return None, (f"`{commit}`'s `{CONTEXT}` status is in state `{row.get('state')}`, not "
                      "`success`, so it is not trusted")
    match = RECORD.fullmatch(str(row.get("description", "")))
    if match is None:
        return None, f"`{commit}`'s `{CONTEXT}` status is malformed: {row.get('description')!r}"
    return {"commit": commit, "sha256": match.group(1), "rustc": match.group(2),
            "url": row.get("target_url") or ""}, ""


@functools.cache
def router():
    spec = importlib.util.spec_from_file_location("ci_path_router",
                                                  ROOT / "scripts/ci-path-router.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def documentation_only(root: pathlib.Path, older: str, newer: str) -> bool:
    """Whether every path that differs between the two commits is one no build reads: the router's
    own evidence class, the class whose changes build no module at all."""
    result = git(root, "diff", "--name-only", "-z", older, newer)
    if result.returncode != 0:
        return False
    paths = [path for path in result.stdout.split("\0") if path]
    return not paths or router().classify_paths(paths) == "evidence"


def base_record(root: pathlib.Path, repository: str, base: str) -> tuple[dict | None, str]:
    """The record for `base`, or for its nearest recorded first-parent ancestor when the two differ
    only in documentation, or None and the reason."""
    record, reason = recorded(repository, base)
    if record is not None:
        return record, ""
    candidate = base
    for _ in range(ANCESTOR_LIMIT):
        parent = git(root, "rev-parse", "--verify", "-q", f"{candidate}^1").stdout.strip()
        if not parent or not documentation_only(root, parent, base):
            break
        found, _ = recorded(repository, parent)
        if found is not None:
            found["via"] = base
            return found, ""
        candidate = parent
    return None, (f"{reason}: its qualification run has not built the module yet, never ran, or "
                  "ran before records existed; rerun this job once the base's `artifact` job has "
                  "finished")


# ---------------------------------------------------------------------------------------------
# The release fingerprint.


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


def release_checks(root: pathlib.Path, built: str, rustc: str) -> tuple[list[str], bool]:
    lines, ok = [], True
    pin = (show(root, "HEAD", PIN) or "").strip()
    if pin == built:
        lines.append(f"  - the pin `{PIN}` is `{pin}`: the built bytes")
    else:
        ok = False
        lines.append(f"  - **FAIL** the pin `{PIN}` is `{pin or 'missing'}`, not the built `{built}`")
    publish = show(root, "HEAD", NPM_PUBLISH) or ""
    match = EXPECTED_WORKLET.search(publish)
    expected = match.group(1) if match else None
    if expected == pin:
        lines.append(f"  - `{NPM_PUBLISH}`'s `EXPECTED_WORKLET_SHA256` is the pin")
    else:
        ok = False
        lines.append(f"  - **FAIL** `{NPM_PUBLISH}`'s `EXPECTED_WORKLET_SHA256` is "
                     f"`{expected or 'missing'}`, not the pin `{pin or 'missing'}`")
    # `npm-publish.yml` rebuilds the module and refuses any other bytes, so any difference in its
    # build environment can only fail `qualify`, never publish unreviewed bytes. The toolchain is
    # the difference a release is most likely to carry, so it is caught here, in the release PR.
    match = NPM_TOOLCHAIN.search(publish)
    toolchain = match.group(1) if match else None
    if toolchain == rustc:
        lines.append(f"  - `{NPM_PUBLISH}` builds with `RUSTUP_TOOLCHAIN` `{toolchain}`, the rustc "
                     "the `artifact` job built with")
    else:
        ok = False
        lines.append(f"  - **FAIL** `{NPM_PUBLISH}`'s `RUSTUP_TOOLCHAIN` is "
                     f"`{toolchain or 'missing'}`, but the `artifact` job built with rustc "
                     f"`{rustc}`: `qualify` would build other bytes")
    try:
        results = json.loads(show(root, "HEAD", RESULTS) or "null")
    except json.JSONDecodeError:
        results = None
    results = results if isinstance(results, dict) else {}
    qualified = results.get("wasmSha256")
    candidate = results.get("candidateCommit")
    if qualified == built:
        lines.append(f"  - `{RESULTS}` records the three-browser qualification of the built bytes")
    else:
        ok = False
        lines.append(f"  - **FAIL** artifact-lineage: `{RESULTS}` records a qualification of "
                     f"`{qualified}`, not of the built `{built}`: re-record it "
                     "(`npm run qualify -- --browser all --record-matrix`)")
    if isinstance(candidate, str) and COMMIT.fullmatch(candidate):
        lines.append(f"  - its `candidateCommit` is `{candidate}`")
    else:
        ok = False
        lines.append(f"  - **FAIL** candidate-lineage: `{RESULTS}`'s `candidateCommit` "
                     f"`{candidate}` is not a canonical lowercase 40-hex commit")
    return lines, ok


# ---------------------------------------------------------------------------------------------
# The report.


def report(root: pathlib.Path, event: str, before: str, repository: str, built: str, rustc: str,
           twin_module: pathlib.Path) -> tuple[list[str], bool]:
    if SHA256.fullmatch(built) is None:
        raise Usage(f"--built is not a lowercase sha256: {built!r}")
    if RUSTC_RELEASE.fullmatch(rustc) is None:
        raise Usage(f"--rustc is not a rustc release: {rustc!r}")
    if REPOSITORY.fullmatch(repository) is None:
        raise Usage(f"--repository is not owner/name: {repository!r}")
    if not twin_module.is_file():
        raise Usage(f"--twin is not a file: {twin_module}")
    head = git(root, "rev-parse", "HEAD").stdout.strip()
    base, why = base_commit(root, event, before)
    if base is None:
        record, reason = None, f"there is no base: {why}"
    else:
        record, reason = base_record(root, repository, base)
    ok = True
    lines = ["### Shipped AudioWorklet artifact", ""]
    if record is None:
        lines.append(f"ARTIFACT CANNOT TELL: `{built}` at `{head}`; {reason}")
    else:
        against = (f"the digest base `{base}`'s own CI run recorded" if "via" not in record else
                   f"the digest `{record['commit']}`'s own CI run recorded (base `{base}` differs "
                   "from it only in documentation)")
        if record["sha256"] == built:
            lines.append(f"ARTIFACT UNCHANGED: `{built}` at `{head}` is {against}")
        else:
            lines.append(f"ARTIFACT CHANGED: `{record['sha256']}` -> `{built}` at `{head}`, "
                         f"against {against}")
    lines += ["", f"- Every artifact gate in this run read `{built}`: the `artifact` job built it, "
              "and each job that reads it checks its download against that digest."]
    if record is None:
        lines.append(f"- Built with rustc `{rustc}`.")
    else:
        moved = "" if record["rustc"] == rustc else " (a toolchain change)"
        lines.append(f"- Built with rustc `{rustc}`; the recorded base module with rustc "
                     f"`{record['rustc']}`{moved}. Record: {record['url'] or 'no run link'}.")
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
    pin = (show(root, "HEAD", PIN) or "").strip()
    state = "are" if pin == built else "are not"
    edited = release_change(root, base) if base is not None else []
    if base is None:
        lines.append(f"- Release fingerprint not checked: with no base, this run cannot tell whether "
                     f"the change edits the pin or `{NPM_PUBLISH}`'s release identity. The committed "
                     f"pin is `{pin or 'missing'}`; the built bytes {state} the pinned module "
                     f"({RELEASE_DOC}).")
    elif edited:
        lines.append(f"- RELEASE: this change edits {' and '.join(edited)}, so the release "
                     f"fingerprint must describe the built bytes ({RELEASE_DOC}):")
        checked, release_ok = release_checks(root, built, rustc)
        lines += checked
        ok = ok and release_ok
    else:
        lines.append(f"- Release fingerprint not checked: this change does not edit the pin or "
                     f"`{NPM_PUBLISH}`'s release identity. The committed pin is "
                     f"`{pin or 'missing'}`; the built bytes {state} the pinned module "
                     f"({RELEASE_DOC}).")
    return lines, ok


# ---------------------------------------------------------------------------------------------
# Self-test: each rule is the red mutation of one fixture, in a scratch repository, with a fake
# `gh` that answers `gh api` from a JSON table and logs every call.

FAKE_GH = r'''#!/usr/bin/env python3
import json, os, sys
table = json.load(open(os.environ["FAKE_GH_TABLE"]))
with open(os.environ["FAKE_GH_LOG"], "a") as log:
    log.write(json.dumps(sys.argv[1:]) + "\n")
if sys.argv[1:2] != ["api"]:
    sys.exit(2)
args = sys.argv[2:]
if args[:2] == ["-X", "POST"]:
    sys.exit(3)
answer = table.get(args[0])
if answer is None:
    sys.stderr.write("HTTP 403: Resource not accessible by integration\n")
    sys.exit(1)
sys.stdout.write(answer if isinstance(answer, str) else json.dumps(answer))
'''

# The fixture's commits must not depend on the caller's git configuration (identity, signing).
AUTHOR = ("-c", "user.name=identity-self-test", "-c", "user.email=identity-self-test@invalid",
          "-c", "commit.gpgsign=false")
REPO_NAME = "owner/engine"


def self_test() -> None:
    scratch = pathlib.Path(tempfile.mkdtemp(prefix="web-audioworklet-identity-"))
    saved = {key: os.environ.get(key) for key in ("PATH", "FAKE_GH_TABLE", "FAKE_GH_LOG")}
    try:
        run_self_test(scratch)
    finally:
        for key, value in saved.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value
        shutil.rmtree(scratch)


def run_self_test(scratch: pathlib.Path) -> None:
    repo = scratch / "repo"
    repo.mkdir()
    bin_dir = scratch / "bin"
    bin_dir.mkdir()
    (bin_dir / "gh").write_text(FAKE_GH)
    (bin_dir / "gh").chmod(0o755)
    table_path, log_path = scratch / "table.json", scratch / "gh.log"
    os.environ.update({"PATH": f"{bin_dir}{os.pathsep}{os.environ.get('PATH', '')}",
                       "FAKE_GH_TABLE": str(table_path), "FAKE_GH_LOG": str(log_path)})
    modules = {}
    for name in ("a", "b", "c"):
        path = scratch / f"{name}.wasm"
        path.write_bytes(f"module {name}\n".encode())
        modules[name] = (path, digest(path))
    a, b, c = (modules[name][1] for name in "abc")
    failures: list[str] = []

    def must(result: subprocess.CompletedProcess[str]) -> str:
        if result.returncode != 0:
            raise AssertionError(f"fixture git failed: {result.stderr}")
        return result.stdout.strip()

    def commit(message: str, pin: str = a, version: str = "1.0.0", expected: str = a,
               qualified: str = a, candidate: str = "1" * 40, path: str = "src/lib.rs",
               toolchain: str = "1.97.1") -> str:
        for name in (PIN, NPM_PUBLISH, RESULTS, path):
            (repo / name).parent.mkdir(parents=True, exist_ok=True)
        (repo / PIN).write_text(f"{pin}\n")
        (repo / NPM_PUBLISH).write_text(
            f'env:\n  PACKAGE_NAME: "@misofm/engine"\n  PACKAGE_VERSION: "{version}"\n'
            f'  EXPECTED_WORKLET_SHA256: "{expected}"\n  RUSTUP_TOOLCHAIN: "{toolchain}"\n')
        (repo / RESULTS).write_text(json.dumps(
            {"candidateCommit": candidate, "wasmSha256": qualified}, indent=2) + "\n")
        with (repo / path).open("a") as handle:
            handle.write(f"{message}\n")
        must(git(repo, "add", "-A"))
        must(git(repo, *AUTHOR, "commit", "-q", "-m", message))
        return must(git(repo, "rev-parse", "HEAD"))

    def at(ref: str) -> None:
        must(git(repo, "checkout", "-q", ref))

    def merge(onto: str, change: str) -> str:
        """A pull request's merge commit: first parent `onto`, second parent `change`."""
        at(onto)
        must(git(repo, *AUTHOR, "merge", "-q", "--no-ff", "-m", "merge", change))
        return must(git(repo, "rev-parse", "HEAD"))

    def statuses(records: dict[str, str | None], state: str = "success") -> None:
        """Serve each commit's combined status: our record's description, or none at all."""
        table = {}
        for commit_id, description in records.items():
            rows = [{"context": "ci/other", "state": "success", "description": "not ours"}]
            if description is not None:
                rows.append({"context": CONTEXT, "state": state, "description": description,
                             "target_url": f"https://example.invalid/runs/{commit_id[:7]}"})
            table[f"repos/{REPO_NAME}/commits/{commit_id}/status"] = {"statuses": rows}
        table_path.write_text(json.dumps(table))

    def case(label: str, ok: bool, needles: tuple[str, ...], event: str = "pull_request",
             before: str = "", built: str = b, twin: str = "b", rustc: str = "1.97.1",
             absent: tuple[str, ...] = ()) -> None:
        try:
            lines, observed = report(repo, event, before, REPO_NAME, built, rustc, modules[twin][0])
        except (Usage, LookupFailed) as error:
            failures.append(f"{label}: {type(error).__name__} {error}")
            return
        text = "\n".join(lines)
        if observed != ok:
            failures.append(f"{label}: expected ok={ok}, got ok={observed}\n{text}")
        for needle in needles:
            if needle not in text:
                failures.append(f"{label}: {needle!r} missing from\n{text}")
        for needle in absent:
            if needle in text:
                failures.append(f"{label}: {needle!r} present in\n{text}")

    # base (recorded) <- docs (documentation only, no record) <- change; and base <- code (a source
    # change, no record) <- another change.
    must(git(repo, "init", "-q"))
    base = commit("base: module a, released")
    docs = commit("docs only", path="docs/note.md")
    change = commit("a change")
    pr = merge(base, change)
    record_a = f"{a} rustc 1.97.1"
    # The base's own record decides: matching, different, missing, malformed, refused.
    statuses({base: record_a})
    case("unchanged", True, ("ARTIFACT UNCHANGED", f"is the digest base `{base}`'s own CI run",
                             "Reproducible", "does not edit the pin"), built=a, twin="a")
    case("changed", True, ("ARTIFACT CHANGED", f"`{a}` -> `{b}`", "are not the pinned module"))
    case("not reproducible", False, ("NOT REPRODUCIBLE", f"produced `{a}`"), twin="a")
    # Attempts 1 and 2: whatever the change does to the build environment, the base's digest is the
    # one the base's own run recorded, so a toolchain-only change reports CHANGED.
    case("toolchain change", True, ("ARTIFACT CHANGED", "rustc `1.97.1` (a toolchain change)"),
         rustc="1.98.1")
    statuses({base: None})
    case("missing record", True, ("ARTIFACT CANNOT TELL", f"`{base}` has no `{CONTEXT}` status",
                                  "rerun this job"), absent=("UNCHANGED", "ARTIFACT CHANGED"))
    statuses({base: "deadbeef rustc 1.97.1"})
    case("malformed record", True, ("ARTIFACT CANNOT TELL", "is malformed"))
    # A well-formed record in any state but success (a forged `failure`, a `pending`) is not
    # trusted, even when its digest is the built one.
    for state in ("failure", "error", "pending"):
        statuses({base: f"{b} rustc 1.97.1"}, state=state)
        case(f"{state} record", True, ("ARTIFACT CANNOT TELL", f"in state `{state}`", "not trusted"),
             absent=("UNCHANGED", "ARTIFACT CHANGED"))
    table_path.write_text("{}")
    try:
        report(repo, "pull_request", "", REPO_NAME, b, "1.97.1", modules["b"][0])
        failures.append("a refused status lookup reported instead of failing")
    except LookupFailed:
        pass
    # A base with no record uses its nearest recorded ancestor across documentation only.
    merge(docs, change)
    statuses({base: record_a, docs: None})
    case("documentation-only base", True,
         ("ARTIFACT CHANGED", f"`{base}`'s own CI run recorded (base `{docs}` differs",
          "only in documentation"))
    at(base)
    code = commit("code on main", path="src/main.rs")
    merge(code, commit("another change"))
    statuses({base: record_a, code: None})
    case("source between base and record", True, ("ARTIFACT CANNOT TELL", f"`{code}` has no"))
    # A push compares with the previous tip; a manual run has no base.
    at(pr)
    statuses({base: record_a})
    case("push", True, ("ARTIFACT CHANGED",), event="push", before=base)
    case("manual run", True, ("ARTIFACT CANNOT TELL", "there is no base",
                              "with no base, this run cannot tell"),
         event="workflow_dispatch", absent=("does not edit",))
    # Release changes: the fingerprint must describe the built bytes.
    at(base)
    release_pr = merge(base, commit("release b", pin=b, version="1.1.0", expected=b, qualified=b))
    case("release", True, ("RELEASE: this change edits", "the built bytes", "is the pin",
                           "`RUSTUP_TOOLCHAIN` `1.97.1`", "records the three"))
    case("release built other bytes", False, ("**FAIL** the pin", "**FAIL** artifact-lineage"),
         built=c, twin="c")
    case("release, npm-publish toolchain differs", False,
         ("**FAIL** `.github/workflows/npm-publish.yml`'s `RUSTUP_TOOLCHAIN` is `1.97.1`",),
         rustc="1.98.1")
    case("release, manual run", True, ("with no base, this run cannot tell",),
         event="workflow_dispatch", absent=("does not edit", "RELEASE:"))
    for label, fields, needle in (
        ("release without a re-pin", dict(version="1.1.0"), "**FAIL** the pin"),
        ("release, EXPECTED_WORKLET_SHA256 stale", dict(pin=b, qualified=b),
         "**FAIL** `.github/workflows/npm-publish.yml`'s `EXPECTED_WORKLET_SHA256`"),
        ("release, results.json stale", dict(pin=b, version="1.1.0", expected=b),
         "**FAIL** artifact-lineage"),
        ("release, candidate malformed", dict(pin=b, version="1.1.0", expected=b, qualified=b,
                                              candidate="0" * 39), "**FAIL** candidate-lineage"),
    ):
        at(base)
        merge(base, commit(label, **fields))
        case(label, False, (needle,))
    for bad in ("A" * 64, "0" * 63, ""):
        try:
            report(repo, "pull_request", "", REPO_NAME, bad, "1.97.1", modules["a"][0])
            failures.append(f"malformed --built {bad!r} was accepted")
        except Usage:
            pass
    for bad in ("1.97", "stable", "1.97.1; rm"):
        try:
            report(repo, "pull_request", "", REPO_NAME, a, bad, modules["a"][0])
            failures.append(f"malformed --rustc {bad!r} was accepted")
        except Usage:
            pass

    # The base: a pull request's merge commit compares against its first parent, a push against
    # the previous tip, and nothing else has a base.
    for event, before, head_ref, expected in (
        ("pull_request", "", pr, base), ("pull_request", "", release_pr, base),
        ("push", base, pr, base), ("push", "0" * 40, pr, None), ("push", "f" * 40, pr, None),
        ("push", "", pr, None), ("workflow_dispatch", base, pr, None),
        ("pull_request", "", change, None),
    ):
        at(head_ref)
        observed, reason = base_commit(repo, event, before)
        if observed != expected:
            failures.append(f"base {event} {before!r} at {head_ref[:7]}: expected {expected}, "
                            f"got {observed} ({reason})")

    # The command line, through the fake gh: `report` exits 0, 1 on a failed rule, 2 on a usage
    # error or a refused lookup, and looks up only the base.
    at(pr)
    statuses({base: record_a})
    script = str(pathlib.Path(__file__).resolve())
    common = ["--event", "pull_request", "--repository", REPO_NAME, "--rustc", "1.97.1"]
    log_path.write_text("")

    def cli(args: list[str]) -> subprocess.CompletedProcess[str]:
        return subprocess.run([sys.executable, "-B", script, "--root", str(repo), *args],
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                              check=False)

    for args, status in (
        (["report", *common, "--built", a, "--twin", str(modules["a"][0])], 0),
        (["report", *common, "--built", b, "--twin", str(modules["a"][0])], 1),
        (["report", *common, "--built", b, "--twin", str(scratch / "missing.wasm")], 2),
    ):
        result = cli(args)
        if result.returncode != status:
            failures.append(f"CLI {args[:2]}: expected exit {status}, got {result.returncode}: "
                            f"{result.stderr}")
    calls = [json.loads(line) for line in log_path.read_text().splitlines()]
    lookup = ["api", f"repos/{REPO_NAME}/commits/{base}/status"]
    if calls != [lookup, lookup]:
        failures.append(f"gh calls were {calls}, not two lookups of the base")
    table_path.write_text("{}")
    result = cli(["report", *common, "--built", a, "--twin", str(modules["a"][0])])
    if result.returncode != 2 or "HTTP 403" not in result.stderr:
        failures.append(f"a refused lookup exited {result.returncode}: {result.stderr}")
    if failures:
        raise AssertionError("\n".join(failures))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--root", type=pathlib.Path, default=ROOT)
    parser.add_argument("--self-test", action="store_true")
    commands = parser.add_subparsers(dest="command")
    check = commands.add_parser("report")
    check.add_argument("--event", required=True)
    check.add_argument("--before", default="")
    check.add_argument("--repository", required=True)
    check.add_argument("--built", required=True)
    check.add_argument("--rustc", required=True)
    check.add_argument("--twin", required=True, type=pathlib.Path)
    args = parser.parse_args()

    if args.self_test:
        if args.command is not None:
            parser.error("--self-test takes no command")
        self_test()
        print("web AudioWorklet identity self-test passed: the base's recorded digest (matching, "
              "different, missing, malformed, not success, refused, across documentation), "
              "reproducibility, a toolchain change, and seven release mutations")
        return 0
    try:
        if args.command == "report":
            lines, ok = report(args.root, args.event, args.before, args.repository, args.built,
                               args.rustc, args.twin)
            print("\n".join(lines))
            if not ok:
                print("web-audioworklet-identity: FAIL (see the report above)", file=sys.stderr)
            return 0 if ok else 1
    except (Usage, LookupFailed) as error:
        print(f"web-audioworklet-identity: {error}", file=sys.stderr)
        return 2
    parser.error("a command (report) or --self-test is required")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
