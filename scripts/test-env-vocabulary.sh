#!/usr/bin/env bash
# Mutations for the environment/marker vocabulary gate (#104 phase C).
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

check() { bash "$1/scripts/check-env-vocabulary.sh" "$1"; }

new_case() {
    case_root="$scratch/$1"
    mkdir -p "$case_root"
    cp -R "$root/docs" "$root/scripts" "$root/tools" "$case_root/"
}

expect_failure() {
    local label=$1
    if check "$case_root" >/dev/null 2>&1; then
        printf 'env vocabulary mutation escaped: %s\n' "$label" >&2
        exit 1
    fi
}

new_case baseline
check "$case_root" >/dev/null

# Rule 1: a second prefix anywhere.
# Every name this file feeds the checker is assembled, never spelled, so that this file is scanned
# by the very rules it tests instead of needing an exemption from them.
retired_prefix=MISO_
family=MISO_ENGINE_BENCH_
new_case stray-prefix
printf '\n# %sRACK_BENCH_ROUND\n' "$retired_prefix" >>"$case_root/scripts/run-rack-benchmark.sh"
expect_failure stray-prefix

new_case stray-prefix-in-tool
printf '\n// %sINTERCHANGE_CANDIDATE_COMMIT\n' "$retired_prefix" \
    >>"$case_root/tools/bench/src/rack.rs"
expect_failure stray-prefix-in-tool

# Rule 2, forward: a name used but not documented. This is finding F2 -- the runner and the binary
# agreeing on a name nobody wrote down is exactly how they stopped agreeing.
new_case undocumented-name
printf '\nexport %sUNDECLARED=1\n' "$family" \
    >>"$case_root/scripts/run-rack-benchmark.sh"
expect_failure undocumented-name

# Rule 2, backward: a documented name nothing uses, and a documented name deleted from the table.
new_case unused-row
printf '| `%sABANDONED` | nothing reads this. |\n' "$family" \
    >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure unused-row

new_case deleted-row
sed -i "/^| \`${family}CPU_MODEL\`/d" "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure deleted-row

# The rule-1 exemption is exactly two paths wide: a stray name in a source or script file fails
# even though the same name in a spec does not.
new_case stray-prefix-in-doc
mkdir -p "$case_root/docs"
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
check "$case_root" >/dev/null

new_case missing-vocabulary
rm "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure missing-vocabulary

new_case symlink-vocabulary
mv "$case_root/docs/ENGINE_ENV_VOCABULARY.md" "$case_root/docs/vocabulary-target.md"
ln -s vocabulary-target.md "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure symlink-vocabulary

new_case missing-tools
rm -rf "$case_root/tools"
expect_failure missing-tools

new_case missing-scripts
rm -rf "$case_root/scripts"
missing_scripts_output="$(bash "$root/scripts/check-env-vocabulary.sh" "$case_root" 2>&1)" && missing_scripts_rc=0 || missing_scripts_rc=$?
[[ "$missing_scripts_rc" -ne 0 && "$missing_scripts_output" == *'missing required source root: scripts'* ]] || {
    printf 'missing-scripts root escaped: %s\n' "$missing_scripts_output" >&2; exit 1;
}

new_case malformed-row
sed -i "0,/^| \`${family}CPU_MODEL\`/s/\` |/ |/" "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure malformed-row

# A synonym for a fact that already has a name: the collapse #104 phase C performed must stay
# collapsed.
new_case reintroduced-synonym
printf '\nexport %sGOVERNOR=performance\n' "$family" \
    >>"$case_root/scripts/run-rack-benchmark.sh"
expect_failure reintroduced-synonym

# Git discovery and listing are separate checked callsites.  A real initialized fixture is a
# positive; a valid listing followed by an error must remain a failure.
new_case git-positive
(cd "$case_root" && git init -q && git config user.email test@example.invalid && git config user.name test && git add .)
check "$case_root" >/dev/null
mkdir "$scratch/git-list-fail"
cat >"$scratch/git-list-fail/git" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == 'ls-files -z --cached --others --exclude-standard' ]]; then
    [[ "\${ENV_MODE:-error}" == full ]] && git.real "\$@"
    exit 7
fi
exec git.real "\$@"
EOF
ln -s "$(command -v git)" "$scratch/git-list-fail/git.real"
chmod +x "$scratch/git-list-fail/git"
assert_fault() {
    local checker=$1 bin_dir=$2 expected=$3 partial=${4:-} output rc
    output="$(PATH="$bin_dir:$PATH" bash "$checker" "$case_root" 2>&1)" && rc=0 || rc=$?
    if [[ "$expected" == 'Git file listing failed' ]]; then
        printf 'Git listing assertion: mode=%s checker_status=%s partial_required=%s\n' \
            "${ENV_MODE:-unset}" "$rc" "${partial:-none}"
    fi
    if [[ "$rc" == 0 ]]; then
        printf 'env checker unexpectedly succeeded (%s)\n' "$expected" >&2
        return 86
    fi
    [[ "$output" == *"$expected"* ]] || {
        printf 'env selective fault escaped (%s): %s\n' "$expected" "$output" >&2; return 1;
    }
    [[ -z "$partial" || "$output" == *"$partial"* ]] || {
        printf 'env selective fault dropped partial output (%s): %s\n' "$expected" "$output" >&2; return 1;
    }
}

for mode in error full; do
    expected_listing=''; [[ "$mode" == full ]] && expected_listing='scripts/check-env-vocabulary.sh'
    ENV_MODE="$mode" assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/git-list-fail" \
        'Git file listing failed' "$expected_listing"
done

real_git="$(command -v git)"; real_grep="$(command -v grep)"; real_comm="$(command -v comm)"
mkdir "$scratch/classification-fail" "$scratch/source-fail" "$scratch/vocabulary-fail" "$scratch/late-comm-fail"
cat >"$scratch/classification-fail/git" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == rev-parse ]]; then [[ "\${ENV_MODE:-error}" == full ]] && "$real_git" "\$@" || true; exit 7; fi
exec "$real_git" "\$@"
EOF
cat >"$scratch/source-fail/grep" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *'scripts/run-rack-benchmark.sh'* ]]; then [[ "\${ENV_MODE:-full}" == full ]] && "$real_grep" "\$@" || true; exit 7; fi
exec "$real_grep" "\$@"
EOF
cat >"$scratch/vocabulary-fail/grep" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == -oE && "\$*" == *'ENGINE_ENV_VOCABULARY.md'* ]]; then [[ "\${ENV_MODE:-full}" == full ]] && "$real_grep" "\$@" || true; exit 7; fi
exec "$real_grep" "\$@"
EOF
cat >"$scratch/late-comm-fail/comm" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == -13 ]]; then exit 7; fi
exec "$real_comm" "\$@"
EOF
chmod +x "$scratch"/{classification-fail/git,source-fail/grep,vocabulary-fail/grep,late-comm-fail/comm}
for mode in error full; do
    ENV_MODE=$mode assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/classification-fail" 'Git classification failed (status 7)' "$([[ $mode == full ]] && printf true)"
    ENV_MODE=$mode assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/source-fail" 'source scan failed for scripts/run-rack-benchmark.sh (grep status 7)' "$([[ $mode == full ]] && printf '%sCPU_MODEL' "$family")"
    ENV_MODE=$mode assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/vocabulary-fail" 'vocabulary scan failed (grep status 7)' "$([[ $mode == full ]] && printf '%sCPU_MODEL' "$family")"
done
assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/late-comm-fail" 'unused-name comparison failed'

# Every retained environment transform/consumer is selected independently. In full mode the
# wrapper delegates first, so the checker receives the operation's complete real output and then
# the injected error.
multi="$scratch/env-operation-fault"; mkdir "$multi"
for command in find tr sed grep sort comm wc; do
    real_command="$(command -v "$command")"
    cat >"$multi/$command" <<EOF
#!/usr/bin/env bash
matched=0
case "\$ENV_STAGE:$command" in
  FIND:find) matched=1 ;;
  PATH_TR:tr) [[ "\$*" == *"\\0"* ]] && matched=1 ;;
  COUNT_TR:tr) [[ "\$*" == *"-d  "* ]] && matched=1 ;;
  VOCAB_TR:tr) [[ "\$1" == -d && "\$2" == *'|'* ]] && matched=1 ;;
  NORMALIZE:sed) matched=1 ;;
  PATH_EXCLUDE:grep) [[ "\$*" == *'ENGINE_ENV_VOCABULARY.md'* && "\$1" == -v ]] && matched=1 ;;
  ISSUE_EXCLUDE:grep) [[ "\$*" == *'ISSUE_SPECS'* ]] && matched=1 ;;
  PREFIX_FILTER:grep) [[ "\$*" == *'^MISO_ENGINE_'* && "\$1" == -v ]] && matched=1 ;;
  USED_READ:grep) [[ "\$1" == -rhoE ]] && matched=1 ;;
  FRAGMENT_FILTER:grep) [[ "\$*" == *'_$'* ]] && matched=1 ;;
  STRAY_SORT:sort) [[ "\$*" == */stray ]] && matched=1 ;;
  USED_SORT:sort) [[ "\$*" == */used-filtered ]] && matched=1 ;;
  DOCUMENTED_SORT:sort) [[ "\$*" == */documented-trimmed ]] && matched=1 ;;
  COMM23:comm) [[ "\$1" == -23 ]] && matched=1 ;;
  COMM13:comm) [[ "\$1" == -13 ]] && matched=1 ;;
  COUNT:wc) matched=1 ;;
esac
if [[ "\$matched" == 1 ]]; then
  [[ "\$ENV_MODE" == full ]] && "$real_command" "\$@" || true
  exit 7
fi
exec "$real_command" "\$@"
EOF
    chmod +x "$multi/$command"
done
while IFS='|' read -r stage diagnostic payload; do
    for mode in error full; do
        expected_payload=''; [[ "$mode" == full ]] && expected_payload="$payload"
        ENV_STAGE="$stage" ENV_MODE="$mode" assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$multi" "$diagnostic" "$expected_payload"
    done
done <<EOF
PATH_TR|path NUL conversion failed (tr status 7)|scripts/check-env-vocabulary.sh
NORMALIZE|path normalization failed (sed status 7)|scripts/check-env-vocabulary.sh
PATH_EXCLUDE|vocabulary path exclusion failed (grep status 7)|scripts/check-env-vocabulary.sh
ISSUE_EXCLUDE|issue-spec path exclusion failed (grep status 7)|scripts/check-env-vocabulary.sh
STRAY_SORT|stray-name sort failed (sort status 7)|${family}CPU_MODEL
PREFIX_FILTER|stray-name prefix filter failed (grep status 7)|
USED_READ|tools/scripts source scan failed (grep status 7)|${family}CPU_MODEL
FRAGMENT_FILTER|used-name fragment filter failed (grep status 7)|${family}CPU_MODEL
USED_SORT|used-name sort failed (sort status 7)|${family}CPU_MODEL
VOCAB_TR|vocabulary delimiter removal failed (tr status 7)|${family}CPU_MODEL
DOCUMENTED_SORT|documented-name sort failed (sort status 7)|${family}CPU_MODEL
COMM23|undocumented-name comparison failed (comm status 7)|
COMM13|unused-name comparison failed (comm status 7)|
COUNT|documented-name count failed (wc status 7)|98
COUNT_TR|documented-name count formatting failed (tr status 7)|98
EOF

saved_case_root="$case_root"; case_root="$scratch/non-git-operation"; cp -R "$saved_case_root" "$case_root"; rm -rf "$case_root/.git"
for mode in error full; do ENV_STAGE=FIND ENV_MODE=$mode assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$multi" 'fallback file traversal failed' "$([[ $mode == full ]] && printf './scripts/check-env-vocabulary.sh')"; done
case_root="$saved_case_root"

# Actual callsite counters use the same assertions that reject the production checker.
mutant="$scratch/check-env-mutant.sh"
cp "$case_root/scripts/check-env-vocabulary.sh" "$mutant"
[[ "$(grep -Fc "fail 'Git file listing failed'" "$mutant")" == 1 ]] || exit 1
sed -i "s/fail 'Git file listing failed'/:/" "$mutant"
set +e; counter_output="$(ENV_MODE=full assert_fault "$mutant" "$scratch/git-list-fail" 'Git file listing failed' 'scripts/check-env-vocabulary.sh' 2>&1)"; counter_rc=$?; set -e
if [[ "$counter_rc" != 86 || "$counter_output" != *'unexpectedly succeeded'* ]]; then
    printf 'Git listing same-assertion counter-mutant escaped\n' >&2; exit 1
fi
printf 'Git listing mutant assertion: mode=full assertion_status=%s outcome=named-unexpected-success\n' "$counter_rc"
cp "$case_root/scripts/check-env-vocabulary.sh" "$mutant"
[[ "$(grep -Fc "comm -13 \"\$used\" \"\$documented\"" "$mutant")" == 1 ]] || exit 1
sed -i '/fail "unused-name comparison failed (comm status \$rc)"/s/fail .*/:; }/' "$mutant"
set +e; counter_output="$(assert_fault "$mutant" "$scratch/late-comm-fail" 'unused-name comparison failed' 2>&1)"; counter_rc=$?; set -e
if [[ "$counter_rc" != 86 || "$counter_output" != *'unexpectedly succeeded'* ]]; then
    printf 'late comparison same-assertion counter-mutant escaped\n' >&2; exit 1
fi

check "$case_root" >/dev/null

invalid_git="$scratch/invalid-git-dir"
output="$(GIT_DIR="$invalid_git" bash "$case_root/scripts/check-env-vocabulary.sh" "$case_root" 2>&1)" && rc=0 || rc=$?
[[ "$rc" -ne 0 && "$output" == *'Git classification failed (status 128)'* && "$output" == *'not a git repository'* ]] || {
    printf 'invalid configured Git repository escaped: %s\n' "$output" >&2; exit 1;
}

empty_root="$scratch/empty-populations"
mkdir -p "$empty_root"/{docs,scripts,tools}
printf 'table\n' >"$empty_root/docs/ENGINE_ENV_VOCABULARY.md"
printf 'plain source\n' >"$empty_root/scripts/plain.sh"
output="$(bash "$root/scripts/check-env-vocabulary.sh" "$empty_root" 2>&1)" && rc=0 || rc=$?
[[ "$rc" -ne 0 && "$output" == *'no environment names used under tools/ or scripts/'* ]] || {
    printf 'empty used population escaped: %s\n' "$output" >&2; exit 1;
}
printf '%sX\n' "$family" >"$empty_root/scripts/plain.sh"
output="$(bash "$root/scripts/check-env-vocabulary.sh" "$empty_root" 2>&1)" && rc=0 || rc=$?
[[ "$rc" -ne 0 && "$output" == *'no documented environment names'* ]] || { printf 'empty documented population escaped: %s\n' "$output" >&2; exit 1; }

printf 'env vocabulary mutations: ok\n'
