#!/usr/bin/env python3
"""Every workspace `test-support` feature must be turned on by a CI step that tests its package.

Issue #1021. A `test-support` feature gates whole tests and assertion blocks inside tests that do
run. Clippy's `--all-features` compiles them, so nothing looked wrong while no `cargo test` step
turned the feature on: four host-web/host-core tests and a protocol test never ran, and one of
them failed on `main` for three days before anyone noticed.

The rule: for every workspace package that declares `test-support`, some `cargo test` step of the
required workflow (`.github/workflows/qualification.yml`) must

* test the whole package: it selects the package (`-p`, or `--workspace` minus `--exclude`),
  selects no narrower target than `--all-targets`/`--tests`, passes no `--no-run`, and gives the
  test harness no filter, `--exact`, `--skip`, `--ignored` or `--list`;
* run unconditionally inside its job: a step-level `if:` does not count; and
* enable `<package>/test-support`, directly in `--features` (or `--all-features`) or by forwarding
  through the `[features]` tables of the workspace manifests.

A feature a dependency declaration turns on does not count. Whether it is unified in depends on
which other packages the step selects, so the step's local reproduction (`cargo test -p <pkg>`)
would not run the same program. A weak `dep?/feature` entry does not count either: it forwards
only if something else enables the dependency.

The checker is static: it reads the workflow and manifests as text and needs no Rust toolchain.
`scripts/test-test-support-ci.py` proves it rejects each protected mutation.
"""
from __future__ import annotations

import argparse
import glob
import pathlib
import re
import shlex
import sys
import tomllib

FEATURE = "test-support"
WORKFLOW = ".github/workflows/qualification.yml"
# Test-harness arguments (after `--`) that keep a run whole. Anything else -- a name filter,
# `--exact`, `--skip`, `--ignored`, `--list` -- narrows or replaces the run.
WHOLE_RUN_HARNESS_ARGS = {"--nocapture", "--show-output", "--include-ignored", "-q", "--quiet"}
WHOLE_RUN_HARNESS_PREFIXES = ("--test-threads=", "--color=", "--format=")
# Cargo target selectors narrower than `--all-targets`/`--tests`.
NARROWING_TARGET_FLAGS = {"--lib", "--bins", "--examples", "--benches", "--doc"}
NARROWING_TARGET_OPTIONS = {"--bin", "--test", "--example", "--bench"}
# Cargo options whose value is the next token when not written `--option=value`.
VALUE_OPTIONS = {
    "--target", "--target-dir", "--profile", "-j", "--jobs", "--color", "--message-format",
    "--config", "-Z", "--lockfile-path",
}
DEPENDENCY_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


class Invalid(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise Invalid(message)


class Package:
    def __init__(self, name: str, features: dict[str, list[str]], dependencies: dict[str, str]):
        self.name = name
        self.features = features
        # Dependency key (as written in `[features]` entries) -> package name.
        self.dependencies = dependencies


def dependency_package(key: str, spec: object, workspace_dependencies: dict) -> str:
    if isinstance(spec, dict):
        if isinstance(spec.get("package"), str):
            return spec["package"]
        if spec.get("workspace") is True:
            inherited = workspace_dependencies.get(key)
            if isinstance(inherited, dict) and isinstance(inherited.get("package"), str):
                return inherited["package"]
    return key


def load_workspace(root: pathlib.Path) -> dict[str, Package]:
    try:
        manifest = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise Invalid(f"Cargo.toml: {error}") from error
    workspace = manifest.get("workspace")
    require(isinstance(workspace, dict), "Cargo.toml: missing [workspace]")
    require("package" not in manifest,
            "Cargo.toml: a root [package] is not modeled; teach this checker its selection rule")
    members = workspace.get("members")
    require(isinstance(members, list) and members, "Cargo.toml: missing workspace.members")
    workspace_dependencies = workspace.get("dependencies", {})
    packages: dict[str, Package] = {}
    for pattern in members:
        paths = [str(root / pattern)]
        if glob.has_magic(pattern):
            paths = sorted(glob.glob(str(root / pattern)))
        require(bool(paths), f"Cargo.toml: workspace member {pattern!r} matches nothing")
        for path in paths:
            member = pathlib.Path(path) / "Cargo.toml"
            try:
                data = tomllib.loads(member.read_text(encoding="utf-8"))
            except (OSError, tomllib.TOMLDecodeError) as error:
                raise Invalid(f"{member.relative_to(root)}: {error}") from error
            name = data.get("package", {}).get("name")
            require(isinstance(name, str), f"{member.relative_to(root)}: missing package.name")
            dependencies: dict[str, str] = {}
            tables = [data] + [
                target for target in data.get("target", {}).values() if isinstance(target, dict)
            ]
            for table in tables:
                for kind in DEPENDENCY_TABLES:
                    for key, spec in table.get(kind, {}).items():
                        dependencies[key] = dependency_package(key, spec, workspace_dependencies)
            features = data.get("features", {})
            require(isinstance(features, dict), f"{member.relative_to(root)}: malformed [features]")
            packages[name] = Package(name, features, dependencies)
    return packages


def forwarded(packages: dict[str, Package], enabled: set[tuple[str, str]]) -> set[tuple[str, str]]:
    """Close `enabled` over the workspace `[features]` tables."""
    closed = set(enabled)
    pending = list(enabled)
    while pending:
        package_name, feature = pending.pop()
        package = packages.get(package_name)
        if package is None:
            continue
        for entry in package.features.get(feature, []):
            if entry.startswith("dep:"):
                continue
            if "/" in entry:
                dependency, dependency_feature = entry.split("/", 1)
                if dependency.endswith("?"):
                    continue  # weak: forwards only if something else enables the dependency
                target = (package.dependencies.get(dependency, dependency), dependency_feature)
            else:
                target = (package_name, entry)
            if target not in closed:
                closed.add(target)
                pending.append(target)
    return closed


def steps(text: str) -> list[tuple[str, str, bool, str]]:
    """(job, step name, conditional, run script) for every step that has a `run:`."""
    _, separator, jobs_block = text.partition("\njobs:\n")
    require(bool(separator), f"{WORKFLOW}: missing jobs:")
    found = []
    job_matches = list(re.finditer(r"^  ([A-Za-z0-9_-]+):\n", jobs_block, re.MULTILINE))
    require(bool(job_matches), f"{WORKFLOW}: no jobs found")
    for index, job_match in enumerate(job_matches):
        end = job_matches[index + 1].start() if index + 1 < len(job_matches) else len(jobs_block)
        job_text = jobs_block[job_match.end():end]
        step_starts = [match.start() for match in re.finditer(r"^      - ", job_text, re.MULTILINE)]
        for position, start in enumerate(step_starts):
            stop = step_starts[position + 1] if position + 1 < len(step_starts) else len(job_text)
            # Re-indent the step's first key so every key of the step sits at eight spaces.
            step_text = "        " + job_text[start + len("      - "):stop]
            conditional = re.search(r"^        if:", step_text, re.MULTILINE) is not None
            name_match = re.search(r"^        name: (.+)$", step_text, re.MULTILINE)
            name = name_match.group(1).strip() if name_match else "(unnamed step)"
            run_match = re.search(r"^        run:(.*)$", step_text, re.MULTILINE)
            if run_match is None:
                continue
            value = run_match.group(1).strip()
            if value in ("|", "|-", "|+", ">", ">-", ">+"):
                body = []
                for line in step_text[run_match.end() + 1:].splitlines():
                    if line.strip() and not line.startswith("          "):
                        break
                    body.append(line[10:])
                script = "\n".join(body)
            else:
                script = value
            found.append((job_match.group(1), name, conditional, script))
    return found


def logical_lines(script: str) -> list[str]:
    lines: list[str] = []
    current = ""
    for raw in script.splitlines():
        stripped = raw.strip()
        if not current and stripped.startswith("#"):
            continue
        if stripped.endswith("\\"):
            current += stripped[:-1] + " "
            continue
        current += stripped
        if current.strip():
            lines.append(current)
        current = ""
    if current.strip():
        lines.append(current)
    return lines


def cargo_test_invocations(line: str) -> list[list[str]]:
    lexer = shlex.shlex(line, posix=True, punctuation_chars=True)
    lexer.whitespace_split = True
    lexer.commenters = "#"
    try:
        tokens = list(lexer)
    except ValueError as error:
        raise Invalid(f"{WORKFLOW}: cannot tokenize run line {line!r}: {error}") from error
    invocations = []
    for index in range(len(tokens) - 1):
        if tokens[index] == "cargo" and tokens[index + 1] == "test":
            arguments = []
            for token in tokens[index + 2:]:
                if token and all(character in "();<>|&" for character in token):
                    if arguments and arguments[-1].isdigit():
                        arguments.pop()  # the file descriptor of a redirection such as `2>`
                    break
                arguments.append(token)
            invocations.append(arguments)
    return invocations


class Invocation:
    def __init__(self, job: str, step: str, packages: set[str], enabled: set[tuple[str, str]],
                 whole: bool, doctest: bool = False):
        self.job = job
        self.step = step
        self.packages = packages
        self.enabled = enabled
        self.whole = whole
        self.doctest = doctest


def parse_invocation(job: str, step: str, arguments: list[str],
                     packages: dict[str, Package]) -> Invocation:
    cargo_arguments, harness_arguments = arguments, []
    if "--" in arguments:
        split = arguments.index("--")
        cargo_arguments, harness_arguments = arguments[:split], arguments[split + 1:]
    selected: list[str] = []
    excluded: list[str] = []
    features: list[str] = []
    workspace = False
    all_features = False
    no_default = False
    whole = True
    doctest = False
    iterator = iter(cargo_arguments)
    for argument in iterator:
        option, equals, inline = argument.partition("=")
        if argument in ("-p", "--package"):
            selected.append(next(iterator, ""))
        elif option == "--package" and equals:
            selected.append(inline)
        elif argument.startswith("-p") and len(argument) > 2:
            selected.append(argument[2:])
        elif argument in ("--workspace", "--all"):
            workspace = True
        elif argument == "--exclude":
            excluded.append(next(iterator, ""))
        elif option == "--exclude" and equals:
            excluded.append(inline)
        elif argument in ("--features", "-F"):
            features.append(next(iterator, ""))
        elif option == "--features" and equals:
            features.append(inline)
        elif argument == "--all-features":
            all_features = True
        elif argument == "--no-default-features":
            no_default = True
        elif argument in ("--no-run", "--manifest-path") or option == "--manifest-path":
            whole = False
        elif argument in NARROWING_TARGET_FLAGS:
            whole = False
            doctest = doctest or argument == "--doc"
        elif option in NARROWING_TARGET_OPTIONS:
            whole = False
            if not equals:
                next(iterator, "")
        elif argument in VALUE_OPTIONS:
            next(iterator, "")
        elif not argument.startswith("-"):
            whole = False  # a positional test-name filter
    for harness_argument in harness_arguments:
        if harness_argument not in WHOLE_RUN_HARNESS_ARGS and not harness_argument.startswith(
                WHOLE_RUN_HARNESS_PREFIXES):
            whole = False
    where = f"{WORKFLOW}: job {job!r}, step {step!r}"
    for name in selected + excluded:
        require(name in packages, f"{where}: unknown workspace package {name!r}")
    if workspace:
        chosen = set(packages) - set(excluded)
    elif selected:
        chosen = set(selected)
    else:
        chosen = set(packages)  # a virtual workspace root with no default-members tests them all
    enabled: set[tuple[str, str]] = set()
    for item in (part for group in features for part in re.split(r"[,\s]+", group) if part):
        if "/" in item:
            package_name, feature = item.split("/", 1)
            if package_name in packages:
                require(feature in packages[package_name].features,
                        f"{where}: {package_name!r} declares no feature {feature!r}")
            enabled.add((package_name, feature))
        else:
            enabled.update((name, item) for name in chosen if item in packages[name].features)
    for name in chosen:
        if all_features:
            enabled.update((name, feature) for feature in packages[name].features)
        elif not no_default and "default" in packages[name].features:
            enabled.add((name, "default"))
    return Invocation(job, step, chosen, forwarded(packages, enabled), whole, doctest)


def invocations(root: pathlib.Path, packages: dict[str, Package]) -> list[Invocation]:
    try:
        text = (root / WORKFLOW).read_text(encoding="utf-8")
    except OSError as error:
        raise Invalid(f"{WORKFLOW}: {error}") from error
    found = []
    for job, step, conditional, script in steps(text):
        if conditional:
            continue
        for line in logical_lines(script):
            for arguments in cargo_test_invocations(line):
                found.append(parse_invocation(job, step, arguments, packages))
    require(any(invocation.whole for invocation in found),
            f"{WORKFLOW}: found no unconditional whole-package cargo test step; the parser has "
            "drifted from the workflow")
    return found


def check(root: pathlib.Path) -> list[str]:
    packages = load_workspace(root)
    declaring = sorted(name for name, package in packages.items() if FEATURE in package.features)
    require(bool(declaring), f"no workspace package declares {FEATURE!r}; the checker is stale")
    runs = invocations(root, packages)
    report = []
    missing = []
    for name in declaring:
        covering = [
            run for run in runs
            if run.whole and name in run.packages and (name, FEATURE) in run.enabled
        ]
        if covering:
            report.append(f"{name}/{FEATURE}: " + ", ".join(sorted({run.job for run in covering})))
        else:
            missing.append(name)
    require(not missing, "no unconditional whole-package cargo test step in "
            f"{WORKFLOW} tests these packages with their {FEATURE} feature enabled, so their "
            f"gated tests never run in CI: {', '.join(missing)}")
    # `--all-targets` runs no doctest (#1422), so each job's `--doc` step must cover exactly the
    # packages and test-support features of a whole-package step of the same job; a feature
    # dropped from the doctest step alone would otherwise pass unseen.
    for run in runs:
        if run.doctest:
            siblings = [other for other in runs if other.whole and other.job == run.job]
            require(any(other.packages == run.packages and other.enabled == run.enabled
                        for other in siblings),
                    f"{WORKFLOW}: job {run.job!r}, step {run.step!r}: the doctest step must "
                    "enable the same packages and features as a whole-package cargo test step of "
                    "the same job, so its doctests are not built against a different feature set")
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    parser.add_argument("--root", type=pathlib.Path,
                        default=pathlib.Path(__file__).resolve().parent.parent)
    args = parser.parse_args()
    try:
        report = check(args.root)
    except Invalid as error:
        print(f"test-support CI coverage check failed: {error}", file=sys.stderr)
        return 1
    for line in report:
        print(line)
    print(f"test-support CI coverage passed: {len(report)} packages")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
