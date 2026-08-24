#!/usr/bin/env bash
# Hermetic Issue-030 runner/promotion proof. It never invokes Cargo outside a counted scratch stub.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
runner="$root/scripts/run-graph-compiler-benchmark.sh"
promotion="$root/scripts/promote-issue006-graph-benchmark.sh"

bash -n "$runner" "$promotion" "$0"
[[ -x "$runner" && -x "$promotion" ]] || {
    printf 'graph benchmark public scripts must be executable\n' >&2
    exit 1
}
[[ "$(rg -c 'cargo run --locked --release --quiet -p miso-engine-graph-bench' "$runner")" == 1 ]] || {
    printf 'runner must contain exactly one future workload launch\n' >&2
    exit 1
}
! rg -n 'cargo run|miso-engine-graph-bench' "$promotion" >/dev/null || {
    printf 'promotion command contains a workload launch token\n' >&2
    exit 1
}
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template/scripts/fixtures" "$template/bin" "$template/target/issue6"
cp "$runner" "$promotion" "$template/scripts/"
cp "$root/scripts/graph-benchmark-record-validator.jq" \
    "$root/scripts/graph-benchmark-validator.jq" "$template/scripts/"
cp "$root/scripts/fixtures/graph-benchmark-validator-record.json" "$template/scripts/fixtures/"
jq -c -n --slurpfile record "$root/scripts/fixtures/graph-benchmark-validator-record.json" '
    $record[0] as $base
    | ($base + {
        benchmark_id: "graph_validate_65537_tracks",
        fixture_counts: {tracks: 65537, routes: 1, submixes: 0, effects: 0, sidechains: 0},
        warmup_iterations: 0,
        measured_iterations: 1,
        output_counts: ($base.output_counts + {effects: 0})
      }) as $validate
    | [$base + {benchmark_id: "graph_compile_256t_1024r_32s"},
       $base + {benchmark_id: "graph_debug_sha_dot_256t_1024r_32s"},
       $validate] as $subjects
    | [$subjects[] as $subject | [1, 2][] as $round | $subject + {round: $round}]
    | .[]
' >"$template/frozen.raw.jsonl"
[[ "$(awk 'END { print NR }' "$template/frozen.raw.jsonl")" == 6 ]] || {
    printf 'synthesized raw payload is not six records\n' >&2
    exit 1
}
jq -s -e -L "$root/scripts" -f "$root/scripts/graph-benchmark-validator.jq" \
    "$template/frozen.raw.jsonl" >/dev/null || {
    printf 'synthesized raw payload is rejected by the aggregate validator\n' >&2
    exit 1
}
expected_sha256="$(sha256sum "$template/frozen.raw.jsonl" | awk '{print $1}')"
expected_bytes="$(wc -c <"$template/frozen.raw.jsonl" | tr -d ' ')"
cat >"$template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cargo-stub\n' >>"$MISO_ENGINE_TEST_LAUNCH_LOG"
case "${MISO_ENGINE_TEST_MODE:?}" in
    success) cat "$MISO_ENGINE_TEST_FROZEN_RAW" ;;
    workload_failure) printf '{"partial":"workload"}\n'; exit 73 ;;
    interrupted_partial) printf '{"partial":"interrupted"}\n'; kill -TERM "$BASHPID" ;;
    validator_failure) printf '{}\n' ;;
    *) exit 91 ;;
esac
EOF
chmod 755 "$template/bin/cargo"

case_number=0
new_case() {
    case_number=$((case_number + 1))
    case_root="$scratch/case-$case_number-$1"
    cp -a "$template" "$case_root"
    launch_log="$case_root/launch.log"
}

run_runner() {
    local mode=$1
    MISO_ENGINE_TEST_MODE="$mode" MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" \
        MISO_ENGINE_TEST_FROZEN_RAW="$case_root/frozen.raw.jsonl" \
        PATH="$case_root/bin:$PATH" bash "$case_root/scripts/run-graph-compiler-benchmark.sh"
}

run_promotion() {
    PATH="$case_root/bin:$PATH" bash "$case_root/scripts/promote-issue006-graph-benchmark.sh"
}

expect_no_accepted() {
    [[ ! -e "$case_root/target/issue6/graph-compiler-benchmark.jsonl" ]]
    [[ ! -L "$case_root/target/issue6/graph-compiler-benchmark.jsonl" ]]
}

for argument in '--retry' '--rounds 2' extra; do
    if bash "$runner" $argument >/dev/null 2>&1; then
        printf 'runner accepted invalid arguments: %s\n' "$argument" >&2
        exit 1
    fi
    if bash "$promotion" $argument >/dev/null 2>&1; then
        printf 'promotion accepted invalid arguments: %s\n' "$argument" >&2
        exit 1
    fi
done

new_case runner-success
accepted="$(run_runner success)"
raw="$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
[[ "$accepted" == "$case_root/target/issue6/graph-compiler-benchmark.jsonl" ]]
cmp -s "$raw" "$accepted"
[[ "$(wc -l <"$launch_log")" == 1 ]]
[[ "$(sha256sum "$raw" | awk '{print $1}')" == "$expected_sha256" ]]

new_case workload-failure
set +e
run_runner workload_failure >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 73 ]]
grep -Eq "sha256=$expected_sha256|sha256=[0-9a-f]{64}" "$case_root/result"
grep -Fqx '{"partial":"workload"}' "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
expect_no_accepted
[[ "$(wc -l <"$launch_log")" == 1 ]]

new_case interrupted-partial
set +e
run_runner interrupted_partial >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 143 ]]
grep -Fqx '{"partial":"interrupted"}' "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
expect_no_accepted

new_case validator-failure
if run_runner validator_failure >"$case_root/result" 2>&1; then
    printf 'runner promoted validator-rejected output\n' >&2
    exit 1
fi
grep -Eq "sha256=[0-9a-f]{64}" "$case_root/result"
expect_no_accepted
[[ "$(wc -l <"$launch_log")" == 1 ]]

new_case existing-raw
printf 'protected\n' >"$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
if run_runner success >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" ]]
[[ "$(<"$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl")" == protected ]]

new_case existing-accepted-symlink
ln -s "$case_root/frozen.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.jsonl"
if run_runner success >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" ]]

new_case missing-validator
rm "$case_root/scripts/graph-benchmark-validator.jq"
if run_runner success >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" ]]

new_case missing-jq
mkdir -p "$case_root/no-jq"
ln -s "$(command -v dirname)" "$case_root/no-jq/dirname"
if PATH="$case_root/no-jq" /bin/bash "$case_root/scripts/run-graph-compiler-benchmark.sh" >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" ]]

new_case missing-cargo
mkdir -p "$case_root/no-cargo"
ln -s "$(command -v dirname)" "$case_root/no-cargo/dirname"
ln -s "$(command -v jq)" "$case_root/no-cargo/jq"
if PATH="$case_root/no-cargo" /bin/bash "$case_root/scripts/run-graph-compiler-benchmark.sh" >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" ]]

new_case promotion-missing-source
if run_promotion >/dev/null 2>&1; then exit 1; fi
expect_no_accepted

new_case promotion-success
cp "$case_root/frozen.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
accepted="$(run_promotion)"
raw="$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
[[ "$accepted" == "$case_root/target/issue6/graph-compiler-benchmark.jsonl" ]]
cmp -s "$raw" "$accepted"
[[ "$(sha256sum "$raw" | awk '{print $1}')" == "$expected_sha256" ]]
[[ "$(wc -c <"$accepted" | tr -d ' ')" == "$expected_bytes" ]]
[[ "$(awk 'END { print NR }' "$accepted")" == 6 ]]
[[ "$(tail -c 1 "$accepted" | od -An -t x1 | tr -d '[:space:]')" == 0a ]]

new_case promotion-truncated
head -c 10 "$case_root/frozen.raw.jsonl" >"$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
if run_promotion >/dev/null 2>&1; then exit 1; fi
expect_no_accepted

new_case promotion-appended
cp "$case_root/frozen.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
printf 'x' >>"$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
if run_promotion >/dev/null 2>&1; then exit 1; fi
expect_no_accepted

new_case promotion-validator-rejected
cp "$case_root/frozen.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
printf 'false\n' >"$case_root/scripts/graph-benchmark-validator.jq"
if run_promotion >/dev/null 2>&1; then exit 1; fi
expect_no_accepted

new_case promotion-existing-destination
cp "$case_root/frozen.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
ln "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.jsonl"
if run_promotion >/dev/null 2>&1; then exit 1; fi
cmp -s "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl" \
    "$case_root/target/issue6/graph-compiler-benchmark.jsonl"

new_case promotion-raw-symlink
ln -s "$case_root/frozen.raw.jsonl" "$case_root/target/issue6/graph-compiler-benchmark.raw.jsonl"
if run_promotion >/dev/null 2>&1; then exit 1; fi
expect_no_accepted

# Regression mutations must be detected by the exact-status and static-shape assertions above.
new_case inverted-status-mutation
sed '0,/exit "\$status"/s//exit 0/' "$case_root/scripts/run-graph-compiler-benchmark.sh" >"$case_root/mutated-runner.sh"
chmod 755 "$case_root/mutated-runner.sh"
set +e
MISO_ENGINE_TEST_MODE=workload_failure MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" \
    MISO_ENGINE_TEST_FROZEN_RAW="$case_root/frozen.raw.jsonl" PATH="$case_root/bin:$PATH" \
    bash "$case_root/mutated-runner.sh" >/dev/null 2>&1
status=$?
set -e
[[ "$status" != 73 ]]
new_case detached-if-mutation
sed -e 's/^if ! ($/if !/' \
    -e '/^    status=\$?$/,/^    exit "\$status"$/d' \
    -e 's/^); then$/then/' \
    "$case_root/scripts/run-graph-compiler-benchmark.sh" >"$case_root/mutated-runner.sh"
chmod 755 "$case_root/mutated-runner.sh"
bash -n "$case_root/mutated-runner.sh"
if MISO_ENGINE_TEST_MODE=success MISO_ENGINE_TEST_LAUNCH_LOG="$launch_log" \
    MISO_ENGINE_TEST_FROZEN_RAW="$case_root/frozen.raw.jsonl" PATH="$case_root/bin:$PATH" \
    bash "$case_root/mutated-runner.sh" >/dev/null 2>&1; then
    printf 'detached if mutation preserved the successful runner contract\n' >&2
    exit 1
fi
expect_no_accepted

printf 'graph benchmark Issue-030 hermetic lifecycle: PASS (real workload launches: 0; promotions: scratch only)\n'
