#!/usr/bin/env bash
# Synthetic-only regression for the Issue 081 Wasm export-section parser.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: test-effect-interchange-target-export-parser.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
checker="$root/scripts/check-effect-interchange-targets.sh"
source "$checker"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

accepted="$scratch/accepted.txt"
printf '%s\n' \
    'miso_engine_effect_package.wasm: file format wasm 0x1' \
    'Module name: <miso_engine_effect_package.wasm>' \
    'Function[2]:' \
    ' - func[0] sig=0 <miso_engine_internal_call>' \
    'Export[2]:' \
    ' - memory[0] -> "memory"' \
    ' - func[1] <miso_engine_effect_descriptor_v1_inspect> -> "miso_engine_effect_descriptor_v1_inspect"' \
    'Code[2]:' \
    ' - func[1] size=4 <miso_engine_effect_descriptor_v1_inspect>' \
    >"$accepted"
validate_wasm_exports "$accepted" synthetic-accepted

expect_rejection() {
    local fixture=$1 label=$2
    if validate_wasm_exports "$fixture" "synthetic-$label" >/dev/null 2>&1; then
        printf 'effect interchange export parser regression: accepted %s\n' "$label" >&2
        exit 1
    fi
}

module_only="$scratch/module-only.txt"
printf '%s\n' \
    'Module name: <miso_engine_effect_descriptor_v1_inspect>' \
    'Function[1]:' \
    ' - func[0] <miso_engine_effect_descriptor_v1_inspect>' \
    'Export[1]:' \
    ' - memory[0] -> "memory"' \
    'Code[1]:' \
    ' - func[0] <miso_engine_effect_descriptor_v1_inspect>' \
    >"$module_only"
expect_rejection "$module_only" module-and-call-references

wrong_kind="$scratch/wrong-kind.txt"
printf '%s\n' \
    'Export[1]:' \
    ' - memory[0] -> "miso_engine_effect_descriptor_v1_inspect"' \
    'Code[0]:' \
    >"$wrong_kind"
expect_rejection "$wrong_kind" non-function-export

extra="$scratch/extra.txt"
cp "$accepted" "$extra"
sed -i '/Code\[2\]:/i\ - func[2] <miso_engine_unexpected> -> "miso_engine_unexpected"' "$extra"
expect_rejection "$extra" extra-export

duplicate="$scratch/duplicate.txt"
cp "$accepted" "$duplicate"
sed -i '/Code\[2\]:/i\ - func[1] <miso_engine_effect_descriptor_v1_inspect> -> "miso_engine_effect_descriptor_v1_inspect"' "$duplicate"
expect_rejection "$duplicate" duplicate-export

printf 'effect interchange Wasm export parser regression: ok synthetic_only=1\n'
