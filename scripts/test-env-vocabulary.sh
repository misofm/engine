#!/usr/bin/env bash
# Mutations for the environment/marker vocabulary gate (#104 phase C).
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

check() { bash "$1/scripts/check-env-vocabulary.sh" "$1"; }

new_case() {
    case_root="$scratch/$1"
    mkdir -p "$case_root"
    cp -R "$root/docs" "$root/scripts" "$root/tools" "$case_root/"
}

expect_failure() {
    local label=$1
    if check "$case_root" >/dev/null 2>&1; then
        printf 'env vocabulary mutation escaped: %s\n' "$label" >&2
        exit 1
    fi
}

new_case baseline
check "$case_root" >/dev/null

# Rule 1: a second prefix anywhere.
# Every name this file feeds the checker is assembled, never spelled, so that this file is scanned
# by the very rules it tests instead of needing an exemption from them.
retired_prefix=MISO_
family=MISO_ENGINE_BENCH_
new_case stray-prefix
printf '\n# %sRACK_BENCH_ROUND\n' "$retired_prefix" >>"$case_root/scripts/run-rack-benchmark.sh"
expect_failure stray-prefix

new_case stray-prefix-in-tool
printf '\n// %sINTERCHANGE_CANDIDATE_COMMIT\n' "$retired_prefix" \
    >>"$case_root/tools/miso-engine-bench/src/rack.rs"
expect_failure stray-prefix-in-tool

# Rule 2, forward: a name used but not documented. This is finding F2 -- the runner and the binary
# agreeing on a name nobody wrote down is exactly how they stopped agreeing.
new_case undocumented-name
printf '\nexport %sUNDECLARED=1\n' "$family" \
    >>"$case_root/scripts/run-rack-benchmark.sh"
expect_failure undocumented-name

# Rule 2, backward: a documented name nothing uses, and a documented name deleted from the table.
new_case unused-row
printf '| `%sABANDONED` | nothing reads this. |\n' "$family" \
    >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure unused-row

new_case deleted-row
sed -i "/^| \`${family}CPU_MODEL\`/d" "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure deleted-row

# The rule-1 exemption is exactly two paths wide: a stray name in a source or script file fails
# even though the same name in a spec does not.
new_case stray-prefix-in-doc
mkdir -p "$case_root/docs"
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
check "$case_root" >/dev/null

new_case missing-vocabulary
rm "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure missing-vocabulary

# A synonym for a fact that already has a name: the collapse #104 phase C performed must stay
# collapsed.
new_case reintroduced-synonym
printf '\nexport %sGOVERNOR=performance\n' "$family" \
    >>"$case_root/scripts/run-rack-benchmark.sh"
expect_failure reintroduced-synonym

printf 'env vocabulary mutations: ok\n'
