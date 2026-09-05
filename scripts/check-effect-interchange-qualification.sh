#!/usr/bin/env bash
# Static Issue 081 qualification boundary and immutable-baseline checker.
#
# #104 phase A / #83 wave-4 decision W4-D2. `fixtures/effect-interchange/v1/ACCEPTED.sha256` used
# to seal twelve `crates/miso-engine-effect-{compiler,package}` source files alongside the
# interchange corpus. Waves 1-4 rewrote six of them (`effect-compiler/src/prepare.rs`,
# `effect-package/src/{diagnostic,ffi,lib,package,wire}.rs`), so the source half of the seal went
# permanently red and cannot be refreshed without re-running the Issue-081 qualification. The
# source rows are retired and the manifest is now exactly the 24 corpus/reference-script rows,
# which still verify byte-for-byte on an unchanged tree. The retired rows and the previous manifest
# identity are recorded in `.github/ISSUE_SPECS/081-*.md`.
#
#   accepted manifest identity, before: 6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5 (36 rows)
#   accepted manifest identity, after:  1aaa96dc731c0da3dabb2f8ecd7c2bf803078b580a38cccfccf1ffe280c83588 (24 rows)
#
# No corpus byte changed. The manifest is still self-pinned, so a silent refresh after a fixture
# edit is still a failure (`scripts/test-effect-interchange-policy.sh`, mutation
# `refreshed-baseline`).
#
# Issue #143 P0. The observation section is an additive change to the descriptor wire, so it moves
# exactly two sealed rows and adds three:
#
#   * `scripts/effect-descriptor-v1-reference.py` -- the independent encoder/verifier now carries
#     the observation section and its own mutation matrix. There is no way to add a wire section
#     without moving this row.
#   * `fixtures/effect-descriptor/v1/MANIFEST.sha256` -- three `comprehensive-c` rows added. That
#     manifest is not asserted, it is *recomputed* from the fixture files by the reference's
#     `check()`, so its new identity is derived rather than declared.
#   * `fixtures/effect-descriptor/v1/comprehensive-c.{json,wire.hex,identity.hex}` -- the new
#     tap-bearing vector. `comprehensive-c` is `comprehensive-a` plus a two-tap menu and nothing
#     else (the effect id and display name are the same byte lengths), so `total(C) - total(A)` is
#     exactly `32 + len(name) + len(unit)` per tap; that formula is asserted in both the Python
#     reference and `descriptor_v1_qualification.rs`. The wire bytes are the Python encoder's, and
#     the Rust encoder reproduces them byte for byte in
#     `checked_vectors_match_independent_wire_identity_and_port_permutation`.
#
# Every pre-#143 row is byte-unchanged, which is the point: a zero-tap descriptor encodes to the
# identity it always had, so `comprehensive-a`/`-b` and the whole package and state corpora do not
# move. The re-seal was taken after re-running the 100-process independent-reference matrix
# (`scripts/run-effect-interchange-reference-processes.sh`) on the changed tree.
#
#   accepted manifest identity, before #143: 1aaa96dc731c0da3dabb2f8ecd7c2bf803078b580a38cccfccf1ffe280c83588 (24 rows)
#   accepted manifest identity, after  #143: e3896726979aa746cfda50fc10c1985c0ecef117f87b39e692f18226b7b4fa14 (27 rows)
#
# The identity is pinned in three places -- here, `preflight-effect-interchange-benchmark.sh` and
# `run-effect-interchange-benchmark.sh` -- and all three moved together. A re-seal that moves only
# one is caught by `scripts/test-effect-interchange-benchmark.sh`.
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'effect interchange qualification policy failure: %s\n' "$1" >&2; exit 1; }

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
required_scan() {
    local label=$1
    shift
    local output="$scratch/required-$RANDOM.out" error="$scratch/required-$RANDOM.err" status
    if "$@" >"$output" 2>"$error"; then status=0; else status=$?; fi
    if [[ "$status" -ne 0 ]]; then
        cat "$output" "$error" >&2
        fail "$label failed (status $status)"
    fi
    [[ -s "$output" ]] || fail "$label produced no match"
}
forbidden_scan() {
    local label=$1
    shift
    local output="$scratch/forbidden-$RANDOM.out" error="$scratch/forbidden-$RANDOM.err" status
    if "$@" >"$output" 2>"$error"; then status=0; else status=$?; fi
    if [[ "$status" -gt 1 ]]; then
        cat "$output" "$error" >&2
        fail "$label failed (status $status)"
    fi
    if [[ "$status" -eq 0 ]]; then
        cat "$output" >&2
        fail "$label found a prohibited match"
    fi
}
checked_value() {
    local label=$1 variable=$2
    shift 2
    local output="$scratch/value-$RANDOM.out" error="$scratch/value-$RANDOM.err" status value
    if "$@" >"$output" 2>"$error"; then status=0; else status=$?; fi
    if [[ "$status" -ne 0 ]]; then
        cat "$output" "$error" >&2
        fail "$label failed (status $status)"
    fi
    value=$(<"$output")
    printf -v "$variable" '%s' "$value"
}

manifest=fixtures/effect-interchange/v1/ACCEPTED.sha256
accepted_manifest_sha256=e3896726979aa746cfda50fc10c1985c0ecef117f87b39e692f18226b7b4fa14
[[ -f "$manifest" && ! -L "$manifest" ]] || fail 'missing immutable baseline manifest'
checked_value 'manifest hash production' manifest_sha256_output sha256sum "$manifest"
checked_value 'manifest hash extraction' manifest_sha256 awk '{print $1}' <<<"$manifest_sha256_output"
[[ "$manifest_sha256" == "$accepted_manifest_sha256" ]] ||
    fail 'immutable baseline manifest changed or was refreshed'
sort_error="$scratch/manifest-sort.err"
if LC_ALL=C sort -c -k2,2 "$manifest" 2>"$sort_error"; then status=0; else status=$?; fi
if [[ "$status" -ne 0 ]]; then cat "$sort_error" >&2; fail "baseline manifest sort failed (status $status)"; fi
check_output="$scratch/manifest-check.out"; check_error="$scratch/manifest-check.err"
if sha256sum --check --strict "$manifest" >"$check_output" 2>"$check_error"; then status=0; else status=$?; fi
if [[ "$status" -ne 0 ]]; then cat "$check_output" "$check_error" >&2; fail "accepted baseline check failed (status $status)"; fi
# 24 corpus/reference-script rows plus the three `comprehensive-c` rows issue #143 added.
checked_value 'manifest line count' manifest_lines wc -l <"$manifest"
checked_value 'manifest line count filtering' manifest_count tr -d ' ' <<<"$manifest_lines"
[[ "$manifest_count" -eq 27 ]] || fail 'baseline membership changed'

for path in \
    scripts/effect-interchange-v1-reference.py \
    scripts/run-effect-interchange-reference-processes.sh \
    scripts/test-effect-interchange-reference-runner.sh \
    scripts/check-effect-interchange-targets.sh \
    scripts/check-cross-targets.sh \
    scripts/test-effect-interchange-target-export-parser.sh \
    scripts/effect-interchange-benchmark-validator.py \
    scripts/effect-interchange-benchmark-108-validator.py \
    scripts/check-effect-interchange-benchmark-108.sh \
    scripts/preflight-effect-interchange-benchmark.sh \
    scripts/run-effect-interchange-benchmark.sh \
    scripts/test-effect-interchange-benchmark.sh \
    tools/bench/Cargo.toml \
    tools/bench/src/effect_interchange.rs \
    docs/EFFECT_INTERCHANGE_QUALIFICATION_V1.md; do
    [[ -f "$path" ]] || fail "missing qualification path $path"
done
benchmark=tools/bench/src/effect_interchange.rs
required_scan 'benchmark observation count scan' rg 'const OBSERVATIONS: usize = 256;' "$benchmark"
for workload in descriptor_verify_identity_a package_verify_cid_select_a state_verify_reencode_current migration_two_step_bank_restore; do
    required_scan "benchmark workload scan: $workload" rg "\"$workload\"" "$benchmark"
done
required_scan 'benchmark rounds scan' rg 'for round in 1\.\.=2' "$benchmark"
required_scan 'benchmark record count scan' rg -F 'assert_eq!(records.len(), 8);' "$benchmark"
issue_output="$scratch/issue-branch.out"; issue_error="$scratch/issue-branch.err"
if rg -F '\"issue\":108' "$benchmark" >"$issue_output" 2>"$issue_error"; then
    issue_branch=108
else
    issue_status=$?
    if [[ "$issue_status" -eq 1 ]]; then
        issue_branch=081
    else
        cat "$issue_output" "$issue_error" >&2
        fail "Issue-108 branch search failed (status $issue_status)"
    fi
fi
if [[ "$issue_branch" == 108 ]]; then
    (
        source scripts/check-effect-interchange-benchmark-108.sh
        validate_benchmark_source "$benchmark" || exit $?
        validate_successor_namespace scripts/effect-interchange-benchmark-108-validator.py || exit $?
    ) || { status=$?; fail "current Issue-108 benchmark source policy failed (status $status)"; }
else
python3 -I -B - "$benchmark" scripts/preflight-effect-interchange-benchmark.sh \
    scripts/run-effect-interchange-benchmark.sh <<'PY' || fail 'terminal Issue-081 benchmark output identities diverged'
import pathlib, re, sys
expected = [
    ("descriptor_verify_identity_a", "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1"),
    ("package_verify_cid_select_a", "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f"),
    ("state_verify_reencode_current", "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48"),
    ("migration_two_step_bank_restore", "350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441"),
]
source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
workload_block = re.search(r"const WORKLOADS:.*?= \[(.*?)\];", source, re.S)
digest_block = re.search(r"const EXPECTED_OUTPUT_SHA256:.*?= \[(.*?)\];", source, re.S)
if workload_block is None or digest_block is None:
    raise SystemExit(1)
quoted = lambda value: re.findall(r'"([^"]+)"', value)
if list(zip(quoted(workload_block.group(1)), quoted(digest_block.group(1)))) != expected:
    raise SystemExit(1)
for path_text in sys.argv[2:]:
    text = pathlib.Path(path_text).read_text(encoding="utf-8")
    for workload, digest in expected:
        if f'"{workload}":"{digest}"' not in text:
            raise SystemExit(1)
PY
fi
required_scan 'validator observation scan' rg 'OBSERVATIONS = 256' scripts/effect-interchange-benchmark-validator.py
forbidden_scan 'benchmark dependency scan' rg -n 'serde|criterion|iai|rand' tools/bench/Cargo.toml

for reference in scripts/effect-{descriptor,package,state}-v1-reference.py; do
    forbidden_scan "accepted reference child-process scan: $reference" rg -n 'subprocess|os[.]system|Popen|spawn' "$reference"
done
required_scan 'aggregator import-safe scan' rg '^if __name__ == "__main__":' scripts/effect-interchange-v1-reference.py
required_scan 'reference process bounds scan' rg 'range\(0, ?100\)|seq 0 99' scripts/run-effect-interchange-reference-processes.sh
# WP-2 deleted the print-only 10_000-trial campaign binary and its `TRIALS` constant, replacing
# it with a parametrized `campaigns(trials)` helper and a fixed-trial-count `#[test]`. The seal's
# purpose was "the mutation campaign exists and runs deterministically", not the literal 10_000;
# assert the smoke test that now stands in for it is present, still a test, and still exercises a
# fixed, non-trivial trial count.
required_scan 'mutation campaign declaration scan' rg -U -- '#\[test\]\nfn tiny_deterministic_mutation_smoke\(\) \{' crates/effect-package/tests/effect_interchange_mutation.rs
required_scan 'mutation campaign trial scan' rg -F 'campaigns(4)' crates/effect-package/tests/effect_interchange_mutation.rs
for seed in 0001 0002 0003; do
    required_scan "mutation seed scan: $seed" rg "0x081d_e5c0_0000_$seed" crates/effect-package/tests/effect_interchange_mutation.rs
done
required_scan 'exact migration matrix scan' rg 'exact_portable_migration_qualification_matrix' crates/effect-compiler/tests/migration_terminal.rs

# The real per-mode flags and target triples now live in scripts/check-cross-targets.sh (B2): the
# decorative loop that used to carry these literals in check-effect-interchange-targets.sh has been
# deleted, and this gate must police the matrix that actually runs, not a copy of its literals.
for target in x86_64-unknown-linux-gnu wasm32-unknown-unknown; do
    required_scan "target row scan: $target" rg "$target" scripts/check-cross-targets.sh
done
for feature in 'feature=-simd128' 'feature=+simd128'; do
    required_scan "Wasm target feature scan: $feature" rg -F -- "$feature" scripts/check-cross-targets.sh
done
required_scan 'Wasm Export-section scan' rg -F '/^Export\[/' scripts/check-effect-interchange-targets.sh
required_scan 'Wasm export-arrow scan' rg -F -- '-> "' scripts/check-effect-interchange-targets.sh

forbidden_scan 'production dependency scan' rg -n 'miso-engine-effect-interchange|^bench([.]workspace)?[[:space:]]*=|effect_interchange_qualification' crates/*/Cargo.toml hosts/*/Cargo.toml
forbidden_scan 'render-owned source scan' rg -n 'effect_interchange|EffectInterchange|effect_state_migration|EffectStateMigration' crates/{engine,session,graph,rack-compiler,builtins-compiler}/src
migration_scan="$scratch/migration-serialization.out"; migration_error="$scratch/migration-serialization.err"
if rg -n 'Serialize|Deserialize|serde|migration_wire|encode_migration' crates/effect-compiler/src/migration.rs >"$migration_scan" 2>"$migration_error"; then migration_status=0; else migration_status=$?; fi
if [[ "$migration_status" -gt 1 ]]; then
    cat "$migration_scan" "$migration_error" >&2
    fail "migration serialization scan failed (status $migration_status)"
fi
if [[ "$migration_status" -eq 0 ]]; then cat "$migration_scan" >&2; fail 'migration serialization scan found a prohibited match'; fi
# Anchored at the start of the attribute so the prose in `ffi.rs`'s doc comment, which names the
# attribute, is not counted as a second export (#104 phase A).
# Issue #143 added exactly one additive export, `..._inspect_observations`; the frozen
# `..._inspect` signature, its summary struct and its record layouts are untouched. The count is
# still exact, so a *third* export is still a failure.
export_scan="$scratch/exports.out"; export_error="$scratch/exports.err"
if rg -n '^[[:space:]]*#\[(unsafe\()?no_mangle' crates/effect-package/src >"$export_scan" 2>"$export_error"; then status=0; else status=$?; fi
if [[ "$status" -ne 0 ]]; then cat "$export_scan" "$export_error" >&2; fail "descriptor export scan failed (status $status)"; fi
checked_value 'descriptor export count' export_lines wc -l <"$export_scan"
checked_value 'descriptor export count filtering' exports tr -d ' ' <<<"$export_lines"
[[ "$exports" -eq 2 ]] || fail 'descriptor package gained a C export'
required_scan 'descriptor export presence scan' rg 'fn miso_engine_effect_descriptor_v1_inspect' crates/effect-package/src/ffi.rs
fixture_scan="$scratch/fixtures.out"; fixture_error="$scratch/fixtures.err"
if find fixtures/effect-interchange/v1 -mindepth 1 -maxdepth 1 -type f ! -name ACCEPTED.sha256 -print >"$fixture_scan" 2>"$fixture_error"; then status=0; else status=$?; fi
if [[ "$status" -ne 0 ]]; then cat "$fixture_scan" "$fixture_error" >&2; fail "interchange fixture traversal failed (status $status)"; fi
if [[ -s "$fixture_scan" ]]; then
    fail 'untracked/generated corpus appeared in interchange fixture directory'
fi
artifact_scan="$scratch/artifacts.out"; artifact_error="$scratch/artifacts.err"
if find . -path './target' -prune -o -type f \( -name '*.o' -o -name '*.a' -o -name '*.so' -o -name '*.dylib' -o -name '*.wasm' -o -name '*.profraw' -o -name '*.jsonl.raw' \) -print >"$artifact_scan" 2>"$artifact_error"; then status=0; else status=$?; fi
if [[ "$status" -ne 0 ]]; then cat "$artifact_scan" "$artifact_error" >&2; fail "generated artifact traversal failed (status $status)"; fi
if [[ -s "$artifact_scan" ]]; then
    fail 'generated artifact exists under a source path'
fi
for api in verify_effect_descriptor_wire verify_effect_package inspect_effect_state_selector resolve_effect_state_migration restore_scalar_effect_state_with_migration restore_unpublished_effect_bank_track_state_with_migration; do
    required_scan "public API scan: $api" rg "$api" crates/effect-{package,compiler}/src
done
printf 'effect interchange qualification policy: ok\n'
