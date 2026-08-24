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
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
cd "$root"
fail() { printf 'effect interchange qualification policy failure: %s\n' "$1" >&2; exit 1; }

manifest=fixtures/effect-interchange/v1/ACCEPTED.sha256
accepted_manifest_sha256=1aaa96dc731c0da3dabb2f8ecd7c2bf803078b580a38cccfccf1ffe280c83588
[[ -f "$manifest" && ! -L "$manifest" ]] || fail 'missing immutable baseline manifest'
[[ "$(sha256sum "$manifest" | awk '{print $1}')" == "$accepted_manifest_sha256" ]] ||
    fail 'immutable baseline manifest changed or was refreshed'
LC_ALL=C sort -c -k2,2 "$manifest" || fail 'baseline manifest is not path-sorted'
sha256sum --check --strict "$manifest" >/dev/null || fail 'accepted baseline changed'
[[ $(wc -l <"$manifest" | tr -d ' ') -eq 24 ]] || fail 'baseline membership changed'

for path in \
    scripts/effect-interchange-v1-reference.py \
    scripts/run-effect-interchange-reference-processes.sh \
    scripts/test-effect-interchange-reference-runner.sh \
    scripts/check-effect-interchange-targets.sh \
    scripts/test-effect-interchange-target-export-parser.sh \
    scripts/effect-interchange-benchmark-validator.py \
    scripts/effect-interchange-benchmark-108-validator.py \
    scripts/check-effect-interchange-benchmark-108.sh \
    scripts/preflight-effect-interchange-benchmark.sh \
    scripts/run-effect-interchange-benchmark.sh \
    scripts/test-effect-interchange-benchmark.sh \
    tools/miso-engine-bench/Cargo.toml \
    tools/miso-engine-bench/src/effect_interchange.rs \
    docs/EFFECT_INTERCHANGE_QUALIFICATION_V1.md; do
    [[ -f "$path" ]] || fail "missing qualification path $path"
done
benchmark=tools/miso-engine-bench/src/effect_interchange.rs
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
if rg -n 'serde|criterion|iai|rand' tools/miso-engine-bench/Cargo.toml; then
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
rg -q 'TRIALS: usize = 10_000' \
    crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs ||
    fail 'mutation trial count changed'
for seed in 0001 0002 0003; do
    rg -q "0x081d_e5c0_0000_$seed" \
        crates/miso-engine-effect-package/tests/effect_interchange_mutation.rs ||
        fail "mutation seed $seed changed"
done
rg -q 'exact_portable_migration_qualification_matrix' \
    crates/miso-engine-effect-compiler/tests/migration_terminal.rs ||
    fail 'missing exact migration matrix'

for target in x86_64-unknown-linux-gnu aarch64-linux-android aarch64-apple-ios wasm32-unknown-unknown; do
    rg -q "$target" scripts/check-effect-interchange-targets.sh ||
        fail "target row missing: $target"
done
for feature in 'feature=-simd128' 'feature=+simd128'; do
    rg -Fq -- "$feature" scripts/check-effect-interchange-targets.sh ||
        fail "Wasm target feature row missing: $feature"
done
rg -Fq '/^Export\[/' scripts/check-effect-interchange-targets.sh ||
    fail 'Wasm export parser does not enter the exact Export section'
rg -Fq -- '-> "' scripts/check-effect-interchange-targets.sh ||
    fail 'Wasm export parser does not select explicit export arrows'

if rg -n 'miso-engine-effect-interchange|effect_interchange_qualification' \
    crates/*/Cargo.toml hosts/*/Cargo.toml 2>/dev/null; then
    fail 'qualification dependency reached a production package'
fi
if rg -n 'effect_interchange|EffectInterchange|effect_state_migration|EffectStateMigration' \
    crates/miso-engine-{core,session,graph,rack-compiler,builtins-compiler}/src 2>/dev/null; then
    fail 'interchange qualification or migration reached render-owned source'
fi
if rg -n 'Serialize|Deserialize|serde|migration_wire|encode_migration' \
    crates/miso-engine-effect-compiler/src/migration.rs; then
    fail 'migration registry serialization appeared'
fi
# Anchored at the start of the attribute so the prose in `ffi.rs`'s doc comment, which names the
# attribute, is not counted as a second export (#104 phase A).
exports="$(rg -n '^[[:space:]]*#\[(unsafe\()?no_mangle' \
    crates/miso-engine-effect-package/src | wc -l | tr -d ' ')"
[[ "$exports" -eq 1 ]] || fail 'descriptor package gained a C export'
rg -q 'fn miso_engine_effect_descriptor_v1_inspect' \
    crates/miso-engine-effect-package/src/ffi.rs || fail 'sole descriptor export missing'
if find fixtures/effect-interchange/v1 -mindepth 1 -maxdepth 1 -type f \
    ! -name ACCEPTED.sha256 -print -quit | grep -q .; then
    fail 'untracked/generated corpus appeared in interchange fixture directory'
fi
if find . -path './target' -prune -o -type f \
    \( -name '*.o' -o -name '*.a' -o -name '*.so' -o -name '*.dylib' -o -name '*.wasm' \
       -o -name '*.profraw' -o -name '*.jsonl.raw' \) -print | grep -q .; then
    fail 'generated artifact exists under a source path'
fi
for api in verify_effect_descriptor_wire_v1 verify_effect_package_v1 inspect_effect_state_selector_v1 resolve_effect_state_migration_v1 restore_scalar_effect_state_with_migration_v1 restore_unpublished_effect_bank_track_state_with_migration_v1; do
    rg -q "$api" crates/miso-engine-effect-{package,compiler}/src || fail "stale API $api"
done
printf 'effect interchange qualification policy: ok\n'
