#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
source "$root/scripts/lib/gate.sh"
scratch=$(mktemp -d); trap 'rm -rf -- "$scratch"' EXIT
before_pwd=$PWD
before_opts=$(set -o)

expect_failure() {
    local name=$1 expected=$2
    shift 2
    local output rc
    if output="$("$@" 2>&1)"; then
        printf 'gate helper unexpectedly passed: %s\n' "$name" >&2
        exit 1
    else
        rc=$?
    fi
    [[ "$output" == *"$expected"* ]] || {
        printf 'gate helper %s failed with wrong diagnostic (rc=%s): %s\n' "$name" "$rc" "$output" >&2
        exit 1
    }
    printf '%s' "$output"
}

mkdir -p "$scratch/src"
printf 'clean\n' >"$scratch/src/clean.txt"
GATE_FAILURE_PREFIX='gate test' gate_scan_forbidden clean 'forbidden' '' "$scratch/src"
printf 'forbidden\n' >"$scratch/src/match.txt"
match_output=$(expect_failure match 'gate test: match' env GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_forbidden match forbidden "" "$2"' _ "$root/scripts/lib/gate.sh" "$scratch/src")
[[ "$match_output" == *'match.txt:1:forbidden'* ]] || { echo 'match evidence missing' >&2; exit 1; }
missing_output=$(expect_failure missing 'missing search path(s)' env GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_forbidden missing forbidden "" "$2"' _ "$root/scripts/lib/gate.sh" "$scratch/missing")
[[ "$missing_output" == *'gate test: missing scan could not run (rg exit 2)'* ]] || exit 1
combined_output=$(expect_failure combined 'missing search path(s)' env GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_forbidden combined forbidden "" "$2" "$3"' _ "$root/scripts/lib/gate.sh" "$scratch/src" "$scratch/missing")
[[ "$combined_output" == *'match.txt:1:forbidden'* && "$combined_output" == *'gate test: combined scan could not run (rg exit 2)'* ]] || {
    echo 'combined scan did not preserve match and missing-root evidence' >&2; exit 1;
}
mkdir -p "$scratch/bin"
printf '#!/usr/bin/env bash\nexit 2\n' >"$scratch/bin/rg"
chmod +x "$scratch/bin/rg"
execution_output=$(expect_failure execution-error 'gate test: execution-error scan errored (rg exit 2)' \
    env PATH="$scratch/bin:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_forbidden execution-error forbidden "" "$2"' _ "$root/scripts/lib/gate.sh" "$scratch/src")

manifest="$scratch/dependencies.toml"
cat >"$manifest" <<'EOF'
[dependencies]
zeta.workspace = true
alpha = "1"
[dev-dependencies]
ignored.workspace = true
EOF
[[ "$(gate_toml_dependencies "$manifest")" == $'alpha\nzeta' ]] || { echo 'dependency output is not sorted/scoped' >&2; exit 1; }

check_extractor_failures() {
    local pipefail_setting=$1
    for invocation in direct conditional; do
        local output rc
        if [[ "$invocation" == direct ]]; then
            output="$(bash -c "set $pipefail_setting pipefail; source \"\$1\"; GATE_FAILURE_PREFIX='gate test'; gate_toml_dependencies \"\$2\"" _ "$root/scripts/lib/gate.sh" "$scratch/missing.toml" 2>&1)" && rc=0 || rc=$?
        else
            output="$(bash -c "set $pipefail_setting pipefail; source \"\$1\"; GATE_FAILURE_PREFIX='gate test'; if gate_toml_dependencies \"\$2\"; then exit 0; else exit \$?; fi" _ "$root/scripts/lib/gate.sh" "$scratch/missing.toml" 2>&1)" && rc=0 || rc=$?
        fi
        [[ "$rc" -ne 0 && "$output" == *'gate test: dependency extraction failed'*'awk status'* ]] || {
            printf 'missing-manifest extraction was misclassified (%s/%s): %s\n' "$pipefail_setting" "$invocation" "$output" >&2; exit 1;
        }
    done
}
check_extractor_failures +o
check_extractor_failures -o

mkdir -p "$scratch/awk-fail" "$scratch/sort-fail"
cat >"$scratch/awk-fail/awk" <<'EOF'
#!/usr/bin/env bash
printf 'partial\n'
exit 7
EOF
cat >"$scratch/awk-fail/sort" <<'EOF'
#!/usr/bin/env bash
exec /usr/bin/sort "$@"
EOF
cat >"$scratch/sort-fail/awk" <<'EOF'
#!/usr/bin/env bash
exec /usr/bin/awk "$@"
EOF
cat >"$scratch/sort-fail/sort" <<'EOF'
#!/usr/bin/env bash
exit 8
EOF
chmod +x "$scratch/awk-fail/awk" "$scratch/awk-fail/sort" "$scratch/sort-fail/awk" "$scratch/sort-fail/sort"
for pipefail_setting in +o -o; do
    awk_output="$(PATH="$scratch/awk-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c "set $pipefail_setting pipefail; source \"\$1\"; if gate_toml_dependencies \"\$2\"; then exit 0; else exit \$?; fi" _ "$root/scripts/lib/gate.sh" "$manifest" 2>&1)" && awk_rc=0 || awk_rc=$?
    [[ "$awk_rc" -eq 7 && "$awk_output" == *'awk status 7'* && "$awk_output" != *'partial'* ]] || { echo "awk failure leaked partial output or was not explicit: $awk_output" >&2; exit 1; }
    sort_output="$(PATH="$scratch/sort-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c "set $pipefail_setting pipefail; source \"\$1\"; if gate_toml_dependencies \"\$2\"; then exit 0; else exit \$?; fi" _ "$root/scripts/lib/gate.sh" "$manifest" 2>&1)" && sort_rc=0 || sort_rc=$?
    [[ "$sort_rc" -eq 8 && "$sort_output" == *'sort status 8'* ]] || { echo "sort failure not explicit: $sort_output" >&2; exit 1; }
done

[[ "$PWD" == "$before_pwd" ]] || { echo 'gate changed caller cwd' >&2; exit 1; }
[[ "$(set -o)" == "$before_opts" ]] || { echo 'gate changed caller shell options' >&2; exit 1; }
(cd "$scratch" && bash "$root/scripts/check-rack-policy.sh" "$root" >/dev/null)
printf 'gate library tests: ok\n'
