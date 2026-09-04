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

manifest=fixtures/effect-interchange/v1/ACCEPTED.sha256
accepted_manifest_sha256=e3896726979aa746cfda50fc10c1985c0ecef117f87b39e692f18226b7b4fa14
[[ -f "$manifest" && ! -L "$manifest" ]] || fail 'missing immutable baseline manifest'
[[ "$(sha256sum "$manifest" | awk '{print $1}')" == "$accepted_manifest_sha256" ]] ||
    fail 'immutable baseline manifest changed or was refreshed'
LC_ALL=C sort -c -k2,2 "$manifest" || fail 'baseline manifest is not path-sorted'
sha256sum --check --strict "$manifest" >/dev/null || fail 'accepted baseline changed'
# 24 corpus/reference-script rows plus the three `comprehensive-c` rows issue #143 added.
[[ $(wc -l <"$manifest" | tr -d ' ') -eq 27 ]] || fail 'baseline membership changed'

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
rg -q 'const OBSERVATIONS: usize = 256;' "$benchmark" ||
    fail 'benchmark observation count changed'
for workload in descriptor_verify_identity_a package_verify_cid_select_a state_verify_reencode_current migration_two_step_bank_restore; do
    rg -q "\"$workload\"" "$benchmark" || fail "benchmark workload missing: $workload"
done
rg -q 'for round in 1\.\.=2' "$benchmark" || fail 'benchmark rounds changed'
rg -Fq 'assert_eq!(records.len(), 8);' "$benchmark" || fail 'benchmark record count changed'
if rg -Fq '\"issue\":108' "$benchmark"; then
    (
        source scripts/check-effect-interchange-benchmark-108.sh
        validate_benchmark_source "$benchmark"
        validate_successor_namespace scripts/effect-interchange-benchmark-108-validator.py
    ) || fail 'current Issue-108 benchmark source policy failed'
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
rg -q 'OBSERVATIONS = 256' scripts/effect-interchange-benchmark-validator.py ||
    fail 'validator observation contract changed'
if rg -n 'serde|criterion|iai|rand' tools/bench/Cargo.toml; then
    fail 'benchmark gained a new dependency family'
fi

for reference in scripts/effect-{descriptor,package,state}-v1-reference.py; do
    if rg -n 'subprocess|os[.]system|Popen|spawn' "$reference"; then
        fail "accepted reference launches a child: $reference"
    fi
done
rg -q '^if __name__ == "__main__":' scripts/effect-interchange-v1-reference.py ||
    fail 'aggregator is not import-safe'
rg -q 'range\(0, ?100\)|seq 0 99' scripts/run-effect-interchange-reference-processes.sh ||
    fail 'runner does not freeze indexes 0..99'
# WP-2 deleted the print-only 10_000-trial campaign binary and its `TRIALS` constant, replacing
# it with a parametrized `campaigns(trials)` helper and a fixed-trial-count `#[test]`. The seal's
# purpose was "the mutation campaign exists and runs deterministically", not the literal 10_000;
# assert the smoke test that now stands in for it is present, still a test, and still exercises a
# fixed, non-trivial trial count.
rg -qU -- '#\[test\]\nfn tiny_deterministic_mutation_smoke\(\) \{' \
    crates/effect-package/tests/effect_interchange_mutation.rs ||
    fail 'mutation campaign smoke test missing or renamed'
rg -Fq 'campaigns(4)' \
    crates/effect-package/tests/effect_interchange_mutation.rs ||
    fail 'mutation campaign smoke test trial count changed'
for seed in 0001 0002 0003; do
    rg -q "0x081d_e5c0_0000_$seed" \
        crates/effect-package/tests/effect_interchange_mutation.rs ||
        fail "mutation seed $seed changed"
done
rg -q 'exact_portable_migration_qualification_matrix' \
    crates/effect-compiler/tests/migration_terminal.rs ||
    fail 'missing exact migration matrix'

# The real per-mode flags and target triples now live in scripts/check-cross-targets.sh (B2): the
# decorative loop that used to carry these literals in check-effect-interchange-targets.sh has been
# deleted, and this gate must police the matrix that actually runs, not a copy of its literals.
for target in x86_64-unknown-linux-gnu wasm32-unknown-unknown; do
    rg -q "$target" scripts/check-cross-targets.sh ||
        fail "target row missing: $target"
done
for feature in 'feature=-simd128' 'feature=+simd128'; do
    rg -Fq -- "$feature" scripts/check-cross-targets.sh ||
        fail "Wasm target feature row missing: $feature"
done
rg -Fq '/^Export\[/' scripts/check-effect-interchange-targets.sh ||
    fail 'Wasm export parser does not enter the exact Export section'
rg -Fq -- '-> "' scripts/check-effect-interchange-targets.sh ||
    fail 'Wasm export parser does not select explicit export arrows'

if rg -n 'miso-engine-effect-interchange|^bench([.]workspace)?[[:space:]]*=|effect_interchange_qualification' \
    crates/*/Cargo.toml hosts/*/Cargo.toml 2>/dev/null; then
    fail 'qualification dependency reached a production package'
fi
if rg -n 'effect_interchange|EffectInterchange|effect_state_migration|EffectStateMigration' \
    crates/{engine,session,graph,rack-compiler,builtins-compiler}/src 2>/dev/null; then
    fail 'interchange qualification or migration reached render-owned source'
fi
if rg -n 'Serialize|Deserialize|serde|migration_wire|encode_migration' \
    crates/effect-compiler/src/migration.rs; then
    fail 'migration registry serialization appeared'
fi
# Anchored at the start of the attribute so the prose in `ffi.rs`'s doc comment, which names the
# attribute, is not counted as a second export (#104 phase A).
# Issue #143 added exactly one additive export, `..._inspect_observations`; the frozen
# `..._inspect` signature, its summary struct and its record layouts are untouched. The count is
# still exact, so a *third* export is still a failure.
exports="$(rg -n '^[[:space:]]*#\[(unsafe\()?no_mangle' \
    crates/effect-package/src | wc -l | tr -d ' ')"
[[ "$exports" -eq 2 ]] || fail 'descriptor package gained a C export'
rg -q 'fn miso_engine_effect_descriptor_v1_inspect' \
    crates/effect-package/src/ffi.rs || fail 'sole descriptor export missing'
if find fixtures/effect-interchange/v1 -mindepth 1 -maxdepth 1 -type f \
    ! -name ACCEPTED.sha256 -print -quit | grep -q .; then
    fail 'untracked/generated corpus appeared in interchange fixture directory'
fi
if find . -path './target' -prune -o -type f \
    \( -name '*.o' -o -name '*.a' -o -name '*.so' -o -name '*.dylib' -o -name '*.wasm' \
       -o -name '*.profraw' -o -name '*.jsonl.raw' \) -print | grep -q .; then
    fail 'generated artifact exists under a source path'
fi
for api in verify_effect_descriptor_wire verify_effect_package inspect_effect_state_selector resolve_effect_state_migration restore_scalar_effect_state_with_migration restore_unpublished_effect_bank_track_state_with_migration; do
    rg -q "$api" crates/effect-{package,compiler}/src || fail "stale API $api"
done
printf 'effect interchange qualification policy: ok\n'
