#!/usr/bin/env bash
# One environment/marker vocabulary for the whole repository (#104 phase C).
#
# Rule 1: no tracked file contains a `MISO_`-prefixed identifier that does not continue
#         `MISO_ENGINE_`.
# Rule 2: every `MISO_ENGINE_*` identifier under `tools/` or `scripts/` is a row of the table in
#         `docs/ENGINE_ENV_VOCABULARY.md`, and every row of that table is used under `tools/` or
#         `scripts/`.
#
# Rule 2 is bidirectional on purpose. An undocumented name is how a runner and its binary drift
# apart (#104 F2: the sole authorized builtins runner exported none of the sixteen names the binary
# read, so every accepted record carried all-null metadata). An unused row is the same defect seen
# from the other side: it is a name nobody agreed to stop using.
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
cd "$root"

fail() { printf 'env vocabulary failure: %s\n' "$1" >&2; exit 1; }

vocabulary=docs/ENGINE_ENV_VOCABULARY.md
[[ -f "$vocabulary" && ! -L "$vocabulary" ]] || fail "missing vocabulary: $vocabulary"

# The scan covers tracked and not-yet-ignored files when this is a git worktree, and the tree
# otherwise, so the mutation test can run it against a scratch copy. Two paths are excluded from rule 1
# and both have to be: `$vocabulary`, which documents which prefixes were retired, and
# `.github/ISSUE_SPECS/`, whose whole job is to record what a name used to be. No source,
# configuration or script file is excluded -- `scripts/test-env-vocabulary.sh` assembles the names
# it feeds this checker rather than spelling them, so it is scanned like every other file.
sources() {
    if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        git ls-files -z --cached --others --exclude-standard
    else
        find . -type f -not -path './.git/*' -not -path './target/*' -print0
    fi | tr '\0' '\n' | sed 's|^\./||' |
        grep -v -x -F -e "$vocabulary" | grep -v '^\.github/ISSUE_SPECS/' | tr '\n' '\0'
}

# Rule 1.
stray="$(sources | xargs -0 grep -hoE 'MISO_[A-Z0-9_]+' 2>/dev/null | sort -u | grep -v '^MISO_ENGINE_' || true)"
if [[ -n "$stray" ]]; then
    printf '%s\n' "$stray" >&2
    fail 'identifier outside the MISO_ENGINE_ prefix'
fi

# Rule 2. A match ending in an underscore is a prose fragment naming a family, not a name; a real
# variable or marker never ends in one.
used="$(grep -rhoE 'MISO_ENGINE_[A-Z0-9_]+' tools scripts | grep -v '_$' | sort -u)"
documented="$(grep -oE '^\| `MISO_ENGINE_[A-Z0-9_]+`' "$vocabulary" | tr -d '|` ' | sort -u)"

undocumented="$(comm -23 <(printf '%s\n' "$used") <(printf '%s\n' "$documented"))"
if [[ -n "$undocumented" ]]; then
    printf '%s\n' "$undocumented" >&2
    fail "name used under tools/ or scripts/ but absent from $vocabulary"
fi

unused="$(comm -13 <(printf '%s\n' "$used") <(printf '%s\n' "$documented"))"
if [[ -n "$unused" ]]; then
    printf '%s\n' "$unused" >&2
    fail "name documented in $vocabulary but unused under tools/ or scripts/"
fi

printf 'env vocabulary: ok (%s names, one MISO_ENGINE_ prefix)\n' "$(printf '%s\n' "$documented" | wc -l | tr -d ' ')"
