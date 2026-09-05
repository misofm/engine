#!/usr/bin/env bash
# Hermetic mutations for the Issue-073 dependency and fixture checker.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT

copy_case() {
    case_root="$temp/$1"
    mkdir -p "$case_root/tools" "$case_root/fixtures/native-pcm-runner" \
        "$case_root/fixtures/session/v1" "$case_root/scripts" "$case_root/crates/fake/src" \
        "$case_root/hosts" "$case_root/sidecars"
    cp -R "$root/tools/native-pcm-runner" "$case_root/tools/"
    cp -R "$root/fixtures/native-pcm-runner/v1" "$case_root/fixtures/native-pcm-runner/"
    cp "$root/fixtures/session/v1/parametric-eq-nine-track.json" "$case_root/fixtures/session/v1/"
    cp "$root/scripts/check-native-pcm-runner.sh" "$case_root/scripts/"
}

copy_case baseline
"$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null

# Empty roots and populations containing only the two documented exclusions are valid.
copy_case empty-roots
rm -rf -- "$case_root/crates/fake"; mkdir -p "$case_root/crates"
"$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null
copy_case allowed-populations
printf '\n// native-pcm-runner is intentionally native-only\n' >>"$case_root/hosts/allowed.rs"
"$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null

copy_case fixture-drift
printf mutation >>"$case_root/fixtures/native-pcm-runner/v1/riff-48000.wav"
if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: fixture drift\n' >&2
    exit 1
fi

copy_case bypass
printf '\ngraph.workspace = true\n' >>"$case_root/tools/native-pcm-runner/Cargo.toml"
if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: graph bypass\n' >&2
    exit 1
fi

copy_case reverse-dependency
cat >"$case_root/crates/fake/Cargo.toml" <<'EOF'
[package]
name = "miso-engine-fake"
version = "0.0.0"
[dependencies]
native-pcm-runner = { path = "../../tools/native-pcm-runner" }
EOF
if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 >/dev/null 2>&1; then
    printf 'native PCM runner mutation escaped: reverse dependency\n' >&2
    exit 1
fi

# Required roots are a checked population: each may be empty, but a missing root is an error.
for required_root in crates hosts sidecars; do
    copy_case "missing-$required_root"
    rm -rf -- "$case_root/$required_root"
    output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
    [[ "$status" != 0 ]] || { printf 'unexpected success: missing required root escaped: %s\n' "$required_root" >&2; exit 97; }
    [[ "$output" == *"reachability root is missing: $required_root"* ]] || { printf 'missing root wrong/setup outcome: %s\n%s\n' "$required_root" "$output" >&2; exit 96; }
done
copy_case missing-tools
rm -rf -- "$case_root/tools/native-pcm-runner"
output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 ]] || { printf 'unexpected success: missing tool surface escaped\n' >&2; exit 97; }
[[ "$output" == *'tool surface is incomplete'* ]] || { printf 'missing tool wrong/setup outcome: %s\n' "$output" >&2; exit 96; }
copy_case missing-fixture
rm -f -- "$case_root/fixtures/native-pcm-runner/v1/MANIFEST.tsv"
output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 ]] || { printf 'unexpected success: missing fixture surface escaped\n' >&2; exit 97; }
[[ "$output" == *'fixture surface is incomplete'* ]] || { printf 'missing fixture wrong/setup outcome: %s\n' "$output" >&2; exit 96; }

install_rg_fault() {
    mkdir -p "$case_root/bin"
    cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
set +e
out=$(mktemp); err=$(mktemp)
"$REAL_RG" "$@" >"$out" 2>"$err"; status=$?
if [[ "$*" == *"$RG_FAIL_NEEDLE"* && "$*" == *"${RG_FAIL_CONTEXT:-}"* ]]; then
    [[ "$status" == "$RG_EXPECT_STATUS" ]] || { printf 'DELEGATE_SETUP status=%s expected=%s\n' "$status" "$RG_EXPECT_STATUS" >&2; exit 96; }
    if [[ "$RG_FAIL_MODE" == full ]]; then
        [[ -s "$out" ]] || { printf 'DELEGATE_SETUP unexpectedly empty payload\n' >&2; exit 96; }
        if [[ "$RG_FAIL_NEEDLE" == 'compile_session\(' ]]; then while IFS= read -r row; do [[ "$row" == *:*:*$'miso_engine_v1_compile_session('* ]] || { printf 'DELEGATE_SETUP invalid bypass row=%s\n' "$row" >&2; exit 96; }; done <"$out"; fi
        cat "$out"
    fi
    cat "$err" >&2
    printf 'RG_FAULT_SENTINEL delegate_status=%s payload_lines=%s mode=%s\n' "$status" "$(wc -l <"$out")" "$RG_FAIL_MODE" >&2
    exit 7
fi
cat "$out"; cat "$err" >&2; exit "$status"
EOF
    chmod +x "$case_root/bin/rg"
}
assert_rg_fault() {
    label=$1 needle=$2 diagnostic=$3 context=${4:-} mode=${5:-empty} expected=${6:-0}
    copy_case "$label"; install_rg_fault
    if output="$(REAL_RG="$(command -v rg)" RG_FAIL_NEEDLE="$needle" RG_FAIL_CONTEXT="$context" RG_FAIL_MODE="$mode" RG_EXPECT_STATUS="$expected" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)"; then
        printf 'unexpected success: %s producer failure was swallowed\n%s\n' "$label" "$output" >&2; return 97
    else status=$?; fi
    [[ "$status" != 96 && "$output" == *"$diagnostic"* && "$output" == *"RG_FAULT_SENTINEL delegate_status=$expected"* && "$output" == *"mode=$mode"* ]] || { printf '%s fault wrong/setup outcome (status %s):\n%s\n' "$label" "$status" "$output" >&2; return 96; }
}

# Every V1 search/filter class is reached selectively after the preceding valid fixture checks.
assert_rg_fault required-dependency '^sha2\.workspace = true$' 'missing exact direct dependency sha2 (rg status 7)' '' empty 0
assert_rg_fault forbidden-dependency '^graph-compiler' 'forbidden dependency scan failed (rg status 7)' '' empty 1
assert_rg_fault required-abi miso_engine_v1_engine_destroy 'missing frozen ABI operation miso_engine_v1_engine_destroy (rg status 7)' '' empty 0
assert_rg_fault bypass-source-empty 'compile_session\(' 'Rust product bypass source scan failed (rg status 7)' '' empty 0
assert_rg_fault bypass-source-full 'compile_session\(' 'Rust product bypass source scan failed (rg status 7)' '' full 0
assert_rg_fault bypass-exclusion miso_engine_v1_compile_session 'Rust product bypass exclusion failed (rg status 7)' bypass-source empty 1
assert_rg_fault ownership-filter '^tools/native-pcm-runner/' 'native runner ownership filter failed (rg status 7)' reachable empty 1
assert_rg_fault comment-filter '///?' 'native runner comment filter failed (rg status 7)' owned empty 1
assert_rg_fault reachability-source-empty native-pcm-runner 'native runner reachability scan failed (rg status 7)' crates empty 0

copy_case verifier-error
mkdir -p "$case_root/bin"
real_python="$(command -v python3)"
cat >"$case_root/bin/python3" <<'EOF'
#!/usr/bin/env bash
"$REAL_PYTHON" "$@" || exit $?
printf 'PYTHON_FIXTURE_SENTINEL\n' >&2
exit 6
EOF
chmod +x "$case_root/bin/python3"
if output="$(REAL_PYTHON="$real_python" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)"; then printf 'unexpected success: verifier execution error escaped\n' >&2; exit 97; else status=$?; fi
[[ "$status" != 96 && "$output" == *'python3 status 6'* && "$output" == *PYTHON_FIXTURE_SENTINEL* ]] || { printf 'verifier error wrong/setup outcome (status %s): %s\n' "$status" "$output" >&2; exit 96; }

copy_case verifier-read-failure
rm -f -- "$case_root/fixtures/native-pcm-runner/v1/riff-96000.json"
if output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)"; then
    printf 'unexpected success: verifier required-input read failure escaped\n' >&2; exit 97
else status=$?; fi
[[ "$output" == *'riff-96000.json'* && "$output" == *'No such file or directory'* && "$output" == *'python3 status 1'* ]] || { printf 'verifier read failure wrong/setup outcome (status %s): %s\n' "$status" "$output" >&2; exit 96; }

# Required discovery/count each reject both an empty error and complete real valid output plus error.
assert_find_fault() {
copy_case "find-$1"; mode=$1; mkdir -p "$case_root/bin"; real_find="$(command -v find)"
cat >"$case_root/bin/find" <<'EOF'
#!/usr/bin/env bash
out=$(mktemp); "$REAL_FIND" "$@" >"$out"; status=$?
[[ "$status" == 0 ]] || { printf 'FIND_SETUP status=%s\n' "$status" >&2; exit 96; }
rows=$(wc -l <"$out"); [[ "$rows" == 4 ]] || { printf 'FIND_SETUP rows=%s\n' "$rows" >&2; exit 96; }
while IFS= read -r path; do [[ -f "$path" && "$(basename "$path")" == riff-*.wav ]] || { printf 'FIND_SETUP bad_path=%s\n' "$path" >&2; exit 96; }; done <"$out"
names=$(while IFS= read -r path; do basename "$path"; done <"$out" | sort | tr '\n' ' ')
[[ "$names" == 'riff-44100.wav riff-48000.wav riff-88200.wav riff-96000.wav ' ]] || { printf 'FIND_SETUP names=%s\n' "$names" >&2; exit 96; }
[[ "$FAULT_MODE" == full ]] && cat "$out"
printf 'FIND_SENTINEL delegate_status=0 rows=4 mode=%s\n' "$FAULT_MODE" >&2
rm -f -- "$out"
exit 9
EOF
chmod +x "$case_root/bin/find"
if output="$(REAL_FIND="$real_find" FAULT_MODE="$mode" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)"; then printf 'unexpected success: find %s error swallowed\n' "$mode" >&2; return 97; else status=$?; fi
[[ "$status" != 96 && "$output" == *'RIFF corpus discovery failed (find status 9)'* && "$output" == *"FIND_SENTINEL delegate_status=0 rows=4 mode=$mode"* ]] || { printf 'find %s wrong/setup outcome (status %s): %s\n' "$mode" "$status" "$output" >&2; return 96; }
}
assert_find_fault empty; assert_find_fault full

assert_wc_fault() {
copy_case "wc-$1"; mode=$1; mkdir -p "$case_root/bin"; real_wc="$(command -v wc)"
cat >"$case_root/bin/wc" <<'EOF'
#!/usr/bin/env bash
input=$(mktemp); out=$(mktemp); tee "$input" | "$REAL_WC" "$@" >"$out"; status=${PIPESTATUS[1]}
[[ "$status" == 0 ]] || { printf 'WC_SETUP status=%s\n' "$status" >&2; exit 96; }
rows=$("$REAL_WC" -l <"$input"); value=$(<"$out"); [[ "$rows" == 4 && "$value" =~ ^[[:space:]]*4[[:space:]]*$ ]] || { printf 'WC_SETUP rows=%s value=%s\n' "$rows" "$value" >&2; exit 96; }
while IFS= read -r path; do [[ -f "$path" && "$(basename "$path")" == riff-*.wav ]] || { printf 'WC_SETUP bad_input=%s\n' "$path" >&2; exit 96; }; done <"$input"
[[ "$FAULT_MODE" == full ]] && cat "$out"
printf 'WC_SENTINEL delegate_status=0 input_rows=4 value=4 mode=%s\n' "$FAULT_MODE" >&2
rm -f -- "$input" "$out"
exit 8
EOF
chmod +x "$case_root/bin/wc"
if output="$(REAL_WC="$real_wc" FAULT_MODE="$mode" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)"; then printf 'unexpected success: wc %s error swallowed\n' "$mode" >&2; return 97; else status=$?; fi
[[ "$status" != 96 && "$output" == *'RIFF corpus count failed (wc status 8)'* && "$output" == *"WC_SENTINEL delegate_status=0 input_rows=4 value=4 mode=$mode"* ]] || { printf 'wc %s wrong/setup outcome (status %s): %s\n' "$mode" "$status" "$output" >&2; return 96; }
}
assert_wc_fault empty; assert_wc_fault full

# Reachability status loss is injected after all valid metadata and payload are available. The
# unchanged filters therefore still produce a clean result in the disposable mutant, and the
# same assertion must identify that unexpected success with status 97.
copy_case reachability-error
mkdir -p "$case_root/bin"
cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *'native-pcm-runner'* && "$*" == *'crates'* ]]; then
    out=$(mktemp); "$REAL_RG" "$@" >"$out"; status=$?
    [[ "$status" == 0 && -s "$out" ]] || { printf 'REACHABILITY_SETUP status=%s lines=%s\n' "$status" "$(wc -l <"$out")" >&2; exit 96; }
    while IFS= read -r row; do [[ "$row" == tools/native-pcm-runner/* || "$row" =~ :[0-9]+:[[:space:]]*///?[[:space:]] ]] || { printf 'REACHABILITY_SETUP prohibited_row=%s\n' "$row" >&2; exit 96; }; done <"$out"
    cat "$out"
    printf 'REACHABILITY_SENTINEL delegate_status=0 payload_lines=%s mode=full\n' "$(wc -l <"$out")" >&2
    rm -f -- "$out"
    exit 7
fi
exec "$REAL_RG" "$@"
EOF
chmod +x "$case_root/bin/rg"
real_rg="$(command -v rg)"
assert_reachability_status() {
    checker=$1
    if output="$(REAL_RG="$real_rg" PATH="$case_root/bin:$PATH" "$checker" "$case_root" v1 2>&1)"; then
        printf 'unexpected success: reachability source status was swallowed\n%s\n' "$output" >&2
        return 97
    else
        status=$?
    fi
    [[ "$status" != 96 && "$output" == *'reachability scan failed (rg status 7)'* && "$output" == *'REACHABILITY_SENTINEL delegate_status=0 payload_lines='* && "$output" == *'mode=full'* ]] || { printf 'reachability status wrong/setup diagnostic (status %s): %s\n' "$status" "$output" >&2; return 96; }
}
assert_reachability_status "$case_root/scripts/check-native-pcm-runner.sh"
mutant="$case_root/scripts/check-native-pcm-runner-mutant.sh"
cp "$case_root/scripts/check-native-pcm-runner.sh" "$mutant"
sed -i 's/\[\[ "$source_status" == 0 || "$source_status" == 1 \]\] || fail "native runner reachability scan failed (rg status $source_status): $(<"$scan_tmp\/reachable.err")"/true # MUTANT swallow reachability source status/' "$mutant"
diff -u "$case_root/scripts/check-native-pcm-runner.sh" "$mutant" >"/tmp/454-sol3-reachability-mutant.diff" || [[ $? == 1 ]]
if assert_reachability_status "$mutant"; then printf 'reachability counter-mutant did not reach unexpected-success assertion\n' >&2; exit 1; else status=$?; fi
[[ "$status" == 97 ]] || { printf 'reachability counter-mutant assertion status %s (expected 97)\n' "$status" >&2; exit 1; }
restored="$case_root/scripts/check-native-pcm-runner-restored.sh"; cp "$case_root/scripts/check-native-pcm-runner.sh" "$restored"; cmp -s "$case_root/scripts/check-native-pcm-runner.sh" "$restored"
assert_reachability_status "$restored"

printf 'native PCM runner V1 policy mutations: ok\n'
