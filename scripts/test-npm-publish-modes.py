#!/usr/bin/env python3
"""Hermetic tests for the one-dispatch release workflow, `.github/workflows/npm-publish.yml`.

A fixed-format reader, not a YAML or Actions interpreter: it finds the workflow's known jobs and
steps by name, runs their shell blocks against temporary fixtures with a fake `npm` on `PATH`
(the real `bash`, `node`, `sha256sum` and `tar` do the rest), and checks the permission layout.
It proves the rules a dry run on the runner cannot: which version a dispatch resolves or refuses,
that a dry run or a non-main ref never publishes, that a token fallback or a swapped tarball is
refused, and that the provenance check binds the run's commit and this workflow.
"""
from __future__ import annotations

import base64
import hashlib
import json
import os
import re
import subprocess
import tempfile
import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKFLOW = ROOT / ".github/workflows/npm-publish.yml"
TEXT = WORKFLOW.read_text(encoding="utf-8")
PACKAGE = "@misofm/engine"
SHA = "a" * 40

FAKE_NPM = r'''#!/usr/bin/env bash
printf '%s\n' "$*" >> "$FAKE_NPM_LOG"
case "$1 $2" in
  "view @misofm/engine") ;;
  *) exit 0 ;;
esac
case "$3" in
  versions) printf '%s\n' "$FAKE_VERSIONS" ;;
  dist-tags.latest) printf '%s\n' "$FAKE_LATEST" ;;
esac
'''


class Failure(AssertionError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise Failure(message)


def job(name: str) -> str:
    jobs = TEXT.split("\njobs:\n", 1)[1]
    match = re.search(rf"^  {re.escape(name)}:\n", jobs, re.MULTILINE)
    require(match is not None, f"missing job {name}")
    tail = jobs[match.end():]
    following = re.search(r"^  [A-Za-z0-9_-]+:\n", tail, re.MULTILINE)
    return tail[:following.start() if following else len(tail)]


def jobs() -> list[str]:
    block = TEXT.split("\njobs:\n", 1)[1]
    return re.findall(r"^  ([A-Za-z0-9_-]+):\n", block, re.MULTILINE)


def step_script(job_name: str, marker: str) -> str:
    """The `run: |` block of the step in `job_name` whose header line contains `marker`."""
    block = job(job_name)
    steps = re.split(r"^      - ", block, flags=re.MULTILINE)
    found = [step for step in steps if marker in step.split("\n", 1)[0] or f"name: {marker}" in step]
    require(len(found) == 1, f"{job_name}: expected one step marked {marker!r}, found {len(found)}")
    match = re.search(r"^        run: \|\n((?:(?: {10}.*)?\n)+)", found[0], re.MULTILINE)
    require(match is not None, f"{job_name}: step {marker!r} has no run block")
    return textwrap.dedent(match.group(1))


def heredoc(script: str, opener: str) -> str:
    start = script.index(opener) + len(opener) + 1
    return script[start:script.index("\nNODE\n", start) + 1]


class Sandbox:
    def __init__(self) -> None:
        self.dir = Path(tempfile.mkdtemp(prefix="npm-publish-modes-"))
        (self.dir / "bin").mkdir()
        npm = self.dir / "bin/npm"
        npm.write_text(FAKE_NPM, encoding="utf-8")
        npm.chmod(0o755)
        self.log = self.dir / "npm.log"
        self.output = self.dir / "output"
        self.log.touch()
        self.output.touch()

    def run(self, script: str, **env: str) -> subprocess.CompletedProcess[str]:
        environment = {
            "PATH": f"{self.dir / 'bin'}:{os.environ['PATH']}",
            "HOME": str(self.dir),
            "FAKE_NPM_LOG": str(self.log),
            "GITHUB_OUTPUT": str(self.output),
            "GITHUB_STEP_SUMMARY": str(self.dir / "summary"),
            "RUNNER_TEMP": str(self.dir),
            "GITHUB_SHA": SHA,
            "GITHUB_REF": "refs/heads/main",
            "PACKAGE_NAME": PACKAGE,
            **env,
        }
        return subprocess.run(["bash", "-e", "-c", script], cwd=self.dir, env=environment,
                              capture_output=True, text=True, check=False)

    def npm_calls(self) -> list[str]:
        return self.log.read_text(encoding="utf-8").splitlines()

    def outputs(self) -> dict[str, str]:
        lines = self.output.read_text(encoding="utf-8").splitlines()
        return dict(line.split("=", 1) for line in lines if "=" in line)


def check_layout() -> None:
    on = TEXT.split("\non:\n", 1)[1].split("\npermissions:\n", 1)[0]
    require(re.fullmatch(r"  workflow_dispatch:\n(    .*\n|\n)*", on + "\n") is not None
            and "push:" not in on and "pull_request:" not in on and "schedule:" not in on,
            "on: must be workflow_dispatch only")
    require("\npermissions:\n  contents: read\n\n" in TEXT,
            "top-level permissions must be exactly contents: read")
    for name in jobs():
        block = job(name)
        require("timeout-minutes:" in block, f"{name} has no timeout")
        holds_oidc = "id-token: write" in block
        holds_write = "contents: write" in block
        require(holds_oidc == (name == "publish"), f"{name}: only publish may hold id-token: write")
        require(holds_write == (name == "record"), f"{name}: only record may hold contents: write")
    require("actions/checkout" not in job("record") and "actions/checkout" not in job("publish"),
            "publish and record must check out and run no repository code")
    needs = re.search(r"^    needs: \[(.*)\]$", job("publish"), re.MULTILINE)
    require(needs is not None and set(needs.group(1).split(", ")) == {
        "prepare", "artifact", "core-tests", "package", "artifact-gates", "browser"},
            "publish must wait for every check")
    require("    if: ${{ !inputs.dry_run }}\n" in job("record"), "a dry run must record nothing")
    require(re.search(r"\b(NPM_TOKEN|NODE_AUTH_TOKEN)\b\s*:", TEXT) is None
            and "secrets." not in TEXT, "the workflow must read no npm token or secret")


def check_ref_guard() -> None:
    script = step_script("prepare", "Refuse a real release from anywhere but main")
    box = Sandbox()
    require(box.run(script, DRY_RUN="false").returncode == 0, "a release on main is refused")
    require(box.run(script, DRY_RUN="true", GITHUB_REF="refs/heads/topic").returncode == 0,
            "a dry run on a branch is refused")
    require(box.run(script, DRY_RUN="false", GITHUB_REF="refs/heads/topic").returncode != 0,
            "a real release from a branch is accepted")


def check_version_resolution() -> None:
    script = step_script("prepare", "id: version")
    published = json.dumps(["0.10.3", "0.10.4", "0.10.5", "0.11.0-rc.1"])
    accepted = {"patch": "0.10.6", "minor": "0.11.0", "major": "1.0.0", "0.10.9": "0.10.9",
                "2.0.0": "2.0.0"}
    for requested, expected in accepted.items():
        box = Sandbox()
        result = box.run(script, REQUESTED=requested, FAKE_LATEST="0.10.5", FAKE_VERSIONS=published)
        require(result.returncode == 0, f"{requested} refused: {result.stderr}")
        require(box.outputs().get("version") == expected,
                f"{requested} resolved {box.outputs().get('version')!r}, not {expected}")
    for requested, latest in (("0.10.5", "0.10.5"), ("0.10.4", "0.10.5"), ("0.9.9", "0.10.5"), ("v0.10.6", "0.10.5"),
                              ("0.10", "0.10.5"), ("01.0.0", "0.10.5"), ("0.10.6-rc.1", "0.10.5"),
                              ("patch", "garbage"), ("", "0.10.5"), ("0.10.6; true", "0.10.5")):
        box = Sandbox()
        result = box.run(script, REQUESTED=requested, FAKE_LATEST=latest, FAKE_VERSIONS=published)
        require(result.returncode != 0 and "version=" not in box.output.read_text(),
                f"{requested!r} over latest {latest!r} was accepted")
    box = Sandbox()
    result = box.run(script, REQUESTED="0.10.6", FAKE_LATEST="0.10.5",
                     FAKE_VERSIONS=json.dumps(["0.10.5", "0.10.6"]))
    require(result.returncode != 0, "a version already on npm was accepted")
    box = Sandbox()
    result = box.run(script, REQUESTED="patch", FAKE_LATEST="0.10.5", FAKE_VERSIONS='"0.10.5"')
    require(result.returncode == 0 and box.outputs().get("version") == "0.10.6",
            "a package with one published version (npm prints a bare string) was refused")


def check_publish() -> None:
    script = step_script("publish", "id: publish")

    def attempt(**env: str) -> tuple[subprocess.CompletedProcess[str], Sandbox, Path]:
        box = Sandbox()
        (box.dir / "release").mkdir()
        archive = box.dir / "release/misofm-engine-0.10.6.tgz"
        archive.write_bytes(b"tested tarball bytes")
        sha256 = hashlib.sha256(archive.read_bytes()).hexdigest()
        base = {"VERSION": "0.10.6", "DRY_RUN": "false", "TARBALL": archive.name,
                "TARBALL_SHA256": sha256}
        return box.run(script, **{**base, **env}), box, archive

    def publishes(box: Sandbox) -> list[str]:
        return [call for call in box.npm_calls() if call.startswith("publish")
                and "--dry-run" not in call]

    result, box, archive = attempt()
    require(result.returncode == 0, f"a release on main failed: {result.stderr}")
    calls = publishes(box)
    require(len(calls) == 1 and "--provenance" in calls[0] and "--access public" in calls[0]
            and "--tag latest" in calls[0], f"expected one provenance publish, got {calls}")
    data = archive.read_bytes()
    require(box.outputs() == {
        "shasum": hashlib.sha1(data).hexdigest(),
        "integrity": "sha512-" + base64.b64encode(hashlib.sha512(data).digest()).decode(),
    }, f"wrong tarball identity outputs: {box.outputs()}")

    result, box, _ = attempt(DRY_RUN="true", GITHUB_REF="refs/heads/topic")
    require(result.returncode == 0 and not publishes(box)
            and any("--dry-run" in call for call in box.npm_calls()),
            "a dry run must call npm publish --dry-run and nothing else")
    for env, why in (({"GITHUB_REF": "refs/heads/topic"}, "a real publish from a branch"),
                     ({"NPM_TOKEN": "x"}, "a populated NPM_TOKEN"),
                     ({"NODE_AUTH_TOKEN": "x"}, "a populated NODE_AUTH_TOKEN"),
                     ({"NPM_CONFIG__AUTH_TOKEN": "x"}, "a populated NPM_CONFIG__AUTH_TOKEN"),
                     ({"TARBALL_SHA256": "0" * 64}, "a tarball the package job did not test"),
                     ({"TARBALL_SHA256": "0" * 64, "DRY_RUN": "true"}, "a swapped dry-run tarball")):
        result, box, _ = attempt(**env)
        require(result.returncode != 0 and not publishes(box), f"{why} was published")


def check_provenance() -> None:
    script = step_script("publish", "Fresh install imports every entry")
    verifier = heredoc(script, "node - audit.json <<'NODE'")

    def statement(**changes: object) -> dict:
        body = {
            "_type": "https://in-toto.io/Statement/v1",
            "predicateType": "https://slsa.dev/provenance/v1",
            "predicate": {"buildDefinition": {
                "externalParameters": {"workflow": {
                    "repository": "https://github.com/misofm/engine",
                    "path": ".github/workflows/npm-publish.yml", "ref": "refs/heads/main"}},
                "resolvedDependencies": [{"digest": {"gitCommit": SHA}}]}},
        }
        workflow = body["predicate"]["buildDefinition"]["externalParameters"]["workflow"]
        workflow.update({key: value for key, value in changes.items() if key in workflow})
        if "commit" in changes:
            body["predicate"]["buildDefinition"]["resolvedDependencies"] = [
                {"digest": {"gitCommit": changes["commit"]}}]
        return body

    def audit(body: dict, version: str = "0.10.6", extra: dict | None = None) -> dict:
        payload = base64.b64encode(json.dumps(body).encode()).decode()
        entry = {"name": PACKAGE, "version": version, "attestationBundles": [
            {"predicateType": "https://slsa.dev/provenance/v1",
             "bundle": {"dsseEnvelope": {"payload": payload}}}]}
        return {"verified": [entry], **(extra or {})}

    def verdict(report: dict) -> int:
        box = Sandbox()
        (box.dir / "audit.json").write_text(json.dumps(report), encoding="utf-8")
        (box.dir / "verify.js").write_text(verifier, encoding="utf-8")
        return subprocess.run(["node", "verify.js", "audit.json"], cwd=box.dir, capture_output=True,
                              env={**os.environ, "PACKAGE_NAME": PACKAGE, "VERSION": "0.10.6",
                                   "GITHUB_SHA": SHA, "GITHUB_REPOSITORY": "misofm/engine"},
                              check=False).returncode

    require(verdict(audit(statement())) == 0, "a matching provenance statement was refused")
    for report, why in (
            (audit(statement(commit="b" * 40)), "another commit"),
            (audit(statement(path=".github/workflows/other.yml")), "another workflow"),
            (audit(statement(ref="refs/heads/topic")), "another ref"),
            (audit(statement(repository="https://github.com/fork/engine")), "another repository"),
            (audit(statement(), version="0.10.5"), "another version"),
            (audit(statement(), extra={"invalid": [{"name": PACKAGE, "version": "0.10.6"}]}),
             "an invalid signature beside the verified one"),
            ({"verified": []}, "no verified signature")):
        require(verdict(report) != 0, f"provenance from {why} was accepted")


def main() -> int:
    check_layout()
    check_ref_guard()
    check_version_resolution()
    check_publish()
    check_provenance()
    print("npm-publish.yml release workflow: layout, version, publish and provenance rules hold")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
