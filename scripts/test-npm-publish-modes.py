#!/usr/bin/env python3
"""Hermetic reachability tests for the fixed npm release workflow (issue #755).

This is deliberately a fixed-format checker, rather than a YAML or Actions interpreter.  It
extracts the release job's known step shapes, accepts only the conditions used by this workflow,
and executes the selected shell blocks against temporary package fixtures.  ``npm`` is a small
command-boundary fixture; the workflow's real ``tar``, ``bash`` and ``node`` programs perform the
archive, checksum, registry and provenance checks.
"""
from __future__ import annotations

import base64
import hashlib
import json
import os
import re
import stat
import subprocess
import tarfile
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Iterable


ROOT = Path(__file__).resolve().parent.parent
WORKFLOW = ROOT / ".github/workflows/npm-publish.yml"
QUALIFICATION = ROOT / ".github/workflows/qualification.yml"

PACKAGE = "@misofm/engine"
VERSION = "0.2.3"
EXPECTED_SHA = "a" * 40

# The workflow was read and hashed before the two authorized edits.  Normalizing precisely those
# edits back out makes the test an invariant audit for all dispatch, publication, registry,
# consumer, attestation, pin and evidence statements in the release workflow.
BASELINE_WORKFLOW_SHA256 = "bf92f4676c176a51736f0ed4fc81a819ca57d337f15abf1f279800fd11e25516"
QUALIFY_PACK_STEP_SHA256 = "da78c535653f2b58e5bacc05465be00964d0a5ac2f1c907d3f57996ec130c9ce"

MODE_ENV = "        env:\n          MODE: ${{ inputs.mode }}\n"
PUBLISH_DRY_RUN = '          npm publish --dry-run --ignore-scripts "$archive"\n'
PUBLISH_GUARD = (
    '          if [[ "$MODE" == publish ]]; then\n'
    + '            npm publish --dry-run --ignore-scripts "$archive"\n'
    + "          fi\n"
)

STEP_HEADERS = (
    "      - name: Refuse non-main dispatches before checkout",
    "      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2",
    "      - uses: actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020 # v4.4.0",
    "      - name: Assert the worklet sha256 pin matches its source-of-truth file",
    "      - name: Guard release identity, accepted ancestry, and rejected commits",
    "      - name: Require a successful qualification dispatch before publication or recovery",
    "      - name: Pin npm 11.19.0 for packing, trusted-publisher checks, and provenance verification",
    "      - name: Install Rust 1.97.1 and the Wasm target",
    "      - name: Install locked SDK dependencies",
    "      - name: Build the shipped Engine AudioWorklet closure once and prove its Linux pin",
    "      - name: Run generated, deletion, type, headless, and package preparation gates",
    "      - name: Pack exactly one archive, smoke it, and record immutable local evidence",
    "      - name: Download the exact prior-qualified archive and evidence",
    "      - name: Re-smoke and checksum the exact prior-qualified archive",
    "      - name: Refuse an existing npm version before publish",
    "      - name: Publish the already-smoked archive through OIDC, or verify an ambiguous outcome without retrying",
    "      - name: Verify registry version, integrity, public access, and latest converge together",
    "      - name: Fresh registry install imports all public entries and runs enginectl",
    "      - name: Upload exact tarball and qualification evidence",
)

STEP_CONDITIONS = {
    STEP_HEADERS[0]: None,
    STEP_HEADERS[1]: None,
    STEP_HEADERS[2]: None,
    STEP_HEADERS[3]: None,
    STEP_HEADERS[4]: None,
    STEP_HEADERS[5]: "inputs.mode != 'qualify'",
    STEP_HEADERS[6]: None,
    STEP_HEADERS[7]: "inputs.mode == 'qualify'",
    STEP_HEADERS[8]: None,
    STEP_HEADERS[9]: "inputs.mode == 'qualify'",
    STEP_HEADERS[10]: "inputs.mode == 'qualify'",
    STEP_HEADERS[11]: "inputs.mode == 'qualify'",
    STEP_HEADERS[12]: "inputs.mode != 'qualify'",
    STEP_HEADERS[13]: "inputs.mode != 'qualify'",
    STEP_HEADERS[14]: "inputs.mode == 'publish'",
    STEP_HEADERS[15]: "inputs.mode == 'publish'",
    STEP_HEADERS[16]: "inputs.mode != 'qualify'",
    STEP_HEADERS[17]: "inputs.mode != 'qualify'",
    STEP_HEADERS[18]: "always()",
}

SHARED_NAME = "Re-smoke and checksum the exact prior-qualified archive"
PACK_NAME = "Pack exactly one archive, smoke it, and record immutable local evidence"
DOWNLOAD_NAME = "Download the exact prior-qualified archive and evidence"
REFUSE_NAME = "Refuse an existing npm version before publish"
PUBLISH_NAME = "Publish the already-smoked archive through OIDC, or verify an ambiguous outcome without retrying"
REGISTRY_NAME = "Verify registry version, integrity, public access, and latest converge together"
CONSUMER_NAME = "Fresh registry install imports all public entries and runs enginectl"

RUN_NAMES = (SHARED_NAME, REFUSE_NAME, PUBLISH_NAME, REGISTRY_NAME, CONSUMER_NAME)


class Invalid(RuntimeError):
    """A workflow contract or focused test assertion failed."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise Invalid(message)


@dataclass(frozen=True)
class Step:
    header: str
    raw: str
    name: str | None
    condition: str | None
    run: str | None


def release_block(text: str) -> str:
    marker = "  release:\n"
    start = text.find(marker)
    require(start >= 0, "npm-publish.yml: missing release job")
    tail = text[start + len(marker):]
    next_job = re.search(r"^  [A-Za-z0-9_-]+:\n", tail, re.MULTILINE)
    return tail[:next_job.start()] if next_job else tail


def _run_body(raw: str, header: str) -> str | None:
    run_fields = re.findall(r"^        run: (.*)$", raw, re.MULTILINE)
    require(len(run_fields) <= 1, f"{header}: duplicate run blocks")
    if run_fields and run_fields[0] != "|":
        require(run_fields[0] not in ("", ">", ">-", ">+", "|-", "|+"),
                f"{header}: unsupported run block shape")
        return run_fields[0] + "\n"
    markers = list(re.finditer(r"^        run: \|\n", raw, re.MULTILINE))
    require(len(markers) <= 1, f"{header}: duplicate run blocks")
    if not markers:
        return None
    body_start = markers[0].end()
    body: list[str] = []
    for line in raw[body_start:].splitlines(keepends=True):
        if line.startswith("          "):
            body.append(line[10:])
        elif line in ("\n", "\r\n"):
            body.append(line)
        else:
            break
    require(body, f"{header}: empty run block")
    return "".join(body)


def extract_steps(text: str) -> list[Step]:
    """Extract only the known six-space step records in the release job."""
    block = release_block(text)
    matches = list(re.finditer(r"^      - (?:name: .+|uses: .+)$", block, re.MULTILINE))
    require(len(matches) == len(STEP_HEADERS),
            f"npm-publish.yml: expected {len(STEP_HEADERS)} release steps, got {len(matches)}")
    steps: list[Step] = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(block)
        raw = block[match.start():end]
        header = match.group(0)
        require(header == STEP_HEADERS[index],
                f"npm-publish.yml: unexpected release step shape at index {index}: {header!r}")
        condition_lines = re.findall(r"^        if: (.+)$", raw, re.MULTILINE)
        require(len(condition_lines) <= 1, f"{header}: duplicate or malformed if condition")
        condition = condition_lines[0] if condition_lines else None
        expected = STEP_CONDITIONS[header]
        require(condition == expected,
                f"{header}: unsupported or drifted if condition {condition!r}; expected {expected!r}")
        name_match = re.fullmatch(r"      - name: (.+)", header)
        name = name_match.group(1) if name_match else None
        steps.append(Step(header, raw, name, condition, _run_body(raw, header)))
    require(len({step.header for step in steps}) == len(steps),
            "npm-publish.yml: release step headers must be unique")
    return steps


def step_map(steps: Iterable[Step]) -> dict[str, Step]:
    result: dict[str, Step] = {}
    for step in steps:
        if step.name is not None:
            require(step.name not in result, f"duplicate named release step: {step.name}")
            result[step.name] = step
    return result


def normalize_authorized_edits(text: str) -> str:
    require(text.count(MODE_ENV) == 1,
            "shared re-smoke step must export MODE exactly once")
    require(text.count(PUBLISH_GUARD) == 1,
            "shared re-smoke step must carry exactly one publish-only dry-run guard")
    return text.replace(MODE_ENV, "", 1).replace(PUBLISH_GUARD, PUBLISH_DRY_RUN, 1)


def check_static_contract(text: str, steps: list[Step]) -> None:
    normalized = normalize_authorized_edits(text)
    actual_hash = hashlib.sha256(normalized.encode()).hexdigest()
    require(actual_hash == BASELINE_WORKFLOW_SHA256,
            "npm-publish.yml changed outside the authorized MODE and publish guard edits")
    named = step_map(steps)
    pack = named.get(PACK_NAME)
    require(pack is not None, "missing qualify-only pack step")
    require(hashlib.sha256(pack.raw.encode()).hexdigest() == QUALIFY_PACK_STEP_SHA256,
            "qualify-only pack step is not byte-equivalent to the approved baseline")
    shared = named.get(SHARED_NAME)
    require(shared is not None and shared.run is not None, "shared re-smoke run block is missing")
    require(shared.raw.count(MODE_ENV) == 1 and shared.raw.count(PUBLISH_GUARD) == 1,
            "shared re-smoke MODE export or publish guard is malformed")
    require(shared.run.index('node sdk/test/package-tarball-smoke.mjs "$unpack/package"')
            < shared.run.index(PUBLISH_GUARD.strip().splitlines()[0]),
            "shared smoke must precede the publish-only dry run")
    require(shared.run.index(PUBLISH_GUARD.strip().splitlines()[-1].strip())
            < shared.run.index("node - target/npm-publish/evidence/tarball-evidence.json"),
            "shared publish guard must precede the checksum validator")
    require(shared.run.count("const actual = {") == 1
            and shared.run.count("shasum:") == 1
            and shared.run.count("sha256:") == 1
            and shared.run.count("integrity:") == 1,
            "shared checksum validator must retain all three archive comparisons")
    require(shared.run.count("npm publish --dry-run --ignore-scripts") == 1,
            "shared step must retain exactly one guarded dry-run command")


def selected(condition: str | None, mode: str) -> bool:
    """The only supported release conditions, intentionally not a general expression evaluator."""
    if condition is None:
        return True
    if condition == "inputs.mode == 'qualify'":
        return mode == "qualify"
    if condition == "inputs.mode != 'qualify'":
        return mode != "qualify"
    if condition == "inputs.mode == 'publish'":
        return mode == "publish"
    if condition == "always()":
        return True
    raise Invalid(f"unsupported release condition: {condition!r}")


def check_mode_matrix(steps: list[Step]) -> None:
    by_name = step_map(steps)
    for mode in ("qualify", "publish", "verify"):
        chosen = {step.name for step in steps if step.name and selected(step.condition, mode)}
        require((SHARED_NAME in chosen) == (mode != "qualify"),
                f"{mode}: shared re-smoke selection drifted")
        require((DOWNLOAD_NAME in chosen) == (mode != "qualify"),
                f"{mode}: prior-artifact download selection drifted")
        require((PUBLISH_NAME in chosen) == (mode == "publish"),
                f"{mode}: real publication selection drifted")
        require((REFUSE_NAME in chosen) == (mode == "publish"),
                f"{mode}: immutable-version guard selection drifted")
    require(PACK_NAME in {step.name for step in steps if step.name and selected(step.condition, "qualify")},
            "qualify: pack step is not selected")
    require("npm publish --dry-run --ignore-scripts" in (by_name[PACK_NAME].run or ""),
            "qualify: pre-existing local inspection dry run disappeared")
    require(DOWNLOAD_NAME not in {step.name for step in steps if step.name and selected(step.condition, "qualify")},
            "qualify: prior artifact download became reachable")
    require("npm publish" not in (by_name[REGISTRY_NAME].run or ""),
            "registry verification must not publish")


def check_shell_syntax(steps: list[Step]) -> None:
    for step in steps:
        if step.run is None:
            continue
        result = subprocess.run(
            ["bash", "-n"], input=step.run, text=True,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
        )
        require(result.returncode == 0,
                f"{step.header}: extracted shell failed bash -n: {result.stderr.strip()}")


def check_qualification_wiring() -> None:
    text = QUALIFICATION.read_text(encoding="utf-8")
    require(text.count("python3 -B scripts/test-npm-publish-modes.py") == 1,
            "qualification.yml: focused npm mode test must be invoked once")
    lint_start = text.find("  lint:\n")
    require(lint_start >= 0, "qualification.yml: missing lint job")
    tail = text[lint_start:]
    next_job = re.search(r"^  [A-Za-z0-9_-]+:\n", tail[len("  lint:\n"):], re.MULTILINE)
    lint = tail[:next_job.start() + len("  lint:\n")] if next_job else tail
    require("      - name: npm publish mode reachability and trust gates\n"
            "        run: python3 -B scripts/test-npm-publish-modes.py\n" in lint,
            "qualification.yml: npm mode test is not the single lint invocation")


def write_executable(path: Path, content: str) -> None:
    path.write_text(content, encoding="utf-8")
    path.chmod(path.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)


FAKE_NPM = r'''#!/usr/bin/env python3
import json
import os
import pathlib
import sys

args = sys.argv[1:]
journal = pathlib.Path(os.environ["NPM_JOURNAL"])
with journal.open("a", encoding="utf-8") as stream:
    stream.write(json.dumps({"args": args}) + "\n")

package = os.environ["PACKAGE_NAME"]
version = os.environ["PACKAGE_VERSION"]
package_version = f"{package}@{version}"

def fail(message, code=1):
    print(message, file=sys.stderr)
    raise SystemExit(code)

def write_consumer_package():
    root = pathlib.Path.cwd() / "node_modules" / "@misofm" / "engine"
    root.mkdir(parents=True, exist_ok=True)
    package_json = {
        "name": package, "version": version, "type": "module",
        "exports": {".": "./index.js", "./headless": "./headless.js", "./browser": "./browser.js", "./assets": "./assets.js"},
        "bin": {"enginectl": "bin/enginectl.mjs"},
    }
    (root / "package.json").write_text(json.dumps(package_json) + "\n", encoding="utf-8")
    source = 'import { appendFileSync } from "node:fs"; appendFileSync(process.env.CONSUMER_IMPORT_MARKER, "import\\n"); export const reached = true;\n'
    for name in ("index.js", "headless.js", "browser.js", "assets.js"):
        (root / name).write_text(source, encoding="utf-8")
    bindir = root / "bin"
    bindir.mkdir(exist_ok=True)
    enginectl = '#!/usr/bin/env node\nimport { appendFileSync } from "node:fs"; appendFileSync(process.env.ENGINECTL_MARKER, "enginectl\\n"); if (process.argv.includes("--version")) console.log("enginectl 0.2.3");\n'
    path = bindir / "enginectl.mjs"
    path.write_text(enginectl, encoding="utf-8")
    path.chmod(0o755)
    link = pathlib.Path.cwd() / "node_modules" / ".bin"
    link.mkdir(exist_ok=True)
    launcher = link / "enginectl"
    launcher.write_text("#!/bin/sh\nexec node \"$(dirname \"$0\")/../@misofm/engine/bin/enginectl.mjs\" \"$@\"\n", encoding="utf-8")
    launcher.chmod(0o755)

if args == ["--version"]:
    print("11.19.0")
    raise SystemExit(0)

if args and args[0] == "publish":
    dry = "--dry-run" in args
    if dry and os.environ.get("NPM_VERSION_STATE") == "existing" and os.environ.get("NPM_DRYRUN_EXISTING") == "fail":
        fail("npm error You cannot publish over the previously published versions")
    if not dry and os.environ.get("NPM_REAL_PUBLISH") == "fail":
        fail("npm error publication result is ambiguous")
    raise SystemExit(0)

if args[:2] == ["view", package_version]:
    if "version" in args:
        state = os.environ.get("NPM_VERSION_STATE")
        if state == "existing":
            print(json.dumps(version))
            raise SystemExit(0)
        if state == "ambiguous":
            fail("temporary registry failure")
        fail("npm error code E404\nnpm error 404 Not Found")
    if os.environ.get("NPM_REGISTRY_STATE") == "mismatch":
        reply = {"version": version, "dist": {"shasum": "0" * 40, "integrity": "sha512-" + "0" * 16}}
    else:
        reply = {"version": version, "dist": {"shasum": os.environ["ARCHIVE_SHA1"], "integrity": os.environ["ARCHIVE_INTEGRITY"]}}
    print(json.dumps(reply))
    raise SystemExit(0)

if args[:3] == ["view", package, "dist-tags.latest"]:
    print(json.dumps("0.9.9" if os.environ.get("NPM_REGISTRY_STATE") == "mismatch" else version))
    raise SystemExit(0)

if args[:4] == ["access", "get", "status", package]:
    print(json.dumps("private" if os.environ.get("NPM_REGISTRY_STATE") == "mismatch" else "public"))
    raise SystemExit(0)

if args and args[0] == "install":
    write_consumer_package()
    print("installed fixture package")
    raise SystemExit(0)

if args[:2] == ["audit", "signatures"]:
    report = pathlib.Path(os.environ["NPM_AUDIT_REPORT"]).read_text(encoding="utf-8")
    print(report, end="")
    pathlib.Path(os.environ["AUDIT_REACHED_MARKER"]).write_text("audit\n", encoding="utf-8")
    raise SystemExit(0)

fail("unexpected fake npm command: " + " ".join(args), 97)
'''

SLEEP = r'''#!/bin/sh
printf '%s\n' "$*" >> "$SLEEP_JOURNAL"
exit 0
'''

SMOKE = '''import { appendFileSync, existsSync, readFileSync } from "node:fs";
const packageDir = process.argv[2];
if (!packageDir || !existsSync(`${packageDir}/package.json`)) throw new Error("fixture smoke package is missing");
const packageJson = JSON.parse(readFileSync(`${packageDir}/package.json`, "utf8"));
if (packageJson.name !== "@misofm/engine" || packageJson.version !== "0.2.3") throw new Error("fixture package identity mismatch");
appendFileSync(process.env.SMOKE_MARKER, "smoke\\n");
'''


@dataclass
class Fixture:
    root: Path
    archive: Path
    evidence: Path
    journal: Path
    smoke_marker: Path
    import_marker: Path
    enginectl_marker: Path
    audit_marker: Path
    sleep_journal: Path
    audit_report: Path
    fakebin: Path

    def environment(
        self,
        *,
        mode: str,
        version_state: str = "missing",
        dryrun_existing: str = "fail",
        registry_state: str = "match",
        real_publish: str = "success",
    ) -> dict[str, str]:
        original_path = os.environ.get("PATH", "")
        env = os.environ.copy()
        env.update({
            "PATH": f"{self.fakebin}{os.pathsep}{original_path}",
            "MODE": mode,
            "PACKAGE_NAME": PACKAGE,
            "PACKAGE_VERSION": VERSION,
            "EXPECTED_SHA": EXPECTED_SHA,
            "GITHUB_WORKSPACE": str(self.root),
            "RUNNER_TEMP": str(self.root / "runner-temp"),
            "NPM_CONFIG_REGISTRY": "http://127.0.0.1:9/issue755-no-network",
            "NPM_JOURNAL": str(self.journal),
            "SLEEP_JOURNAL": str(self.sleep_journal),
            "SMOKE_MARKER": str(self.smoke_marker),
            "CONSUMER_IMPORT_MARKER": str(self.import_marker),
            "ENGINECTL_MARKER": str(self.enginectl_marker),
            "AUDIT_REACHED_MARKER": str(self.audit_marker),
            "NPM_AUDIT_REPORT": str(self.audit_report),
            "NPM_VERSION_STATE": version_state,
            "NPM_DRYRUN_EXISTING": dryrun_existing,
            "NPM_REGISTRY_STATE": registry_state,
            "NPM_REAL_PUBLISH": real_publish,
            "ARCHIVE_SHA1": json.loads((self.evidence / "tarball-evidence.json").read_text())["shasum"],
            "ARCHIVE_INTEGRITY": json.loads((self.evidence / "tarball-evidence.json").read_text())["integrity"],
        })
        (self.root / "runner-temp").mkdir(exist_ok=True)
        return env


def valid_audit_report(sha512: str) -> dict:
    statement = {
        "_type": "https://in-toto.io/Statement/v1",
        "predicateType": "https://slsa.dev/provenance/v1",
        "subject": [{"name": "pkg:npm/%40misofm/engine@0.2.3", "digest": {"sha512": sha512}}],
        "predicate": {
            "buildDefinition": {
                "externalParameters": {"workflow": {
                    "repository": "https://github.com/misofm/engine",
                    "path": ".github/workflows/npm-publish.yml",
                    "ref": "refs/heads/main",
                }},
                "resolvedDependencies": [{"digest": {"gitCommit": EXPECTED_SHA}}],
            }
        },
    }
    payload = base64.b64encode(json.dumps(statement, separators=(",", ":")).encode()).decode()
    return {
        "verified": [{
            "name": PACKAGE,
            "version": VERSION,
            "attestationBundles": [{
                "predicateType": "https://slsa.dev/provenance/v1",
                "bundle": {"dsseEnvelope": {
                    "payloadType": "application/vnd.in-toto+json",
                    "payload": payload,
                }},
            }],
        }],
        "invalid": [],
        "missing": [],
        "unverified": [],
    }


def make_fixture() -> tuple[tempfile.TemporaryDirectory[str], Fixture]:
    temporary = tempfile.TemporaryDirectory(prefix="issue755-attempt1-")
    root = Path(temporary.name)
    evidence = root / "target/npm-publish/evidence"
    evidence.mkdir(parents=True)
    package = root / "package-fixture"
    package.mkdir()
    (package / "package.json").write_text(
        json.dumps({"name": PACKAGE, "version": VERSION}) + "\n", encoding="utf-8"
    )
    (package / "index.js").write_text("fixture\n", encoding="utf-8")
    archive = evidence / "engine-sdk.tgz"
    with tarfile.open(archive, "w:gz") as tar:
        tar.add(package, arcname="package")
    bytes_ = archive.read_bytes()
    sha1 = hashlib.sha1(bytes_).hexdigest()
    sha256 = hashlib.sha256(bytes_).hexdigest()
    sha512 = hashlib.sha512(bytes_).hexdigest()
    integrity = "sha512-" + base64.b64encode(hashlib.sha512(bytes_).digest()).decode()
    (evidence / "tarball-evidence.json").write_text(json.dumps({
        "package": PACKAGE, "version": VERSION, "archive": archive.name, "files": 1,
        "shasum": sha1, "integrity": integrity, "sha256": sha256, "sha512": sha512,
    }) + "\n", encoding="utf-8")
    sdk_test = root / "sdk/test"
    sdk_test.mkdir(parents=True)
    (sdk_test / "package-tarball-smoke.mjs").write_text(SMOKE, encoding="utf-8")
    fakebin = root / "fake-bin"
    fakebin.mkdir()
    write_executable(fakebin / "npm", FAKE_NPM)
    write_executable(fakebin / "sleep", SLEEP)
    journal = root / "npm-journal.jsonl"
    journal.write_text("", encoding="utf-8")
    sleep_journal = root / "sleep-journal.txt"
    sleep_journal.write_text("", encoding="utf-8")
    smoke_marker = root / "smoke-reached.txt"
    import_marker = root / "consumer-import-reached.txt"
    enginectl_marker = root / "enginectl-reached.txt"
    audit_marker = root / "audit-reached.txt"
    audit_report = root / "audit-report.json"
    report = valid_audit_report(sha512)
    audit_report.write_text(json.dumps(report) + "\n", encoding="utf-8")
    return temporary, Fixture(root, archive, evidence, journal, smoke_marker, import_marker,
                             enginectl_marker, audit_marker, sleep_journal, audit_report, fakebin)


def journal(fixture: Fixture) -> list[dict]:
    if not fixture.journal.read_text(encoding="utf-8"):
        return []
    return [json.loads(line) for line in fixture.journal.read_text(encoding="utf-8").splitlines()]


def args_matching(entries: list[dict], prefix: list[str]) -> list[dict]:
    return [entry for entry in entries if entry["args"][:len(prefix)] == prefix]


def run_step(step: Step, fixture: Fixture, env: dict[str, str], *, expect_success: bool = True) -> subprocess.CompletedProcess[str]:
    require(step.run is not None, f"cannot execute non-shell step {step.header}")
    result = subprocess.run(
        ["bash"], input=step.run, cwd=fixture.root, env=env, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
    )
    if expect_success and result.returncode != 0:
        raise Invalid(f"{step.name} failed unexpectedly:\n{result.stdout}{result.stderr}")
    return result


def run_path(steps: list[Step], fixture: Fixture, mode: str, *, version_state: str = "missing",
             dryrun_existing: str = "fail", registry_state: str = "match",
             real_publish: str = "success") -> None:
    by_name = step_map(steps)
    env = fixture.environment(mode=mode, version_state=version_state,
                              dryrun_existing=dryrun_existing, registry_state=registry_state,
                              real_publish=real_publish)
    for name in RUN_NAMES:
        step = by_name[name]
        if not selected(step.condition, mode):
            continue
        run_step(step, fixture, env)


def expect_invalid(label: str, operation: Callable[[], object]) -> None:
    try:
        operation()
    except (Invalid, AssertionError, ValueError):
        return
    raise Invalid(f"{label}: malformed workflow was accepted")


def test_qualify_shape(steps: list[Step]) -> None:
    chosen = {step.name for step in steps if step.name and selected(step.condition, "qualify")}
    require(SHARED_NAME not in chosen and DOWNLOAD_NAME not in chosen,
            "qualify must not select download or shared re-smoke")
    require(PACK_NAME in chosen and PUBLISH_NAME not in chosen,
            "qualify must retain pack and exclude real publication")


def test_publish_missing(steps: list[Step]) -> None:
    temporary, fixture = make_fixture()
    try:
        run_path(steps, fixture, "publish", version_state="missing")
        entries = journal(fixture)
        publish_entries = [entry for entry in entries if entry["args"] and entry["args"][0] == "publish"]
        dry = [entry for entry in publish_entries if "--dry-run" in entry["args"]]
        real = [entry for entry in publish_entries if "--dry-run" not in entry["args"]]
        require(len(dry) == 1 and len(real) == 1,
                f"publish missing-version count: expected one dry run and one real publish, got {publish_entries}")
        dry_index = entries.index(dry[0])
        absence = args_matching(entries, ["view", f"{PACKAGE}@{VERSION}", "version", "--json"])
        require(len(absence) == 1 and entries.index(absence[0]) > dry_index,
                "publish dry run must precede the immutable-version absence query")
        require(fixture.smoke_marker.exists() and fixture.import_marker.exists()
                and fixture.enginectl_marker.exists() and fixture.audit_marker.exists(),
                "publish path did not reach smoke, consumer, enginectl and audit boundaries")
    finally:
        temporary.cleanup()


def test_verify_existing(steps: list[Step]) -> None:
    temporary, fixture = make_fixture()
    try:
        run_path(steps, fixture, "verify", version_state="existing")
        entries = journal(fixture)
        publish_entries = [entry for entry in entries if entry["args"] and entry["args"][0] == "publish"]
        require(not publish_entries, f"verify invoked npm publish: {publish_entries}")
        require(args_matching(entries, ["view", f"{PACKAGE}@{VERSION}", "--json"]),
                "verify did not reach registry version verification")
        require(args_matching(entries, ["access", "get", "status", PACKAGE]),
                "verify did not reach registry access verification")
        require(args_matching(entries, ["view", PACKAGE, "dist-tags.latest", "--json"]),
                "verify did not reach registry latest verification")
        require(fixture.smoke_marker.exists() and fixture.import_marker.exists()
                and fixture.enginectl_marker.exists() and fixture.audit_marker.exists(),
                "verify did not reach smoke, consumer, enginectl and audit boundaries")
    finally:
        temporary.cleanup()


def test_red_restored_unconditional_dry_run(workflow: str, steps: list[Step]) -> None:
    mutated = workflow.replace(PUBLISH_GUARD, PUBLISH_DRY_RUN, 1)
    mutated_steps = extract_steps(mutated)
    temporary, fixture = make_fixture()
    try:
        env = fixture.environment(mode="verify", version_state="existing", dryrun_existing="fail")
        result = run_step(step_map(mutated_steps)[SHARED_NAME], fixture, env, expect_success=False)
        require(result.returncode != 0,
                "red mutation restoring unconditional dry run unexpectedly passed verify")
        entries = journal(fixture)
        require(len([entry for entry in entries if entry["args"][:1] == ["publish"]]) == 1,
                "unconditional dry-run red mutation did not exercise existing-version refusal")
        require(not args_matching(entries, ["view", f"{PACKAGE}@{VERSION}", "--json"]),
                "unconditional dry-run red mutation reached registry verification")
    finally:
        temporary.cleanup()


def test_red_removed_publish_dry_run(workflow: str, steps: list[Step]) -> None:
    mutated = workflow.replace(PUBLISH_GUARD, "          :\n", 1)
    mutated_steps = extract_steps(mutated)
    temporary, fixture = make_fixture()
    try:
        run_path(mutated_steps, fixture, "publish", version_state="missing")
        entries = journal(fixture)
        publish_entries = [entry for entry in entries if entry["args"][:1] == ["publish"]]
        dry = [entry for entry in publish_entries if "--dry-run" in entry["args"]]
        real = [entry for entry in publish_entries if "--dry-run" not in entry["args"]]
        require(len(real) == 1 and len(dry) != 1,
                "removing the publish dry run did not fail the exact publish-count assertion")
    finally:
        temporary.cleanup()


def test_invalid_shapes(workflow: str) -> None:
    unknown_condition = workflow.replace(
        "        if: inputs.mode != 'qualify'\n        env:\n          MODE: ${{ inputs.mode }}\n",
        "        if: inputs.mode == 'recover'\n        env:\n          MODE: ${{ inputs.mode }}\n", 1,
    )
    expect_invalid("unknown mode condition", lambda: extract_steps(unknown_condition))
    unknown_expression = workflow.replace(
        "        if: inputs.mode != 'qualify'\n        env:\n          MODE: ${{ inputs.mode }}\n",
        "        if: inputs.mode != 'qualify' && github.ref == 'refs/heads/main'\n        env:\n          MODE: ${{ inputs.mode }}\n", 1,
    )
    expect_invalid("compound mode condition", lambda: extract_steps(unknown_expression))
    unknown_run = workflow.replace(
        "        run: |\n          set -euo pipefail\n",
        "        run: >\n          set -euo pipefail\n",
        1,
    )
    expect_invalid("folded run block", lambda: extract_steps(unknown_run))
    missing_mode = workflow.replace(MODE_ENV, "", 1)
    expect_invalid("missing MODE export", lambda: check_static_contract(missing_mode, extract_steps(missing_mode)))


def test_corrupt_archive_and_checksum(steps: list[Step]) -> None:
    temporary, fixture = make_fixture()
    try:
        fixture.archive.write_bytes(b"not a gzip archive")
        env = fixture.environment(mode="verify", version_state="existing")
        result = run_step(step_map(steps)[SHARED_NAME], fixture, env, expect_success=False)
        require(result.returncode != 0, "corrupt archive unexpectedly passed")
        require(not args_matching(journal(fixture), ["view", f"{PACKAGE}@{VERSION}", "--json"]),
                "corrupt archive reached registry verification")
    finally:
        temporary.cleanup()

    for field in ("shasum", "sha256", "integrity"):
        temporary, fixture = make_fixture()
        try:
            evidence_path = fixture.evidence / "tarball-evidence.json"
            evidence = json.loads(evidence_path.read_text(encoding="utf-8"))
            evidence[field] = "0" * len(evidence[field])
            evidence_path.write_text(json.dumps(evidence) + "\n", encoding="utf-8")
            env = fixture.environment(mode="verify", version_state="existing")
            result = run_step(step_map(steps)[SHARED_NAME], fixture, env, expect_success=False)
            require(result.returncode != 0, f"corrupt {field} evidence unexpectedly passed")
            require(not args_matching(journal(fixture), ["view", f"{PACKAGE}@{VERSION}", "--json"]),
                    f"corrupt {field} evidence reached registry verification")
        finally:
            temporary.cleanup()


def test_publish_refusals(steps: list[Step]) -> None:
    for state, dryrun, label in (
        ("existing", "fail", "existing version dry run"),
        ("existing", "pass", "existing version guard"),
        ("ambiguous", "pass", "ambiguous absence"),
    ):
        temporary, fixture = make_fixture()
        try:
            by_name = step_map(steps)
            env = fixture.environment(mode="publish", version_state=state, dryrun_existing=dryrun)
            if dryrun == "fail":
                result = run_step(by_name[SHARED_NAME], fixture, env, expect_success=False)
                require(result.returncode != 0, f"{label} did not refuse during publish dry run")
                publish_entries = [entry for entry in journal(fixture) if entry["args"][:1] == ["publish"]]
                require(len(publish_entries) == 1 and "--dry-run" in publish_entries[0]["args"],
                        f"{label} reached or retried real publication")
                continue
            run_step(by_name[SHARED_NAME], fixture, env)
            result = run_step(by_name[REFUSE_NAME], fixture, env, expect_success=False)
            require(result.returncode != 0, f"{label} did not refuse before publication")
            publish_entries = [entry for entry in journal(fixture) if entry["args"][:1] == ["publish"]]
            require(len(publish_entries) == 1 and "--dry-run" in publish_entries[0]["args"],
                    f"{label} reached or retried real publication")
        finally:
            temporary.cleanup()


def test_ambiguous_publish(steps: list[Step]) -> None:
    temporary, fixture = make_fixture()
    try:
        run_path(steps, fixture, "publish", version_state="missing", real_publish="fail")
        publish_entries = [entry for entry in journal(fixture) if entry["args"][:1] == ["publish"]]
        require(len([entry for entry in publish_entries if "--dry-run" in entry["args"]]) == 1
                and len([entry for entry in publish_entries if "--dry-run" not in entry["args"]]) == 1,
                "ambiguous real publication was retried")
        require(fixture.import_marker.exists() and fixture.audit_marker.exists(),
                "ambiguous publication did not proceed to read-only verification")
    finally:
        temporary.cleanup()


def test_registry_mismatch(steps: list[Step]) -> None:
    temporary, fixture = make_fixture()
    try:
        by_name = step_map(steps)
        env = fixture.environment(mode="verify", version_state="existing", registry_state="mismatch")
        run_step(by_name[SHARED_NAME], fixture, env)
        result = run_step(by_name[REGISTRY_NAME], fixture, env, expect_success=False)
        require(result.returncode != 0, "registry mismatch unexpectedly converged")
        version_queries = args_matching(journal(fixture), ["view", f"{PACKAGE}@{VERSION}", "--json"])
        require(len(version_queries) == 12, f"registry mismatch did not exhaust 12 bounded attempts: {len(version_queries)}")
        require(not fixture.import_marker.exists() and not fixture.audit_marker.exists(),
                "registry mismatch reached the consumer or audit boundary")
    finally:
        temporary.cleanup()


def test_provenance_negatives(steps: list[Step]) -> None:
    by_name = step_map(steps)
    for label in ("workflow", "sha", "dependency", "attestation"):
        temporary, fixture = make_fixture()
        try:
            report = json.loads(fixture.audit_report.read_text(encoding="utf-8"))
            if label == "workflow":
                statement_payload = report["verified"][0]["attestationBundles"][0]["bundle"]["dsseEnvelope"]["payload"]
                statement = json.loads(base64.b64decode(statement_payload))
                statement["predicate"]["buildDefinition"]["externalParameters"]["workflow"]["repository"] = "https://example.invalid/untrusted"
                report["verified"][0]["attestationBundles"][0]["bundle"]["dsseEnvelope"]["payload"] = base64.b64encode(
                    json.dumps(statement, separators=(",", ":")).encode()
                ).decode()
            elif label == "sha":
                statement_payload = report["verified"][0]["attestationBundles"][0]["bundle"]["dsseEnvelope"]["payload"]
                statement = json.loads(base64.b64decode(statement_payload))
                statement["subject"][0]["digest"]["sha512"] = "0" * 128
                report["verified"][0]["attestationBundles"][0]["bundle"]["dsseEnvelope"]["payload"] = base64.b64encode(
                    json.dumps(statement, separators=(",", ":")).encode()
                ).decode()
            elif label == "dependency":
                statement_payload = report["verified"][0]["attestationBundles"][0]["bundle"]["dsseEnvelope"]["payload"]
                statement = json.loads(base64.b64decode(statement_payload))
                statement["predicate"]["buildDefinition"]["resolvedDependencies"][0]["digest"]["gitCommit"] = "b" * 40
                report["verified"][0]["attestationBundles"][0]["bundle"]["dsseEnvelope"]["payload"] = base64.b64encode(
                    json.dumps(statement, separators=(",", ":")).encode()
                ).decode()
            else:
                report["verified"][0]["attestationBundles"] = []
            fixture.audit_report.write_text(json.dumps(report) + "\n", encoding="utf-8")
            env = fixture.environment(mode="verify", version_state="existing")
            run_step(by_name[SHARED_NAME], fixture, env)
            run_step(by_name[REGISTRY_NAME], fixture, env)
            result = run_step(by_name[CONSUMER_NAME], fixture, env, expect_success=False)
            require(result.returncode != 0, f"bad provenance {label} unexpectedly passed")
            require(fixture.import_marker.exists(), f"bad provenance {label} skipped fresh package imports")
        finally:
            temporary.cleanup()


def main() -> int:
    workflow = WORKFLOW.read_text(encoding="utf-8")
    steps = extract_steps(workflow)
    check_static_contract(workflow, steps)
    check_mode_matrix(steps)
    check_shell_syntax(steps)
    check_qualification_wiring()
    test_qualify_shape(steps)
    test_publish_missing(steps)
    test_verify_existing(steps)
    test_red_restored_unconditional_dry_run(workflow, steps)
    test_red_removed_publish_dry_run(workflow, steps)
    test_invalid_shapes(workflow)
    test_corrupt_archive_and_checksum(steps)
    test_publish_refusals(steps)
    test_ambiguous_publish(steps)
    test_registry_mismatch(steps)
    test_provenance_negatives(steps)
    print("npm publish mode reachability and trust gates: ok")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (Invalid, AssertionError, OSError, subprocess.SubprocessError) as error:
        print(f"npm publish mode reachability and trust gates: failed: {error}", file=os.sys.stderr)
        raise SystemExit(1)
