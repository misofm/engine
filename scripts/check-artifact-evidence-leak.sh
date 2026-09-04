#!/usr/bin/env bash
# #105 phase 2 C2: a CI invocation that produces a shipped artifact names no evidence crate.
#
# Cargo resolves and unifies features across the packages selected by ONE invocation. Before this
# gate, the old `ci.yml` workflow (retired by design #359 §12 stage 3; qualification.yml is its
# sole successor) built `host-web` and `conformance` in a single
# `cargo build --target wasm32-unknown-unknown`, so conformance's edge to
# `engine/realtime-audit` was unified into the very wasm module the browser ships:
# `cargo tree` on the CI list showed `realtime-audit`, while `-p host-web` alone did
# not. `scripts/check-realtime-audit-leak.sh` (#84 phase D) proves each package's own graph is
# clean, which is necessary and not sufficient -- the leak lived in the invocation, not in a
# manifest. This gate owns the invocation.
#
# The rule has two halves, and both are enforced, because dropping the evidence crates from the
# artifact lists would otherwise silently drop their cross-target compile coverage:
#
#   1. an artifact invocation (a cross-target `cargo build`/`cargo check` that names a shipped
#      package) must not name an evidence crate; and
#   2. every cross-target triple that has an artifact invocation must also have a separate
#      invocation that compiles every evidence crate and no shipped package.
#
# N1: the workflow YAML is not the only source of cross-target cargo invocations --
# scripts/check-cross-targets.sh (called from qualification.yml) carries its own. It is scanned
# separately, below, for rule 1 only: some of its `--target` values come from a
# `for target in ...` shell loop variable rather than a literal triple in the script text, so
# rule 2's per-target artifact/coverage pairing (which needs a literal target string) stays owned
# by the workflow scan above -- the calling workflow is where this script's own evidence-crate
# cross-target coverage already lives (see "Evidence crates compile for Wasm").
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
cd "$root"

# design #359 §12 stage 3: qualification.yml is the sole required PR workflow and carries every
# cross-target wasm cargo line (the wasm-guests job among them), so it is this gate's one
# mandatory scan target -- its absence is a hard failure, not a skip.
workflows=(.github/workflows/qualification.yml)
for workflow in "${workflows[@]}"; do
    [[ -f "$workflow" ]] || { printf 'artifact evidence gate failure: missing %s\n' "$workflow" >&2; exit 1; }
done

cross_targets_script=scripts/check-cross-targets.sh
[[ -f "$cross_targets_script" ]] ||
    { printf 'artifact evidence gate failure: missing %s\n' "$cross_targets_script" >&2; exit 1; }

# The evidence crates: test scaffolding and the f64 oracle. Nothing that ships may resolve with
# them, because whatever features they turn on are unified into the artifact.
evidence=(conformance dsp-reference)
# The packages whose cross-target build IS the deliverable. effect-package ships a cdylib
# (design §6.7, scripts/check-release-shape.py's pinned cdylib/staticlib set) alongside host-web,
# host-mobile, host-core and capi.
shipped=(host-web host-mobile host-core capi effect-package)

fail() { printf 'artifact evidence gate failure: %s\n' "$1" >&2; exit 1; }

names_one_of() {
    local line=$1; shift
    local name
    for name in "$@"; do
        [[ "$line" == *" -p $name"* ]] && return 0
    done
    return 1
}

target_of() {
    # `--target <triple>` on a cargo line.
    sed -n 's/.*--target \([A-Za-z0-9_.-]*\).*/\1/p' <<<"$1"
}

total_artifact_targets=0
for workflow in "${workflows[@]}"; do
    artifact_targets=()
    coverage_targets=()
    while IFS= read -r line; do
        [[ "$line" == *"--target "* ]] || continue
        target="$(target_of "$line")"
        [[ -n "$target" ]] || continue
        if names_one_of "$line" "${shipped[@]}"; then
            for crate in "${evidence[@]}"; do
                if [[ "$line" == *" -p $crate"* ]]; then
                    printf '%s\n' "$line" >&2
                    fail "$workflow: the $target artifact invocation names the evidence crate $crate"
                fi
            done
            artifact_targets+=("$target")
            continue
        fi
        # A candidate coverage invocation: every evidence crate and no shipped package.
        missing=0
        for crate in "${evidence[@]}"; do
            [[ "$line" == *" -p $crate"* ]] || missing=1
        done
        [[ $missing -eq 0 ]] && coverage_targets+=("$target")
    done < <(grep -E '(^|[[:space:]])cargo (build|check)([[:space:]]|$)' "$workflow")

    [[ ${#artifact_targets[@]} -gt 0 ]] || fail "$workflow: found no cross-target artifact invocation to gate"

    mapfile -t artifact_targets < <(printf '%s\n' "${artifact_targets[@]}" | LC_ALL=C sort -u)
    coverage_list=" ${coverage_targets[*]:-} "
    for target in "${artifact_targets[@]}"; do
        [[ "$coverage_list" == *" $target "* ]] ||
            fail "$workflow: no evidence-crate compile-coverage invocation remains for $target"
    done
    total_artifact_targets=$((total_artifact_targets + ${#artifact_targets[@]}))
done

# N1: rule 1 only, over scripts/check-cross-targets.sh's own cargo invocations -- see the header
# comment for why rule 2 stays with the workflow scan above. Unlike the single-line `run:` entries
# in workflow YAML, this script wraps each cargo invocation across several `\`-continued physical
# lines for readability, so continuations are joined into one logical line first -- otherwise the
# `-p ...` package list (on its own continuation line) would never be seen alongside `cargo`.
script_shipped_invocations=0
while IFS= read -r line; do
    names_one_of "$line" "${shipped[@]}" || continue
    script_shipped_invocations=$((script_shipped_invocations + 1))
    for crate in "${evidence[@]}"; do
        if [[ "$line" == *" -p $crate"* ]]; then
            printf '%s\n' "$line" >&2
            fail "$cross_targets_script: invocation names both a shipped package and evidence crate $crate"
        fi
    done
done < <(sed -e ':a' -e '/\\$/{N;s/\\\n[[:space:]]*/ /;ba' -e '}' "$cross_targets_script" |
    grep -E '(^|[[:space:]])cargo (build|check|rustc)([[:space:]]|$)')
[[ $script_shipped_invocations -gt 0 ]] ||
    fail "$cross_targets_script: found no shipped-package cargo invocation to gate"

printf 'artifact evidence gate: ok (%s workflow(s), %s artifact targets, %s script shipped-package invocation(s), all evidence-free and all still covered)\n' \
    "${#workflows[@]}" "$total_artifact_targets" "$script_shipped_invocations"
