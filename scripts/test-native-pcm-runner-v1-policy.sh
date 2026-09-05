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
    [[ "$status" != 0 && "$output" == *"reachability root is missing: $required_root"* ]] || {
        printf 'missing required root was not rejected: %s\n%s\n' "$required_root" "$output" >&2; exit 1;
    }
done
copy_case missing-tools
rm -rf -- "$case_root/tools/native-pcm-runner"
output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'tool surface is incomplete'* ]] || { printf 'missing tool surface was not rejected: %s\n' "$output" >&2; exit 1; }
copy_case missing-fixture
rm -f -- "$case_root/fixtures/native-pcm-runner/v1/MANIFEST.tsv"
output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'fixture surface is incomplete'* ]] || { printf 'missing fixture surface was not rejected: %s\n' "$output" >&2; exit 1; }

install_rg_fault() {
    mkdir -p "$case_root/bin"
    cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
set +e
"$REAL_RG" "$@"
status=$?
if [[ "$*" == *"$RG_FAIL_NEEDLE"* && "$*" == *"${RG_FAIL_CONTEXT:-}"* ]]; then printf 'RG_FAULT_SENTINEL\n' >&2; exit 7; fi
exit "$status"
EOF
    chmod +x "$case_root/bin/rg"
}
assert_rg_fault() {
    label=$1 needle=$2 diagnostic=$3 context=${4:-}
    copy_case "$label"; install_rg_fault
    output="$(REAL_RG="$(command -v rg)" RG_FAIL_NEEDLE="$needle" RG_FAIL_CONTEXT="$context" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)" && status=0 || status=$?
    [[ "$status" != 0 && "$output" == *"$diagnostic"* && "$output" == *RG_FAULT_SENTINEL* ]] || { printf '%s fault was misclassified (status %s):\n%s\n' "$label" "$status" "$output" >&2; exit 96; }
}

# Every V1 search/filter class is reached selectively after the preceding valid fixture checks.
assert_rg_fault required-dependency '^sha2\.workspace = true$' 'missing exact direct dependency sha2 (rg status 7)'
assert_rg_fault forbidden-dependency '^graph-compiler' 'forbidden dependency scan failed (rg status 7)'
assert_rg_fault required-abi miso_engine_v1_engine_destroy 'missing frozen ABI operation miso_engine_v1_engine_destroy (rg status 7)'
assert_rg_fault bypass-source 'compile_session\(' 'Rust product bypass source scan failed (rg status 7)'
assert_rg_fault bypass-exclusion miso_engine_v1_compile_session 'Rust product bypass exclusion failed (rg status 7)' bypass-source
assert_rg_fault ownership-filter '^tools/native-pcm-runner/' 'native runner ownership filter failed (rg status 7)' reachable
assert_rg_fault comment-filter '///?' 'native runner comment filter failed (rg status 7)' owned

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
output="$(REAL_PYTHON="$real_python" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'python3 status 6'* && "$output" == *PYTHON_FIXTURE_SENTINEL* ]] || { printf 'verifier error was misclassified: %s\n' "$output" >&2; exit 1; }

# Required discovery must report execution errors even when a shim emits all four valid paths.
copy_case find-error
mkdir -p "$case_root/bin"
real_find="$(command -v find)"
cat >"$case_root/bin/find" <<'EOF'
#!/usr/bin/env bash
"$REAL_FIND" "$@"
printf 'FIND_SENTINEL\n' >&2
exit 9
EOF
chmod +x "$case_root/bin/find"
output="$(REAL_FIND="$real_find" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'RIFF corpus discovery failed (find status 9)'* && "$output" == *FIND_SENTINEL* ]] || { printf 'find error escaped: %s\n' "$output" >&2; exit 1; }

copy_case wc-error; mkdir -p "$case_root/bin"; real_wc="$(command -v wc)"
cat >"$case_root/bin/wc" <<'EOF'
#!/usr/bin/env bash
"$REAL_WC" "$@"
printf 'WC_SENTINEL\n' >&2
exit 8
EOF
chmod +x "$case_root/bin/wc"
output="$(REAL_WC="$real_wc" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" v1 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'RIFF corpus count failed (wc status 8)'* && "$output" == *WC_SENTINEL* ]] || { printf 'wc error escaped: %s\n' "$output" >&2; exit 1; }

# Reachability status loss is injected after all valid metadata and payload are available. The
# unchanged filters therefore still produce a clean result in the disposable mutant, and the
# same assertion must identify that unexpected success with status 97.
copy_case reachability-error
mkdir -p "$case_root/bin"
cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *'native-pcm-runner'* && "$*" == *'crates'* ]]; then
    "$REAL_RG" "$@"
    printf 'REACHABILITY_SENTINEL\n' >&2
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
    [[ "$output" == *'reachability scan failed (rg status 7)'* && "$output" == *REACHABILITY_SENTINEL* ]] || { printf 'reachability status wrong diagnostic (status %s): %s\n' "$status" "$output" >&2; return 96; }
}
assert_reachability_status "$case_root/scripts/check-native-pcm-runner.sh"
mutant="$case_root/scripts/check-native-pcm-runner-mutant.sh"
cp "$case_root/scripts/check-native-pcm-runner.sh" "$mutant"
sed -i 's/\[\[ "$source_status" == 0 || "$source_status" == 1 \]\] || fail "native runner reachability scan failed (rg status $source_status): $(<"$scan_tmp\/reachable.err")"/true # MUTANT swallow reachability source status/' "$mutant"
diff -u "$case_root/scripts/check-native-pcm-runner.sh" "$mutant" >"/tmp/454-sol2-reachability-mutant.diff" || [[ $? == 1 ]]
if assert_reachability_status "$mutant"; then printf 'reachability counter-mutant did not reach unexpected-success assertion\n' >&2; exit 1; else status=$?; fi
[[ "$status" == 97 ]] || { printf 'reachability counter-mutant assertion status %s (expected 97)\n' "$status" >&2; exit 1; }
restored="$case_root/scripts/check-native-pcm-runner-restored.sh"; cp "$case_root/scripts/check-native-pcm-runner.sh" "$restored"; cmp -s "$case_root/scripts/check-native-pcm-runner.sh" "$restored"
assert_reachability_status "$restored"

printf 'native PCM runner V1 policy mutations: ok\n'
