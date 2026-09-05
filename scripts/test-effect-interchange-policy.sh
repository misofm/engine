#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/crates" "$root/hosts" "$root/fixtures" "$root/scripts" "$root/docs" \
    "$root/tools" "$temp/"

check() { bash "$temp/scripts/check-effect-interchange-qualification.sh" "$temp" >/dev/null; }
expect_failure() {
    if check 2>/dev/null; then
        printf 'effect interchange policy mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
}

check

# Every producer below is selected by its real argv and, where a command is reused, its matching
# occurrence. The wrapper first verifies the delegate's real clean status and can preserve its
# complete output before injecting a distinctive failure. This exercises the checker's consumers,
# rather than substituting synthetic successful payloads.
fault_bin="$temp/fault-bin"
mkdir -p "$fault_bin"
producer_failure() {
    local label=$1 tool=$2 needle=$3 occurrence=$4 expected=$5 mode=${6:-complete}
    local real log status
    real=$(command -v "$tool")
    find "$fault_bin" -mindepth 1 -maxdepth 1 -type f -delete
    cat >"$fault_bin/$tool" <<'SH'
#!/usr/bin/env bash
args=$(printf '%s\034' "$@")
if [[ "$args" == *"$MISO_FAULT_NEEDLE"* ]]; then
    count=0
    [[ ! -f "$MISO_FAULT_STATE" ]] || read -r count <"$MISO_FAULT_STATE"
    count=$((count + 1)); printf '%s\n' "$count" >"$MISO_FAULT_STATE"
    if [[ "$count" -eq "$MISO_FAULT_OCCURRENCE" ]]; then
        if [[ "$MISO_FAULT_MODE" == complete ]]; then
            if "$MISO_REAL_TOOL" "$@"; then delegate=0; else delegate=$?; fi
            if [[ "$delegate" -ne "$MISO_EXPECT_DELEGATE" ]]; then
                printf 'producer-wrapper-wrong-delegate expected=%s actual=%s\n' "$MISO_EXPECT_DELEGATE" "$delegate" >&2
                exit 72
            fi
        elif [[ "$MISO_FAULT_MODE" == violation ]]; then
            printf 'producer-wrapper-prohibited-row\n'
        fi
        printf 'producer-error-sentinel:%s\n' "$MISO_FAULT_LABEL" >&2
        exit 73
    fi
fi
exec "$MISO_REAL_TOOL" "$@"
SH
    chmod 755 "$fault_bin/$tool"
    : >"$temp/fault-state"
    log="$temp/fault-$label.log"
    if MISO_REAL_TOOL="$real" MISO_FAULT_NEEDLE="$needle" MISO_FAULT_OCCURRENCE="$occurrence" \
        MISO_EXPECT_DELEGATE="$expected" MISO_FAULT_MODE="$mode" MISO_FAULT_LABEL="$label" \
        MISO_FAULT_STATE="$temp/fault-state" PATH="$fault_bin:$PATH" check >"$log" 2>&1; then
        printf 'effect interchange producer failure escaped: %s\n' "$label" >&2; exit 97
    else status=$?; fi
    if [[ "$status" -ne 1 ]] || ! rg -F "producer-error-sentinel:$label" "$log" >/dev/null || \
        ! rg -F '(status 73)' "$log" >/dev/null; then
        printf 'effect interchange producer failure setup/diagnostic mismatch: %s status=%s\n' "$label" "$status" >&2
        cat "$log" >&2; exit 96
    fi
}

for mode in empty complete; do
    producer_failure manifest-sha256sum-$mode sha256sum 'fixtures/effect-interchange/v1/ACCEPTED.sha256' 1 0 "$mode"
    producer_failure manifest-awk-$mode awk '{print $1}' 1 0 "$mode"
    producer_failure manifest-sort-$mode sort 'fixtures/effect-interchange/v1/ACCEPTED.sha256' 1 0 "$mode"
    producer_failure manifest-check-$mode sha256sum '--check' 1 0 "$mode"
    producer_failure manifest-wc-$mode wc '-l' 1 0 "$mode"
    producer_failure manifest-tr-$mode tr '-d' 1 0 "$mode"
    producer_failure export-rg-$mode rg 'no_mangle' 1 0 "$mode"
    producer_failure export-wc-$mode wc '-l' 2 0 "$mode"
    producer_failure export-tr-$mode tr '-d' 2 0 "$mode"
done

for row in \
    'observations|rg|const OBSERVATIONS|1|0' \
    'workload|rg|descriptor_verify_identity_a|1|0' \
    'rounds|rg|for round in|1|0' \
    'records|rg|records.len|1|0' \
    'import-safe|rg|__main__|1|0' \
    'reference-bounds|rg|run-effect-interchange-reference-processes.sh|1|0' \
    'multiline-campaign|rg|tiny_deterministic_mutation_smoke|1|0' \
    'campaign-count|rg|campaigns(4)|1|0' \
    'seed|rg|0x081d_e5c0_0000_0001|1|0' \
    'matrix|rg|exact_portable_migration|1|0' \
    'target|rg|x86_64-unknown-linux-gnu|1|0' \
    'simd|rg|feature=+simd128|1|0' \
    'export-syntax|rg|/^Export|1|0' \
    'export-presence|rg|fn miso_engine_effect_descriptor_v1_inspect|1|0' \
    'late-api|rg|restore_unpublished_effect_bank_track_state_with_migration|1|0'; do
    IFS='|' read -r label tool needle occurrence expected <<<"$row"
    producer_failure "$label" "$tool" "$needle" "$occurrence" "$expected" complete
done

producer_failure dependency-error rg 'Cargo.toml' 1 1 empty
producer_failure reference-error rg 'effect-descriptor-v1-reference.py' 1 1 empty
producer_failure render-error rg 'rack-compiler' 1 1 empty
producer_failure fixture-find-error find 'fixtures/effect-interchange/v1' 1 0 empty
producer_failure fixture-find-violation-error find 'fixtures/effect-interchange/v1' 1 0 violation
producer_failure artifact-find-error find './target' 1 0 empty
producer_failure artifact-find-violation-error find './target' 1 0 violation
producer_failure issue-branch-error rg '\"issue\":108' 1 0 complete
producer_failure source-python-error python3 'tools/bench/src/effect_interchange.rs' 1 0 complete

# The late migration scan has its own status guard so this causal control mutates only that guard.
producer_failure migration-original rg 'migration_wire' 1 1 empty
migration_mutant="$temp/check-effect-interchange-qualification-migration-mutant.sh"
cp "$temp/scripts/check-effect-interchange-qualification.sh" "$migration_mutant"
sed -i 's/if \[\[ "$migration_status" -gt 1 \]\]; then/if [[ "$migration_status" -gt 73 ]]; then/' "$migration_mutant"
assert_migration_error() {
    local checker=$1 log="$temp/migration-control.log" status
    : >"$temp/fault-state"
    if MISO_REAL_TOOL="$(command -v rg)" MISO_FAULT_NEEDLE=migration_wire MISO_FAULT_OCCURRENCE=1 \
        MISO_EXPECT_DELEGATE=1 MISO_FAULT_MODE=empty MISO_FAULT_LABEL=migration-control \
        MISO_FAULT_STATE="$temp/fault-state" PATH="$fault_bin:$PATH" bash "$checker" "$temp" >"$log" 2>&1; then status=0; else status=$?; fi
    [[ "$status" -ne 0 ]] || return 97
    [[ "$status" -eq 1 ]] || return 96
    rg -F producer-error-sentinel:migration-control "$log" >/dev/null || return 96
    rg -F '(status 73)' "$log" >/dev/null || return 96
}
producer_failure migration-restored rg 'migration_wire' 1 1 empty
assert_migration_error "$temp/scripts/check-effect-interchange-qualification.sh" || exit $?

# Required inputs remain fail-closed after the otherwise-valid fixture tree has passed.
mv "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256" "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256.saved"
expect_failure missing-manifest
mv "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256.saved" "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256"
mv "$temp/tools/bench/src/effect_interchange.rs" "$temp/tools/bench/src/effect_interchange.rs.saved"
expect_failure missing-benchmark-source
mv "$temp/tools/bench/src/effect_interchange.rs.saved" "$temp/tools/bench/src/effect_interchange.rs"
mv "$temp/crates/effect-compiler/src" "$temp/crates/effect-compiler/src.saved"
expect_failure missing-required-root
mv "$temp/crates/effect-compiler/src.saved" "$temp/crates/effect-compiler/src"
if assert_migration_error "$migration_mutant"; then
    printf 'effect interchange qualification policy: migration status-loss mutant did not escape\n' >&2
    exit 96
else
    mutation_status=$?
    [[ "$mutation_status" -eq 97 ]] || exit 96
fi
assert_migration_error "$temp/scripts/check-effect-interchange-qualification.sh" || exit $?
printf 'changed\n' >>"$temp/fixtures/effect-state/v1/canonical.state.hex"
expect_failure baseline
cp "$root/fixtures/effect-state/v1/canonical.state.hex" \
    "$temp/fixtures/effect-state/v1/canonical.state.hex"

printf 'changed\n' >>"$temp/fixtures/effect-state/v1/canonical.state.hex"
replacement="$(sha256sum "$temp/fixtures/effect-state/v1/canonical.state.hex" | awk '{print $1}')"
sed -i \
    "s/^4d00a6c3661d119dcf62d16e6c72a68a5f12283397610cfcf18ece7471a2b014 /$replacement /" \
    "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256"
expect_failure refreshed-baseline
cp "$root/fixtures/effect-state/v1/canonical.state.hex" \
    "$temp/fixtures/effect-state/v1/canonical.state.hex"
cp "$root/fixtures/effect-interchange/v1/ACCEPTED.sha256" \
    "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256"

printf '\nbench.workspace = true\n' \
    >>"$temp/crates/engine/Cargo.toml"
expect_failure production-dependency
cp "$root/crates/engine/Cargo.toml" "$temp/crates/engine/Cargo.toml"

printf '\npub fn effect_interchange_qualification_render_leak() {}\n' \
    >>"$temp/crates/engine/src/realtime/plan.rs"
expect_failure render-reachability
cp "$root/crates/engine/src/realtime/plan.rs" \
    "$temp/crates/engine/src/realtime/plan.rs"

printf '\n#[unsafe(no_mangle)] pub extern "C" fn miso_engine_effect_state_v1_new_export() {}\n' \
    >>"$temp/crates/effect-package/src/ffi.rs"
expect_failure new-export
cp "$root/crates/effect-package/src/ffi.rs" \
    "$temp/crates/effect-package/src/ffi.rs"

sed -i 's/campaigns(4)/campaigns(3)/' \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"
expect_failure mutation-count
cp "$root/crates/effect-package/tests/effect_interchange_mutation.rs" \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"

sed -i 's/fn tiny_deterministic_mutation_smoke/fn tiny_deterministic_mutation_smoke_renamed/' \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"
expect_failure mutation-smoke-renamed
cp "$root/crates/effect-package/tests/effect_interchange_mutation.rs" \
    "$temp/crates/effect-package/tests/effect_interchange_mutation.rs"

sed -i 's/const OBSERVATIONS: usize = 256/const OBSERVATIONS: usize = 255/' \
    "$temp/tools/bench/src/effect_interchange.rs"
expect_failure benchmark-observations
cp "$root/tools/bench/src/effect_interchange.rs" \
    "$temp/tools/bench/src/effect_interchange.rs"

# B2: the qualification gate must police the real matrix in check-cross-targets.sh, not a
# decorative copy of its literals -- dropping a required target triple must fail. #378 retired the
# aarch64 rows this used to mutate (owner ruling: native AArch64 is unsupported, no claim); the
# equivalent mutation on a remaining target is erasing wasm32-unknown-unknown everywhere it
# appears in the real matrix, which the qualification gate's target-row loop must still catch.
sed -i 's/wasm32-unknown-unknown/wasm-target-erased/g' \
    "$temp/scripts/check-cross-targets.sh"
expect_failure cross-target-dropped-wasm
cp "$root/scripts/check-cross-targets.sh" "$temp/scripts/check-cross-targets.sh"

# B2: turning the Wasm simd leg scalar (dropping the +simd128 feature row from the real matrix)
# must also fail.
sed -i 's/feature=+simd128/feature=-simd128/' \
    "$temp/scripts/check-cross-targets.sh"
expect_failure cross-target-simd-leg-scalar
cp "$root/scripts/check-cross-targets.sh" "$temp/scripts/check-cross-targets.sh"

printf 'effect interchange qualification policy mutations: ok\n'

# Required CI reaches the successor authority and the hermetic lifecycle from this original
# repository root. Both suites use their own scratch roots; neither is run against a mutation.
bash "$root/scripts/test-effect-interchange-benchmark-108-policy.sh"
bash "$root/scripts/test-effect-interchange-benchmark.sh"
