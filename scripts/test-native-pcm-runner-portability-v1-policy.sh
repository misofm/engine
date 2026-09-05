#!/usr/bin/env bash
# Hermetic mutations for the Issue-116 platform publication boundary.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT

copy_case() {
    case_root="$temp/$1"
    mkdir -p "$case_root/tools/native-pcm-runner/src" "$case_root/scripts" \
        "$case_root/docs"
    cp "$root/tools/native-pcm-runner/src/lib.rs" \
        "$case_root/tools/native-pcm-runner/src/lib.rs"
    cp "$root/scripts/check-native-pcm-runner.sh" "$case_root/scripts/"
    cp "$root/docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md" "$case_root/docs/"
}

reject() {
    if "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" portability \
        >/dev/null 2>&1; then
        printf 'native PCM portability mutation escaped: %s\n' "$1" >&2
        exit 1
    fi
}

copy_case baseline
"$case_root/scripts/check-native-pcm-runner.sh" "$case_root" portability >/dev/null

copy_case missing-source; rm -f -- "$case_root/tools/native-pcm-runner/src/lib.rs"
if output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" portability 2>&1)"; then printf 'unexpected success: missing portability source escaped\n' >&2; exit 97; else status=$?; fi
[[ "$output" == *'runner library or contract is missing'* ]] || { printf 'missing source wrong/setup outcome (status %s): %s\n' "$status" "$output" >&2; exit 96; }
copy_case missing-contract; rm -f -- "$case_root/docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md"
if output="$($case_root/scripts/check-native-pcm-runner.sh "$case_root" portability 2>&1)"; then printf 'unexpected success: missing portability contract escaped\n' >&2; exit 97; else status=$?; fi
[[ "$output" == *'runner library or contract is missing'* ]] || { printf 'missing contract wrong/setup outcome (status %s): %s\n' "$status" "$output" >&2; exit 96; }

copy_case pathname-fallback
printf '\n// mutation\nfn escaped_fallback(&self) { fs::hard_link(&self.partial_path, &self.final_path); }\n' \
    >>"$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'generic pathname fallback'

copy_case unix-import
sed -i '/#\[cfg(unix)\]/{N;s/#\[cfg(unix)\]\n//;}' \
    "$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'unguarded Unix import'

copy_case replace-enabled
sed -i '0,/replace_if_exists = 0/s//replace_if_exists = 1/' \
    "$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'replace-enabled Windows publication'

copy_case missing-post-identity
sed -i '0,/if !adapter.partial_is_absent() || !adapter.final_is_owned()/s//if false/' \
    "$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'unchecked published identity'

copy_case followed-path
sed -i '0,/O_NOFOLLOW/s//O_FOLLOW/' \
    "$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'followed Unix pathname identity'

copy_case replace-enabled-linux
sed -i '0,/RENAME_NOREPLACE/s//RENAME_REPLACE/' \
    "$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'replace-enabled Linux publication'

copy_case missing-exclusive-contract
sed -i 's/exclusively owned for the complete runner/shared for the runner/' \
    "$case_root/docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md"
reject 'missing exclusive-directory contract'

copy_case concurrency-overclaim
printf '\nThe runner is safe against concurrent same-privilege mutation.\n' \
    >>"$case_root/docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md"
reject 'concurrent mutation overclaim'

copy_case unowned-cleanup
sed -i '0,/if self.path_is_owned(&self.final_path)/s//if true/' \
    "$case_root/tools/native-pcm-runner/src/lib.rs"
reject 'unowned final cleanup'

install_rg_fault() {
    mkdir -p "$case_root/bin"
    cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
set +e
out=$(mktemp); err=$(mktemp); "$REAL_RG" "$@" >"$out" 2>"$err"; status=$?
if [[ "$*" == *"$RG_FAIL_NEEDLE"* && "$*" == *"${RG_FAIL_CONTEXT:-}"* ]]; then
    [[ "$status" == "$RG_EXPECT_STATUS" ]] || { printf 'DELEGATE_SETUP status=%s expected=%s\n' "$status" "$RG_EXPECT_STATUS" >&2; exit 96; }
    if [[ "$RG_FAIL_MODE" == full ]]; then
        [[ -s "$out" ]] || { printf 'DELEGATE_SETUP empty payload\n' >&2; exit 96; }
        value=$(tr -d '[:space:]' <"$out")
        if [[ "$RG_FAIL_NEEDLE" == FileIdentity::from_file ]]; then [[ "$value" =~ ^[0-9]+$ && "$value" -ge 2 ]] || { printf 'DELEGATE_SETUP identity_count=%s\n' "$value" >&2; exit 96; }; fi
        if [[ "$RG_FAIL_NEEDLE" == O_NOFOLLOW ]]; then [[ "$value" == 4 ]] || { printf 'DELEGATE_SETUP nofollow_count=%s\n' "$value" >&2; exit 96; }; fi
        cat "$out"
    fi
    cat "$err" >&2; printf 'PORTABILITY_RG_SENTINEL delegate_status=%s payload=%s mode=%s injected=%s\n' "$status" "$(tr -d '[:space:]' <"$out")" "$RG_FAIL_MODE" "$RG_INJECT_STATUS" >&2; exit "$RG_INJECT_STATUS"
fi
cat "$out"; cat "$err" >&2; exit "$status"
EOF
    chmod +x "$case_root/bin/rg"
}
assert_rg_fault() {
    label=$1 needle=$2 diagnostic=$3 context=${4:-} mode=${5:-empty} expected=${6:-0} injected=${7:-7}
    copy_case "$label"; install_rg_fault
    if output="$(REAL_RG="$(command -v rg)" RG_FAIL_NEEDLE="$needle" RG_FAIL_CONTEXT="$context" RG_FAIL_MODE="$mode" RG_EXPECT_STATUS="$expected" RG_INJECT_STATUS="$injected" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" portability 2>&1)"; then printf 'unexpected success: %s failure swallowed\n' "$label" >&2; return 97; else status=$?; fi
    [[ "$status" != 96 && "$output" == *"$diagnostic"* && "$output" == *"PORTABILITY_RG_SENTINEL delegate_status=$expected"* && "$output" == *"mode=$mode injected=$injected"* ]] || { printf '%s fault wrong/setup outcome (status %s):\n%s\n' "$label" "$status" "$output" >&2; return 96; }
}

# Required source/contract, forbidden, count, and late ownership producers retain operation status.
assert_rg_fault required-source-boundary complete_publication 'missing required boundary: complete_publication (rg status 7)' '' empty 0
assert_rg_fault required-contract-exclusive 'exclusively owned for the complete runner' 'exclusive output-directory precondition is missing (rg status 7)' '' empty 0
assert_rg_fault required-contract-limit 'does not claim safety against a' 'concurrent same-privilege mutation limitation is missing (rg status 7)' '' empty 0
assert_rg_fault contract-prohibition 'identity-conditionally unlink' 'contract prohibition scan failed (rg status 7)' '' empty 1
assert_rg_fault late-forbidden 'FakeEntry::Owned' 'portability forbidden scan failed (rg status 7)' '' empty 1
assert_rg_fault identity-count-empty 'FileIdentity::from_file' 'FileIdentity count scan failed (rg status 7)' '' empty 0
assert_rg_fault identity-count-full 'FileIdentity::from_file' 'FileIdentity count scan failed (rg status 7)' '' full 0
assert_rg_fault nofollow-count-empty O_NOFOLLOW 'O_NOFOLLOW count scan failed (rg status 7)' '-c' empty 0
assert_rg_fault nofollow-count-full O_NOFOLLOW 'O_NOFOLLOW count scan failed (rg status 7)' '-c' full 0
assert_rg_fault identity-count-absence 'FileIdentity::from_file' 'held and post-publication handle identities are not both checked (rg status 1: no matches)' '' empty 0 1
assert_rg_fault nofollow-count-absence O_NOFOLLOW 'both Linux/Android and Apple path identity checks must be no-follow (rg status 1: no matches)' '-c' empty 0 1
assert_rg_fault post-publication 'if !adapter.partial_is_absent()' 'missing required ownership boundary: if !adapter.partial_is_absent() || !adapter.final_is_owned() (rg status 7)'
assert_rg_fault partial-cleanup 'if self.path_is_owned(&self.partial_path)' 'missing required ownership boundary: if self.path_is_owned(&self.partial_path) (rg status 7)'
assert_rg_fault final-cleanup 'if self.path_is_owned(&self.final_path)' 'missing required ownership boundary: if self.path_is_owned(&self.final_path) (rg status 7)'

copy_case python-error; mkdir -p "$case_root/bin"; real_python="$(command -v python3)"
cat >"$case_root/bin/python3" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *' - '* || "${1:-}" == '-' ]]; then printf 'PYTHON_IMPORT_SENTINEL\n' >&2; exit 8; fi
exec "$REAL_PYTHON" "$@"
EOF
chmod +x "$case_root/bin/python3"
if output="$(REAL_PYTHON="$real_python" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" portability 2>&1)"; then printf 'unexpected success: Python controlled exit escaped\n' >&2; exit 97; else status=$?; fi
[[ "$status" != 96 && "$output" == *'python3 status 8'* && "$output" == *PYTHON_IMPORT_SENTINEL* ]] || { printf 'Python import fault wrong/setup outcome (status %s): %s\n' "$status" "$output" >&2; exit 96; }

copy_case python-read-failure; mkdir -p "$case_root/bin"; real_python="$(command -v python3)"
cat >"$case_root/bin/python3" <<'EOF'
#!/usr/bin/env bash
if [[ "${4:-}" == tools/native-pcm-runner/src/lib.rs ]]; then
    "$REAL_PYTHON" "${@:1:3}" tools/native-pcm-runner/src/lib.rs.missing
    exit $?
fi
exec "$REAL_PYTHON" "$@"
EOF
chmod +x "$case_root/bin/python3"
if output="$(REAL_PYTHON="$real_python" PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" portability 2>&1)"; then printf 'unexpected success: Python late read failure escaped\n' >&2; exit 97; else status=$?; fi
[[ "$output" == *'lib.rs.missing'* && "$output" == *'No such file or directory'* && "$output" == *'python3 status 1'* ]] || { printf 'Python late read wrong/setup outcome (status %s): %s\n' "$status" "$output" >&2; exit 96; }

# Inject a late forbidden-scan error with clean output. The real checker must expose both status
# and sentinel; swallowing that status in the production scan is caught as unexpected success.
copy_case late-scan-error
mkdir -p "$case_root/bin"
cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *'FakeEntry::Owned'* ]]; then
    out=$(mktemp); "$REAL_RG" "$@" >"$out"; status=$?
    [[ "$status" == 1 && ! -s "$out" ]] || { printf 'LATE_SETUP status=%s bytes=%s\n' "$status" "$(wc -c <"$out")" >&2; exit 96; }
    printf 'LATE_PORTABILITY_SENTINEL delegate_status=1 payload=empty\n' >&2
    exit 9
fi
exec "$REAL_RG" "$@"
EOF
chmod +x "$case_root/bin/rg"
real_rg="$(command -v rg)"
assert_late_status() {
    checker=$1
    if output="$(REAL_RG="$real_rg" PATH="$case_root/bin:$PATH" "$checker" "$case_root" portability 2>&1)"; then
        printf 'unexpected success: late portability status was swallowed\n%s\n' "$output" >&2
        return 97
    else
        status=$?
    fi
    [[ "$status" != 96 && "$output" == *'portability forbidden scan failed (rg status 9)'* && "$output" == *'LATE_PORTABILITY_SENTINEL delegate_status=1 payload=empty'* ]] || { printf 'late scan wrong diagnostic (status %s): %s\n' "$status" "$output" >&2; return 96; }
}
assert_late_status "$case_root/scripts/check-native-pcm-runner.sh"
mutant="$case_root/scripts/check-native-pcm-runner-mutant.sh"
cp "$case_root/scripts/check-native-pcm-runner.sh" "$mutant"
sed -i 's#fail "portability forbidden scan failed (rg status \$status)"#:#' "$mutant"
diff -u "$case_root/scripts/check-native-pcm-runner.sh" "$mutant" >"/tmp/454-sol3-late-portability-mutant.diff" || [[ $? == 1 ]]
if assert_late_status "$mutant"; then
    printf 'late scan counter-mutant did not reach unexpected-success assertion\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 97 ]] || { printf 'late scan counter-mutant assertion status %s (expected 97)\n' "$status" >&2; exit 1; }
restored="$case_root/scripts/check-native-pcm-runner-restored.sh"; cp "$case_root/scripts/check-native-pcm-runner.sh" "$restored"; cmp -s "$case_root/scripts/check-native-pcm-runner.sh" "$restored"
assert_late_status "$restored"

printf 'native PCM runner portability policy mutations: ok\n'
