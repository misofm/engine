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

# Exercise the clean no-match 081 branch with its original terminal migration digest.
cp "$temp/tools/bench/src/effect_interchange.rs" "$temp/tools/bench/src/effect_interchange.rs.current108"
sed -i \
    -e 's/\\"issue\\":108/\\"issue\\":81/' \
    -e 's/5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777/350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441/' \
    "$temp/tools/bench/src/effect_interchange.rs"
check
mv "$temp/tools/bench/src/effect_interchange.rs.current108" "$temp/tools/bench/src/effect_interchange.rs"

# Every producer below is selected by its real argv and, where a command is reused, its matching
# occurrence. The wrapper first verifies the delegate's real clean status and can preserve its
# complete output before injecting a distinctive failure. This exercises the checker's consumers,
# rather than substituting synthetic successful payloads.
fault_bin="$temp/fault-bin"
mkdir -p "$fault_bin"
producer_failure() {
    local label=$1 tool=$2 needle=$3 occurrence=$4 expected=$5 mode=$6 operation=$7 shape=$8
    local real log status diagnostic
    diagnostic=${9:-"effect interchange qualification policy failure: $operation failed (status 73)"}
    diagnostic+=$'\n'
    real=$(command -v "$tool")
    find "$fault_bin" -mindepth 1 -maxdepth 1 -type f -delete
    cat >"$fault_bin/$tool" <<'SH'
#!/usr/bin/env bash
args=$(printf '%s\034' "$@")
if [[ "$args" == *"$MISO_ENGINE_INTERCHANGE_TEST_FAULT_NEEDLE"* ]]; then
    count=0
    [[ ! -f "$MISO_ENGINE_INTERCHANGE_TEST_FAULT_STATE" ]] || read -r count <"$MISO_ENGINE_INTERCHANGE_TEST_FAULT_STATE"
    count=$((count + 1)); printf '%s\n' "$count" >"$MISO_ENGINE_INTERCHANGE_TEST_FAULT_STATE"
    if [[ "$count" -eq "$MISO_ENGINE_INTERCHANGE_TEST_FAULT_OCCURRENCE" ]]; then
        if "$MISO_ENGINE_INTERCHANGE_TEST_REAL_TOOL" "$@" >"$MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT" 2>"$MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_ERROR"; then delegate=0; else delegate=$?; fi
        if [[ "$delegate" -ne "$MISO_ENGINE_INTERCHANGE_TEST_EXPECT_DELEGATE" ]]; then
            printf 'producer-wrapper-wrong-delegate expected=%s actual=%s\n' "$MISO_ENGINE_INTERCHANGE_TEST_EXPECT_DELEGATE" "$delegate" >&2
            exit 72
        fi
        [[ "$MISO_ENGINE_INTERCHANGE_TEST_OUTPUT_SHAPE" != nonempty || -s "$MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT" ]] || { printf 'producer-wrapper-empty-delegate\n' >&2; exit 72; }
        [[ "$MISO_ENGINE_INTERCHANGE_TEST_OUTPUT_SHAPE" != empty || ! -s "$MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT" ]] || { printf 'producer-wrapper-nonempty-delegate\n' >&2; exit 72; }
        cat "$MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_ERROR" >&2
        [[ "$MISO_ENGINE_INTERCHANGE_TEST_FAULT_MODE" != complete ]] || cat "$MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT"
        printf 'producer-error-sentinel:%s\n' "$MISO_ENGINE_INTERCHANGE_TEST_FAULT_LABEL" >&2
        exit 73
    fi
fi
exec "$MISO_ENGINE_INTERCHANGE_TEST_REAL_TOOL" "$@"
SH
    chmod 755 "$fault_bin/$tool"
    : >"$temp/fault-state"
    log="$temp/fault-$label.log"
    if MISO_ENGINE_INTERCHANGE_TEST_REAL_TOOL="$real" MISO_ENGINE_INTERCHANGE_TEST_FAULT_NEEDLE="$needle" MISO_ENGINE_INTERCHANGE_TEST_FAULT_OCCURRENCE="$occurrence" \
        MISO_ENGINE_INTERCHANGE_TEST_EXPECT_DELEGATE="$expected" MISO_ENGINE_INTERCHANGE_TEST_FAULT_MODE="$mode" MISO_ENGINE_INTERCHANGE_TEST_FAULT_LABEL="$label" \
        MISO_ENGINE_INTERCHANGE_TEST_OUTPUT_SHAPE="$shape" MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT="$temp/delegate-output" MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_ERROR="$temp/delegate-error" \
        MISO_ENGINE_INTERCHANGE_TEST_FAULT_STATE="$temp/fault-state" PATH="$fault_bin:$PATH" check >"$log" 2>&1; then
        printf 'effect interchange producer failure escaped: %s\n' "$label" >&2; exit 97
    else status=$?; fi
    if [[ "$status" -ne 1 ]] || ! rg -F "producer-error-sentinel:$label" "$log" >/dev/null || \
        ! rg -F '(status 73)' "$log" >/dev/null || ! rg -F "$operation" "$log" >/dev/null; then
        printf 'effect interchange producer failure setup/diagnostic mismatch: %s status=%s\n' "$label" "$status" >&2
        cat "$log" >&2; exit 96
    fi
    expected_payload="$temp/delegate-$label.out"
    if [[ "$mode" == complete ]]; then
        cp "$temp/delegate-output" "$expected_payload"
    else
        : >"$expected_payload"
    fi
    expected_error="$temp/delegate-$label.err"
    cp "$temp/delegate-error" "$expected_error"
    payload_assertion="$temp/payload-assertion.py"
    if [[ ! -f "$payload_assertion" ]]; then
        cat >"$payload_assertion" <<'PY'
import collections, pathlib, sys
expected_path, log_path, delegate_error_path, label, diagnostic = sys.argv[1:]
expected = pathlib.Path(expected_path).read_bytes()
log = pathlib.Path(log_path).read_bytes()
delegate_error = pathlib.Path(delegate_error_path).read_bytes()
sentinel = f"producer-error-sentinel:{label}\n".encode()
diagnostic = diagnostic.encode()
if not diagnostic.endswith(b"\n"):
    raise SystemExit("producer operation/status framing mismatch")
tail = sentinel + diagnostic
if log.count(sentinel) != 1 or log.count(diagnostic) != 1 or not log.endswith(tail):
    raise SystemExit(
        f"producer diagnostic framing mismatch: {expected_path}: tail={log[-len(tail):]!r}"
    )
prefix = log[:-len(tail)]
if delegate_error and (prefix.count(delegate_error) != 1 or not prefix.endswith(delegate_error)):
    raise SystemExit("producer stderr framing mismatch")
payload = prefix[:-len(delegate_error)] if delegate_error else prefix
if collections.Counter(payload.splitlines(keepends=True)) != collections.Counter(expected.splitlines(keepends=True)):
    raise SystemExit("complete producer payload did not match exactly")
PY
    fi
    python3 -I -B "$payload_assertion" "$expected_payload" "$log" "$expected_error" \
        "$label" "$diagnostic" || exit 96
}

for mode in empty complete; do
    producer_failure manifest-sha256sum-$mode sha256sum 'fixtures/effect-interchange/v1/ACCEPTED.sha256' 1 0 "$mode" 'manifest hash production' nonempty
    producer_failure manifest-awk-$mode awk '{print $1}' 1 0 "$mode" 'manifest hash extraction' nonempty
    producer_failure manifest-sort-$mode sort 'fixtures/effect-interchange/v1/ACCEPTED.sha256' 1 0 "$mode" 'baseline manifest sort' empty
    producer_failure manifest-check-$mode sha256sum '--check' 1 0 "$mode" 'accepted baseline check' nonempty
    producer_failure manifest-wc-$mode wc '-l' 1 0 "$mode" 'manifest line count' nonempty
    producer_failure manifest-tr-$mode tr '-d' 1 0 "$mode" 'manifest line count filtering' nonempty
    producer_failure export-rg-$mode rg 'no_mangle' 1 0 "$mode" 'descriptor export scan' nonempty
    producer_failure export-wc-$mode wc '-l' 2 0 "$mode" 'descriptor export count' nonempty
    producer_failure export-tr-$mode tr '-d' 2 0 "$mode" 'descriptor export count filtering' nonempty
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
    case "$label" in
        observations) operation='benchmark observation count scan' ;;
        workload) operation='benchmark workload scan: descriptor_verify_identity_a' ;;
        rounds) operation='benchmark rounds scan' ;;
        records) operation='benchmark record count scan' ;;
        import-safe) operation='aggregator import-safe scan' ;;
        reference-bounds) operation='reference process bounds scan' ;;
        multiline-campaign) operation='mutation campaign declaration scan' ;;
        campaign-count) operation='mutation campaign trial scan' ;;
        seed) operation='mutation seed scan: 0001' ;;
        matrix) operation='exact migration matrix scan' ;;
        target) operation='target row scan: x86_64-unknown-linux-gnu' ;;
        simd) operation='Wasm target feature scan: feature=+simd128' ;;
        export-syntax) operation='Wasm Export-section scan' ;;
        export-presence) operation='descriptor export presence scan' ;;
        late-api) operation='public API scan: restore_unpublished_effect_bank_track_state_with_migration' ;;
    esac
    for mode in empty complete; do
        producer_failure "$label-$mode" "$tool" "$needle" "$occurrence" "$expected" "$mode" "$operation" nonempty
    done
done

# Apply the same assertion to bounded duplicate, extra and missing payload controls. This uses a
# real nonempty capture and its real stderr/sentinel/status framing.
producer_failure export-rg-complete-controls rg 'no_mangle' 1 0 complete 'descriptor export scan' nonempty
payload_expected="$temp/delegate-export-rg-complete-controls.out"
payload_error="$temp/delegate-export-rg-complete-controls.err"
payload_assertion="$temp/payload-assertion.py"
payload_valid_log="$temp/fault-export-rg-complete-controls.log"
payload_diagnostic=$'effect interchange qualification policy failure: descriptor export scan failed (status 73)\n'
python3 -I -B "$payload_assertion" "$payload_expected" "$payload_valid_log" "$payload_error" \
    export-rg-complete-controls "$payload_diagnostic" || exit 96
payload_control() {
    local label=$1 actual_log=$2 assertion_log="$temp/payload-$1-assertion.log" status
    if python3 -I -B "$payload_assertion" "$payload_expected" "$actual_log" "$payload_error" \
        export-rg-complete-controls "$payload_diagnostic" >"$assertion_log" 2>&1; then
        printf 'effect interchange payload control unexpectedly passed: %s\n' "$label" >&2
        exit 96
    else
        status=$?
    fi
    if [[ "$status" -ne 1 ]] || ! rg -Fx 'complete producer payload did not match exactly' "$assertion_log" >/dev/null; then
        printf 'effect interchange payload control setup/framing mismatch: %s status=%s\n' "$label" "$status" >&2
        cat "$assertion_log" >&2
        exit 96
    fi
}
python3 -I -B - "$payload_valid_log" "$payload_error" "$payload_diagnostic" "$temp" <<'PY'
import pathlib, sys
log_path, error_path, diagnostic, output_dir = sys.argv[1:]
log = pathlib.Path(log_path).read_bytes()
error = pathlib.Path(error_path).read_bytes()
tail = error + b"producer-error-sentinel:export-rg-complete-controls\n" + diagnostic.encode()
if not log.endswith(tail):
    raise SystemExit("payload control source framing mismatch")
payload = log[:-len(tail)]
rows = payload.splitlines(keepends=True)
if not rows:
    raise SystemExit("payload control source was empty")
variants = {
    "duplicate": rows + rows[:1],
    "extra": rows + [b"payload-extra-row\n"],
    "missing": rows[1:],
    "reversed": list(reversed(rows)),
}
for name, variant in variants.items():
    pathlib.Path(output_dir, f"payload-{name}.log").write_bytes(b"".join(variant) + tail)
PY
payload_control duplicate "$temp/payload-duplicate.log"
payload_control extra "$temp/payload-extra.log"
payload_control missing "$temp/payload-missing.log"
if [[ "$(wc -l <"$payload_expected")" -gt 1 ]]; then
    python3 -I -B "$payload_assertion" "$payload_expected" "$temp/payload-reversed.log" "$payload_error" \
        export-rg-complete-controls "$payload_diagnostic" || exit 96
fi

producer_failure dependency-error rg 'tools/bench/Cargo.toml' 1 1 empty 'benchmark dependency scan' empty
producer_failure production-manifest-error rg 'hosts/' 1 1 empty 'production dependency scan' empty
producer_failure reference-error rg 'effect-state-v1-reference.py' 1 1 empty \
    'accepted reference child-process scan: scripts/effect-state-v1-reference.py' empty
producer_failure render-error rg 'rack-compiler' 2 1 empty 'render-owned source scan' empty
producer_failure fixture-find-error find 'fixtures/effect-interchange/v1' 1 0 empty 'interchange fixture traversal' empty
printf 'real fixture violation\n' >"$temp/fixtures/effect-interchange/v1/actual-violation.tmp"
producer_failure fixture-find-violation-error find 'fixtures/effect-interchange/v1' 1 0 complete 'interchange fixture traversal' nonempty
rm "$temp/fixtures/effect-interchange/v1/actual-violation.tmp"
producer_failure artifact-find-error find './target' 1 0 empty 'generated artifact traversal' empty
printf 'real artifact violation\n' >"$temp/actual-violation.o"
producer_failure artifact-find-violation-error find './target' 1 0 complete 'generated artifact traversal' nonempty
rm "$temp/actual-violation.o"
producer_failure issue-branch-error-empty rg '\"issue\":108' 1 0 empty 'Issue-108 branch search' nonempty
producer_failure issue-branch-error-complete rg '\"issue\":108' 1 0 complete 'Issue-108 branch search' nonempty
producer_failure source-python-error python3 'tools/bench/src/effect_interchange.rs' 1 0 complete 'current Issue-108 benchmark source policy' empty

cp "$temp/tools/bench/src/effect_interchange.rs" "$temp/tools/bench/src/effect_interchange.rs.current108"
sed -i \
    -e 's/\\"issue\\":108/\\"issue\\":81/' \
    -e 's/5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777/350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441/' \
    "$temp/tools/bench/src/effect_interchange.rs"
producer_failure issue081-python-error python3 'tools/bench/src/effect_interchange.rs' 1 0 complete \
    'terminal Issue-081 benchmark output identities diverged' empty \
    'effect interchange qualification policy failure: terminal Issue-081 benchmark output identities diverged (status 73)'
chmod 000 "$temp/scripts/preflight-effect-interchange-benchmark.sh"
expect_precise_081_log="$temp/issue081-read-error.log"
if check >"$expect_precise_081_log" 2>&1; then printf 'effect interchange Issue-081 read fault unexpectedly succeeded\n' >&2; exit 97; fi
rg -F 'terminal Issue-081 benchmark output identities diverged (status 1)' "$expect_precise_081_log" >/dev/null || exit 96
rg -F 'preflight-effect-interchange-benchmark.sh' "$expect_precise_081_log" >/dev/null || exit 96
chmod 755 "$temp/scripts/preflight-effect-interchange-benchmark.sh"
mv "$temp/tools/bench/src/effect_interchange.rs.current108" "$temp/tools/bench/src/effect_interchange.rs"

# The late migration scan has its own status guard so this causal control mutates only that guard.
producer_failure migration-original rg 'migration_wire' 1 1 empty 'migration serialization scan' empty
migration_mutant="$temp/check-effect-interchange-qualification-migration-mutant.sh"
cp "$temp/scripts/check-effect-interchange-qualification.sh" "$migration_mutant"
sed -i 's/if \[\[ "$migration_status" -gt 1 \]\]; then/if [[ "$migration_status" -gt 73 ]]; then/' "$migration_mutant"
[[ "$(rg -F -c 'if [[ "$migration_status" -gt 1 ]]; then' "$temp/scripts/check-effect-interchange-qualification.sh")" -eq 1 ]] || exit 96
[[ "$(rg -F -c 'if [[ "$migration_status" -gt 73 ]]; then' "$migration_mutant")" -eq 1 ]] || exit 96
[[ "$(rg -F -c 'if [[ "$migration_status" -gt 1 ]]; then' "$migration_mutant")" -eq 0 ]] || exit 96
assert_migration_error() {
    local checker=$1 log="$temp/migration-control.log" status
    : >"$temp/fault-state"
    if MISO_ENGINE_INTERCHANGE_TEST_REAL_TOOL="$(command -v rg)" MISO_ENGINE_INTERCHANGE_TEST_FAULT_NEEDLE=migration_wire MISO_ENGINE_INTERCHANGE_TEST_FAULT_OCCURRENCE=1 \
        MISO_ENGINE_INTERCHANGE_TEST_EXPECT_DELEGATE=1 MISO_ENGINE_INTERCHANGE_TEST_FAULT_MODE=empty MISO_ENGINE_INTERCHANGE_TEST_FAULT_LABEL=migration-control \
        MISO_ENGINE_INTERCHANGE_TEST_OUTPUT_SHAPE=empty MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_OUTPUT="$temp/migration-delegate-output" MISO_ENGINE_INTERCHANGE_TEST_DELEGATE_ERROR="$temp/migration-delegate-error" \
        MISO_ENGINE_INTERCHANGE_TEST_FAULT_STATE="$temp/fault-state" PATH="$fault_bin:$PATH" bash "$checker" "$temp" >"$log" 2>&1; then status=0; else status=$?; fi
    if [[ "$status" -eq 0 ]]; then
        printf 'effect interchange qualification policy: migration status-loss unexpectedly succeeded\n' >&2
        return 97
    fi
    [[ "$status" -eq 1 ]] || return 96
    rg -F producer-error-sentinel:migration-control "$log" >/dev/null || return 96
    rg -F '(status 73)' "$log" >/dev/null || return 96
    rg -F 'migration serialization scan' "$log" >/dev/null || return 96
}
producer_failure migration-restored rg 'migration_wire' 1 1 empty 'migration serialization scan' empty
assert_migration_error "$temp/scripts/check-effect-interchange-qualification.sh" || exit $?

# Required inputs remain fail-closed after the otherwise-valid fixture tree has passed.
expect_precise_failure() {
    local label=$1 diagnostic=$2 log="$temp/precise-$1.log" status
    if check >"$log" 2>&1; then
        printf 'effect interchange precise mutation unexpectedly succeeded: %s\n' "$label" >&2; exit 97
    else status=$?; fi
    [[ "$status" -eq 1 ]] || { printf 'effect interchange precise mutation wrong status: %s=%s\n' "$label" "$status" >&2; exit 96; }
    rg -F "$diagnostic" "$log" >/dev/null || { cat "$log" >&2; exit 96; }
}
mv "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256" "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256.saved"
expect_precise_failure missing-manifest 'missing immutable baseline manifest'
mv "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256.saved" "$temp/fixtures/effect-interchange/v1/ACCEPTED.sha256"
mv "$temp/tools/bench/src/effect_interchange.rs" "$temp/tools/bench/src/effect_interchange.rs.saved"
expect_precise_failure missing-benchmark-source 'missing qualification path tools/bench/src/effect_interchange.rs'
mv "$temp/tools/bench/src/effect_interchange.rs.saved" "$temp/tools/bench/src/effect_interchange.rs"
mv "$temp/crates/effect-compiler/src" "$temp/crates/effect-compiler/src.saved"
expect_precise_failure missing-required-root 'migration serialization scan failed (status 2)'
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
