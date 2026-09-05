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

# Inject a late forbidden-scan error with clean output. The real checker must expose both status
# and sentinel; swallowing that status in the production scan is caught as unexpected success.
copy_case late-scan-error
mkdir -p "$case_root/bin"
cat >"$case_root/bin/rg" <<'EOF'
#!/usr/bin/env bash
if [[ "$*" == *'FakeEntry::Owned'* ]]; then
    printf 'LATE_PORTABILITY_SENTINEL\n' >&2
    exit 9
fi
exec /usr/bin/rg "$@"
EOF
chmod +x "$case_root/bin/rg"
output="$(PATH="$case_root/bin:$PATH" "$case_root/scripts/check-native-pcm-runner.sh" "$case_root" portability 2>&1)" && status=0 || status=$?
[[ "$status" != 0 && "$output" == *'portability forbidden scan failed (rg status 9)'* && "$output" == *'LATE_PORTABILITY_SENTINEL'* ]] || { printf 'late scan error was misclassified: %s\n' "$output" >&2; exit 1; }
mutant="$case_root/scripts/check-native-pcm-runner-mutant.sh"
cp "$case_root/scripts/check-native-pcm-runner.sh" "$mutant"
sed -i 's#fail "portability forbidden scan failed (rg status \$status)"#:#' "$mutant"
assert_counter_mutant() {
    if output="$(PATH="$case_root/bin:$PATH" "$mutant" "$case_root" portability 2>&1)"; then
        printf 'counter-mutant unexpectedly passed: operation status was swallowed\n%s\n' "$output" >&2
        return 97
    fi
    printf 'counter-mutant rejected before the targeted unexpected-success assertion: %s\n' "$output" >&2
    return 0
}
if assert_counter_mutant; then
    printf 'late scan counter-mutant did not reach unexpected-success assertion\n' >&2
    exit 1
else
    status=$?
fi
[[ "$status" == 97 ]] || { printf 'late scan counter-mutant assertion status %s (expected 97)\n' "$status" >&2; exit 1; }

printf 'native PCM runner portability policy mutations: ok\n'
