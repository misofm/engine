#!/usr/bin/env python3
"""Hermetic mutation tests for scripts/check-script-reachability.py (issue #1027).

Each case copies the repository's workflows, `scripts/`, every `package.json` and every other
shell/Python/jq/JavaScript/TypeScript/YAML file outside `artifacts/`, `docs/` and the issue specs
into a scratch Git work tree, applies one mutation and runs the checker there. The unmutated
scratch must pass, or each "refused" below could be refusing for an unrelated reason.
"""
from __future__ import annotations

import pathlib
import shutil
import subprocess
import sys
import tempfile
from typing import Callable

ROOT = pathlib.Path(__file__).resolve().parent.parent
CHECKER = ROOT / "scripts/check-script-reachability.py"
WORKFLOW = ".github/workflows/qualification.yml"
CARRIER_SUFFIXES = (".sh", ".bash", ".py", ".jq", ".mjs", ".cjs", ".js", ".ts", ".yml", ".yaml")
NOT_COPIED = ("artifacts/", "docs/", ".github/ISSUE_SPECS/")
# The lint step every case anchors a new workflow line on. It must occur exactly once.
ANCHOR = "          python3 -B scripts/check-script-reachability.py\n"
DEAD = "scripts/run-one-shot-benchmark.sh"
JQ_LIBRARY = "scripts/protocol-benchmark-record-validator.jq"

Mutation = Callable[[pathlib.Path], None]


def copied() -> list[str]:
    listing = subprocess.run(
        ["git", "-C", str(ROOT), "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
        stdout=subprocess.PIPE, check=True).stdout.decode().split("\0")
    keep = []
    for path in listing:
        if not path or not (ROOT / path).is_file():
            continue
        if (path.startswith((".github/workflows/", "scripts/"))
                or path.rsplit("/", 1)[-1] == "package.json"
                or (path.endswith(CARRIER_SUFFIXES) and not path.startswith(NOT_COPIED))):
            keep.append(path)
    return keep


FILES = copied()


def workspace() -> pathlib.Path:
    root = pathlib.Path(tempfile.mkdtemp(prefix="script-reachability-"))
    for path in FILES:
        (root / path).parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT / path, root / path)
    subprocess.run(["git", "init", "-q", str(root)], check=True)
    return root


def check(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run([sys.executable, "-B", str(CHECKER), "--root", str(root)],
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False)


def replace(relative: str, old: str, new: str) -> Mutation:
    def apply(root: pathlib.Path) -> None:
        path = root / relative
        text = path.read_text(encoding="utf-8")
        if text.count(old) != 1:
            raise AssertionError(f"mutation anchor must occur once in {relative}: {old!r}")
        path.write_text(text.replace(old, new, 1), encoding="utf-8")
    return apply


def write(relative: str, text: str) -> Mutation:
    def apply(root: pathlib.Path) -> None:
        (root / relative).parent.mkdir(parents=True, exist_ok=True)
        (root / relative).write_text(text, encoding="utf-8")
    return apply


def remove_tree(relative: str) -> Mutation:
    def apply(root: pathlib.Path) -> None:
        shutil.rmtree(root / relative)
    return apply


def workflow_line(line: str) -> Mutation:
    return replace(WORKFLOW, ANCHOR, ANCHOR + line + "\n")


def outcome(mutations: list[Mutation]) -> subprocess.CompletedProcess[str]:
    root = workspace()
    try:
        for mutation in mutations:
            mutation(root)
        return check(root)
    finally:
        shutil.rmtree(root)


def refused(reason: str, named: set[str], *mutations: Mutation, text: str | None = None) -> None:
    """The checker must refuse, naming exactly `named` as unreached."""
    result = outcome(list(mutations))
    marker = "script reachability failure: unreached from any workflow: "
    found = {line.split(marker, 1)[1].split(" ", 1)[0]
             for line in result.stderr.splitlines() if marker in line}
    if result.returncode == 0 or found != named or (text is not None and text not in result.stderr):
        raise AssertionError(f"mutation not refused as expected ({reason}): named {sorted(found)}, "
                             f"expected {sorted(named)}; stderr: {result.stderr}")


def passes(reason: str, *mutations: Mutation) -> None:
    result = outcome(list(mutations))
    if result.returncode != 0:
        raise AssertionError(f"mutation rejected ({reason}): {result.stderr}")


DEAD_SCRIPT = write(DEAD, "#!/usr/bin/env bash\necho one-shot\n")


def main() -> int:
    baseline = outcome([])
    if baseline.returncode != 0:
        raise AssertionError(f"unmutated scratch rejected: {baseline.stdout}{baseline.stderr}")
    cases = 0

    # jq's extension-less `include "name"` is the only reference to the record validator library;
    # a basename-only scan would report it dead, and removing the includes must make it dead.
    refused("jq includes of the record validator removed", {JQ_LIBRARY},
            replace("scripts/protocol-benchmark-validator.jq",
                    'include "protocol-benchmark-record-validator";', ""),
            replace("scripts/run-protocol-benchmark.sh",
                    'include "protocol-benchmark-record-validator"; ', ""),
            replace("scripts/test-protocol-benchmark.sh",
                    'include "protocol-benchmark-record-validator"; ', ""))
    cases += 1

    # A one-shot runner nothing runs, however it is mentioned outside a live line.
    refused("a new script nothing runs", {DEAD}, DEAD_SCRIPT)
    refused("named only in a workflow comment", {DEAD}, DEAD_SCRIPT,
            workflow_line(f"          # bash {DEAD}"))
    # A Rust file is never a carrier, even one a workflow names: an ignore note or a string there
    # is how a dead runner hid.
    refused("named only in a Rust ignore note and string", {DEAD}, DEAD_SCRIPT,
            write("crates/example/tests/bench.rs",
                  f'#[test]\n#[ignore = "run once through {DEAD}"]\n'
                  f'fn bench() {{ let _ = "{DEAD}"; }}\n'),
            workflow_line("          rustfmt --check crates/example/tests/bench.rs"))
    # Nor is a tool under docs/, even one a reached script names.
    refused("named only by a reached tool under docs/", {DEAD}, DEAD_SCRIPT,
            write("docs/example/tools/audit-helper.py", f'print("bash {DEAD}")\n'),
            workflow_line("          echo see docs/example/tools/audit-helper.py"))
    refused("named only in a document", {DEAD}, DEAD_SCRIPT,
            write("docs/example.md", f"Run `bash {DEAD}` once.\n"))
    passes("named on a live workflow line", DEAD_SCRIPT, workflow_line(f"          bash {DEAD}"))
    cases += 6

    # Transitive reach through a script, and its break.
    outer = "scripts/check-new-outer.sh"
    inner = "scripts/new-inner-lib.sh"
    chain = [write(outer, '#!/usr/bin/env bash\nroot=.\nsource "$root/scripts/new-inner-lib.sh"\n'),
             write(inner, "helper() { :; }\n"), workflow_line(f"          bash {outer}")]
    passes("reached through a script a workflow runs", *chain)
    refused("the only reference to the inner script commented out", {inner}, *chain,
            replace(outer, 'source "$root', '# source "$root'))
    cases += 2

    # Python imports count, from Python files only.
    runner = "scripts/check-new-runner.py"
    module = "scripts/new_helper_module.py"
    python = [write(module, "VALUE = 1\n"), workflow_line(f"          python3 -B {runner}")]
    passes("imported by a reached Python script",
           write(runner, "import new_helper_module\nprint(new_helper_module.VALUE)\n"), *python)
    passes("imported with from ... import",
           write(runner, "from new_helper_module import VALUE\nprint(VALUE)\n"), *python)
    refused("no import left", {module}, write(runner, "print(1)\n"), *python)
    cases += 3

    # One level of brace lists names each alternative.
    brace = [write("scripts/new-alpha.sh", ":\n"), write("scripts/new-beta.sh", ":\n")]
    passes("named through a brace list",
           workflow_line('          for name in scripts/new-{alpha,beta}.sh; do bash "$name"; done'),
           *brace)
    refused("a brace list that leaves one out", {"scripts/new-beta.sh"},
            workflow_line('          for name in scripts/new-{alpha,gamma}.sh; do bash "$name"; done'),
            *brace)
    cases += 2

    # A data file under scripts/ is governed too, and a reached script reaches it.
    fixture = "scripts/fixtures/new-one-shot-record.json"
    refused("an unreferenced fixture", {fixture}, write(fixture, "{}\n"))
    passes("a fixture a reached script reads", write(fixture, "{}\n"),
           write(outer, f'#!/usr/bin/env bash\njq . "{fixture}"\n'),
           workflow_line(f"          bash {outer}"))
    cases += 2

    # scripts/operator/ is exempt by design.
    passes("an unreached operator tool", write("scripts/operator/new-tool.sh", ":\n"))
    cases += 1

    # No workflow directory is a refusal, not a vacuous pass.
    empty = outcome([remove_tree(".github/workflows")])
    if empty.returncode == 0 or "no workflow under .github/workflows/" not in empty.stderr:
        raise AssertionError(f"a tree without workflows was accepted: {empty.stderr}")
    cases += 1

    print(f"script reachability mutations: ok ({cases} cases)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
