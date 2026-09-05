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

mkdir -p "$scratch/rg-partial"
cat >"$scratch/rg-partial/rg" <<'EOF'
#!/usr/bin/env bash
printf 'valid partial output\n'
exit 9
EOF
chmod +x "$scratch/rg-partial/rg"
collect_output=$(expect_failure collect-partial 'valid partial output' env PATH="$scratch/rg-partial:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_collect collect-partial anything "" "$2"' _ "$root/scripts/lib/gate.sh" "$scratch/src")
[[ "$collect_output" == *'collect-partial scan errored (rg exit 9)'* ]] || exit 1
filter_output=$(expect_failure filter-partial 'valid partial output' env PATH="$scratch/rg-partial:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_filter_exclude filter-partial allow "kept"' _ "$root/scripts/lib/gate.sh")
[[ "$filter_output" == *'filter-partial filter errored (rg exit 9)'* ]] || exit 1
[[ -z "$(gate_filter_exclude empty-filter forbidden '')" ]] || { echo 'empty filter input was not empty' >&2; exit 1; }
[[ -z "$(gate_filter_exclude allowed-empty clean 'clean')" ]] || { echo 'allowed-empty filter retained a row' >&2; exit 1; }

required_output=$(expect_failure required-absent 'required-absent search failed (rg exit 1)' env GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_required required-absent missing "" "$2"' _ "$root/scripts/lib/gate.sh" "$scratch/src")
required_match="$(GATE_FAILURE_PREFIX='gate test' gate_scan_required required-match forbidden '' "$scratch/src")"
[[ "$required_match" == *'match.txt:1:forbidden'* ]] || { echo 'required match evidence missing' >&2; exit 1; }
required_partial=$(expect_failure required-partial 'required-partial search failed (rg exit 9)' env PATH="$scratch/rg-partial:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_scan_required required-partial anything "" "$2"' _ "$root/scripts/lib/gate.sh" "$scratch/src")
[[ "$required_partial" == *'valid partial output'* ]] || { echo 'required partial evidence missing' >&2; exit 1; }

mkdir -p "$scratch/wc-fail"
cat >"$scratch/wc-fail/wc" <<'EOF'
#!/usr/bin/env bash
cat >/dev/null
printf '2\n'
exit 6
EOF
chmod +x "$scratch/wc-fail/wc"
count_output=$(expect_failure count-partial 'count-partial count errored (wc exit 6)' env PATH="$scratch/wc-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c \
    'source "$1"; gate_count_lines count-partial "$2"' _ "$root/scripts/lib/gate.sh" $'one\ntwo')
[[ "$count_output" == *$'2\n'* ]] || { echo 'count partial evidence missing' >&2; exit 1; }

manifest="$scratch/dependencies.toml"
cat >"$manifest" <<'EOF'
[dependencies]
zeta.workspace = true
alpha = "1"
[dev-dependencies]
ignored.workspace = true
EOF
[[ "$(gate_toml_dependencies "$manifest")" == $'alpha\nzeta' ]] || { echo 'dependency output is not sorted/scoped' >&2; exit 1; }
plain_manifest="$scratch/plain.toml"
cat >"$plain_manifest" <<'EOF'
[dependencies]
zeta.workspace=true
alpha = "1"
lane="1"
[dev-dependencies]
ignored.workspace=true
[target.'cfg(unix)'.dependencies]
target_only.workspace=true
EOF
[[ "$(gate_toml_dependencies "$plain_manifest" plain)" == $'alpha\nlane\nzeta' ]] || { echo 'plain dependency mode changed compact/spaced declaration semantics' >&2; exit 1; }
# The default mode is intentionally frozen for rack callers: compact declarations remain the
# complete first field rather than being reinterpreted by the narrow plain-section mode.
[[ "$(gate_toml_dependencies "$plain_manifest")" == $'alpha\nlane="1"\nzeta.workspace=true' ]] || { echo 'default dependency parser changed' >&2; exit 1; }

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

for pipefail_setting in +o -o; do
    for invocation in direct conditional; do
        command_text='set '"$pipefail_setting"' pipefail; source "$1"; GATE_FAILURE_PREFIX="gate test"; gate_find_collect find-check "$2"'
        [[ $invocation == direct ]] || command_text='set '"$pipefail_setting"' pipefail; source "$1"; GATE_FAILURE_PREFIX="gate test"; if gate_find_collect find-check "$2"; then exit 0; else exit $?; fi'
        find_output=$(PATH="$scratch/rg-partial:$PATH" bash -c "$command_text" _ "$root/scripts/lib/gate.sh" "$scratch/missing" 2>&1) && find_rc=0 || find_rc=$?
        [[ $find_rc -ne 0 && $find_output == *'find-check traversal errored'* ]] || { echo "find status lost ($pipefail_setting/$invocation): $find_output" >&2; exit 1; }
    done
done

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
printf 'alpha\nzeta\n'
exit 8
EOF
chmod +x "$scratch/awk-fail/awk" "$scratch/awk-fail/sort" "$scratch/sort-fail/awk" "$scratch/sort-fail/sort"
for pipefail_setting in +o -o; do
    awk_output="$(PATH="$scratch/awk-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c "set $pipefail_setting pipefail; source \"\$1\"; if gate_toml_dependencies \"\$2\"; then exit 0; else exit \$?; fi" _ "$root/scripts/lib/gate.sh" "$manifest" 2>&1)" && awk_rc=0 || awk_rc=$?
    [[ "$awk_rc" -eq 7 && "$awk_output" == *'awk status 7'* && "$awk_output" != *'partial'* ]] || { echo "awk failure leaked partial output or was not explicit: $awk_output" >&2; exit 1; }
    sort_output="$(PATH="$scratch/sort-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c "set $pipefail_setting pipefail; source \"\$1\"; if gate_toml_dependencies \"\$2\"; then exit 0; else exit \$?; fi" _ "$root/scripts/lib/gate.sh" "$manifest" 2>&1)" && sort_rc=0 || sort_rc=$?
    [[ "$sort_rc" -eq 8 && "$sort_output" == *'sort status 8'* && "$sort_output" != *$'alpha\nzeta'* ]] || { echo "sort failure not explicit or leaked partial output: $sort_output" >&2; exit 1; }
done

unique_output=$(PATH="$scratch/awk-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c 'source "$1"; gate_unique_nonempty_lines unique "$2"' _ "$root/scripts/lib/gate.sh" $'engine\nlane' 2>&1) && unique_rc=0 || unique_rc=$?
[[ $unique_rc -eq 7 && $unique_output == *'uniqueness filter errored (awk status 7)' ]] || { echo "unique failure not explicit: $unique_output" >&2; exit 1; }
mkdir -p "$scratch/paste-fail"
cat >"$scratch/paste-fail/paste" <<'EOF'
#!/usr/bin/env bash
printf 'engine|lane\n'
exit 9
EOF
chmod +x "$scratch/paste-fail/paste"
join_output=$(PATH="$scratch/paste-fail:$PATH" GATE_FAILURE_PREFIX='gate test' bash -c 'source "$1"; gate_join_lines join "|" "$2"' _ "$root/scripts/lib/gate.sh" $'engine\nlane' 2>&1) && join_rc=0 || join_rc=$?
[[ $join_rc -eq 9 && $join_output == *'join errored (paste status 9)' ]] || { echo "join failure not explicit: $join_output" >&2; exit 1; }

# Acceptance counter-mutants: each simulates the historic fail-open result. The same assertions
# above must classify the forged success/output as rejection evidence rather than silently accept it.
counter_dir="$scratch/counter-mutants"
mkdir -p "$counter_dir"
for mechanism in collect required filter count plain; do
    cp "$root/scripts/lib/gate.sh" "$counter_dir/$mechanism.sh"
done
sed -i '/gate_scan_collect()/,/gate_scan_required()/ s/0|1) printf.*return 0/0|1|*) printf '\''%s'\'' "$output"; return 0/' "$counter_dir/collect.sh"
sed -i '/gate_scan_required()/,/gate_filter_exclude()/ s/return "$rc"/return 0/' "$counter_dir/required.sh"
sed -i '/gate_filter_exclude()/,/gate_count_lines()/ s/return "$rc"/return 0/' "$counter_dir/filter.sh"
sed -i '/gate_count_lines()/,/gate_toml_dependencies()/ s/return "$rc"/printf '\''2'\''; return 0/' "$counter_dir/count.sh"
sed -i 's/if \[\[ "$mode" == plain || "$mode" == plain-target \]\]/if false/' "$counter_dir/plain.sh"
if PATH="$scratch/rg-partial:$PATH" bash -c 'source "$1"; gate_scan_collect mutant x "" "$2" >/dev/null' _ "$counter_dir/collect.sh" "$scratch/src"; then :; else echo 'collect counter-mutant did not forge success' >&2; exit 1; fi
if PATH="$scratch/rg-partial:$PATH" bash -c 'source "$1"; gate_scan_required mutant x "" "$2" >/dev/null' _ "$counter_dir/required.sh" "$scratch/src"; then :; else echo 'required counter-mutant did not forge success' >&2; exit 1; fi
if PATH="$scratch/rg-partial:$PATH" bash -c 'source "$1"; gate_filter_exclude mutant x y >/dev/null' _ "$counter_dir/filter.sh"; then :; else echo 'filter counter-mutant did not forge success' >&2; exit 1; fi
[[ "$(PATH="$scratch/wc-fail:$PATH" bash -c 'source "$1"; gate_count_lines mutant "$2"' _ "$counter_dir/count.sh" $'one\ntwo')" == 2 ]] || { echo 'count counter-mutant did not forge expected count' >&2; exit 1; }
[[ "$(bash -c 'source "$1"; gate_toml_dependencies "$2" plain' _ "$counter_dir/plain.sh" "$plain_manifest")" != $'alpha\nlane\nzeta' ]] || { echo 'plain parser counter-mutant escaped acceptance' >&2; exit 1; }
printf 'counter-mutant controls exercised: collect required filter count plain\n'

[[ "$PWD" == "$before_pwd" ]] || { echo 'gate changed caller cwd' >&2; exit 1; }
[[ "$(set -o)" == "$before_opts" ]] || { echo 'gate changed caller shell options' >&2; exit 1; }
(cd "$scratch" && bash "$root/scripts/check-rack-policy.sh" "$root" >/dev/null)
printf 'gate library tests: ok\n'
