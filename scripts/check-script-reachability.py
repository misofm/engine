#!/usr/bin/env python3
"""Every file under `scripts/` outside `scripts/operator/` is reachable from a GitHub workflow.

Issue #1027. `scripts/operator/README.md` states the rule; nothing checked it, and by
2026-09-28 eighteen files had quietly stopped being reached: seventeen used-up one-shot
benchmark runners, validators and fixtures whose records were long sealed, and the live console
benchmark a person runs. A file that no workflow reaches is either a tool a person runs, which
belongs in `scripts/operator/`, or dead.

Reachability is a fixed point over mentions. The seeds are every workflow under
`.github/workflows/` and every `package.json` (the npm scripts a workflow runs). A reached file
that is a carrier (shell, Python, jq, JavaScript/TypeScript, YAML, `package.json`; not this
checker or its test, which name scripts as data) reaches every file it mentions on a line that is
not a comment:

* by basename, anywhere in the text, after expanding one level of `a{b,c}d` brace lists;
* a jq module by its extension-less `include "name"` or `import "name" as ...`, which is how jq
  pulls in a validator library (`protocol-benchmark-record-validator.jq` is reached only this
  way, and a basename-only scan reports it dead);
* a Python module by `import name` or `from name import ...`.

Carriers outside `scripts/` (for example `hosts/host-web/qualification/*.mjs`) count once they
are themselves reached. Rust sources are deliberately not carriers: an `#[ignore = "run through
scripts/..."]` note or a doc string would otherwise keep a dead runner alive, which is exactly
how the one-shots this rule caught hid.

`NAMED_EXCEPTIONS` lists the files allowed to be unreached, each with its reason. An exception
that is reached again, or no longer exists, fails too, so the list cannot rot.

The checker is static and launches nothing. `scripts/test-script-reachability.py` proves it
rejects each protected mutation.
"""
from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

OPERATOR = "scripts/operator/"
NAMED_EXCEPTIONS = {
    "scripts/run-console-benchmark.sh": (
        "the live native console benchmark, run by a person; its move to scripts/operator/ "
        "waits for #1025, which rewrites the same file in the same batch"
    ),
}
# This rule's own checker and mutation test name scripts as data, never to run them, so they are
# reached (a workflow runs them) but never carriers: otherwise `NAMED_EXCEPTIONS` would reach every
# exception it lists.
SELF = ("scripts/check-script-reachability.py", "scripts/test-script-reachability.py")
HASH_COMMENTS = (".sh", ".bash", ".py", ".jq", ".yml", ".yaml")
SLASH_COMMENTS = (".mjs", ".cjs", ".js", ".ts")
CARRIER_SUFFIXES = HASH_COMMENTS + SLASH_COMMENTS
# Carriers outside scripts/ never come from sealed records or issue prose.
NOT_CARRIERS = ("artifacts/", ".github/ISSUE_SPECS/", "docs/")
BRACES = re.compile(r"([A-Za-z0-9_./-]*)\{([A-Za-z0-9_.,-]+)\}([A-Za-z0-9_./-]*)")
TOKEN = re.compile(r"[A-Za-z0-9_+-][A-Za-z0-9_.+-]*")
JQ_MODULE = re.compile(r"\b(?:include|import)\s+\"([^\"]+)\"")
PY_IMPORT = re.compile(r"^[ \t]*(?:from[ \t]+([A-Za-z_][\w.]*)[ \t]+import\b|"
                       r"import[ \t]+([A-Za-z_][\w.]*(?:[ \t]*,[ \t]*[A-Za-z_][\w.]*)*))",
                       re.MULTILINE)


class Invalid(RuntimeError):
    pass


def listed(root: pathlib.Path) -> list[str]:
    result = subprocess.run(
        ["git", "-C", str(root), "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if result.returncode != 0:
        raise Invalid(f"git ls-files failed ({result.returncode}): "
                      f"{result.stderr.decode(errors='replace').strip()}")
    paths = sorted({path for path in result.stdout.decode().split("\0") if path})
    # A tracked file deleted from the working tree is gone for this rule.
    return [path for path in paths if (root / path).is_file()]


def seed(path: str) -> bool:
    return ((path.startswith(".github/workflows/") and path.endswith((".yml", ".yaml")))
            or path.rsplit("/", 1)[-1] == "package.json")


def carrier(path: str) -> bool:
    if seed(path):
        return True
    if path in SELF:
        return False
    if not path.startswith("scripts/") and path.startswith(NOT_CARRIERS):
        return False
    return path.endswith(CARRIER_SUFFIXES)


def code_text(path: str, text: str) -> str:
    kept = []
    for line in text.splitlines():
        stripped = line.strip()
        if path.endswith(HASH_COMMENTS) and stripped.startswith("#") and not stripped.startswith("#!"):
            continue
        if path.endswith(SLASH_COMMENTS) and stripped.startswith(("//", "/*", "*")):
            continue
        kept.append(line)
    code = "\n".join(kept)
    expanded = [prefix + alternative + suffix
                for prefix, alternatives, suffix in BRACES.findall(code)
                for alternative in alternatives.split(",")]
    return code + "\n" + "\n".join(expanded)


def mentioned(path: str, text: str) -> set[str]:
    code = code_text(path, text)
    names = {token.rstrip(".") for token in TOKEN.findall(code)}
    if path.endswith(".jq") or path.endswith(HASH_COMMENTS):
        # jq programs are embedded in shell and YAML as often as they are files of their own.
        names |= {module.rsplit("/", 1)[-1] + ".jq" for module in JQ_MODULE.findall(code)}
    if path.endswith(".py"):
        for from_module, imported in PY_IMPORT.findall(code):
            for module in ([from_module] if from_module else imported.split(",")):
                module = module.strip()
                if module:
                    names.add(module.rsplit(".", 1)[-1] + ".py")
    return names


def reachability(root: pathlib.Path) -> tuple[list[str], set[str]]:
    paths = listed(root)
    by_name: dict[str, list[str]] = {}
    for path in paths:
        if path.startswith("scripts/") or carrier(path):
            by_name.setdefault(path.rsplit("/", 1)[-1], []).append(path)
    reached = {path for path in paths if seed(path)}
    if not any(path.startswith(".github/workflows/") for path in reached):
        raise Invalid("no workflow under .github/workflows/")
    frontier = sorted(reached)
    while frontier:
        source = frontier.pop()
        if not carrier(source):
            continue
        text = (root / source).read_text(encoding="utf-8", errors="replace")
        for name in mentioned(source, text):
            for target in by_name.get(name, ()):
                if target not in reached:
                    reached.add(target)
                    frontier.append(target)
    return paths, reached


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    parser.add_argument("--root", type=pathlib.Path,
                        default=pathlib.Path(__file__).resolve().parent.parent)
    root = parser.parse_args().root.resolve()
    try:
        paths, reached = reachability(root)
    except Invalid as error:
        print(f"script reachability failure: {error}", file=sys.stderr)
        return 1
    governed = [path for path in paths
                if path.startswith("scripts/") and not path.startswith(OPERATOR)]
    if not governed:
        print("script reachability failure: no files under scripts/", file=sys.stderr)
        return 1
    problems = []
    for path in governed:
        if path not in reached and path not in NAMED_EXCEPTIONS:
            problems.append(f"unreached from any workflow: {path} (wire it into a workflow, "
                            f"move a tool a person runs to {OPERATOR}, or delete it)")
    for path in sorted(NAMED_EXCEPTIONS):
        if path not in paths:
            problems.append(f"named exception no longer exists: {path}")
        elif path in reached:
            problems.append(f"named exception is reached again; remove it from the list: {path}")
    if problems:
        for problem in problems:
            print(f"script reachability failure: {problem}", file=sys.stderr)
        return 1
    operator = sum(1 for path in paths if path.startswith(OPERATOR))
    print(f"script reachability: ok ({len(governed) - len(NAMED_EXCEPTIONS)} files under scripts/ "
          f"reached from workflows, {operator} under {OPERATOR} exempt, "
          f"{len(NAMED_EXCEPTIONS)} named exception)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
