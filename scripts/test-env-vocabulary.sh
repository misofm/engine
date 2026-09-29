#!/usr/bin/env bash
# Mutations for the environment/marker vocabulary gate (#104 phase C; one `git grep` pass, #1043).
#
# Hermetic (#1043): every case runs the gate on a small tree this file builds, never on a copy of
# the live docs/, scripts/ and tools/. The suite therefore reads nothing but the gate and itself --
# exactly its router key (scripts/ci-path-router.py SELF_TEST_INPUTS) -- and no live file, name
# or row count can turn it red. The documented-name count the gate reports is the fixture's own,
# derived from the list below rather than pinned.
set -euo pipefail

root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
gate="$root/scripts/check-env-vocabulary.sh"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

# Every name this file feeds the gate is assembled, never spelled, so that the live gate reads this
# file under the very rules it tests instead of needing an exemption from them.
retired_prefix=MISO_
family=MISO_ENGINE_
round="${family}FIXTURE_ROUND"
cpu_model="${family}FIXTURE_CPU_MODEL"
power_mode="${family}FIXTURE_POWER_MODE"
audit_begin="${family}FIXTURE_AUDIT_BEGIN"
documented=("$round" "$cpu_model" "$power_mode" "$audit_begin")

base="$scratch/base"
mkdir -p "$base"/{docs,scripts,tools/bench/src,crates/engine/src,.github/ISSUE_SPECS}
{
    printf '# Environment vocabulary fixture\n\n| name | meaning |\n|---|---|\n'
    for name in "${documented[@]}"; do printf '| `%s` | fixture row. |\n' "$name"; done
} >"$base/docs/ENGINE_ENV_VOCABULARY.md"
printf 'Prose that names no environment variable.\n' >"$base/docs/guide.md"
printf '#!/usr/bin/env bash\nexport %s=1\nexport %s="$(uname -m)"\nexport %s=performance\n' \
    "$round" "$cpu_model" "$power_mode" >"$base/scripts/fixture-runner.sh"
printf 'fn main() { let _ = (std::env::var("%s"), "%s"); }\n' "$cpu_model" "$audit_begin" \
    >"$base/tools/bench/src/fixture.rs"
printf 'pub fn clean() {}\n' >"$base/crates/engine/src/lib.rs"
printf 'A spec may quote a retired name.\n' >"$base/.github/ISSUE_SPECS/0001-fixture.md"

check() { bash "$gate" "$1"; }

new_case() {
    case_root="$scratch/$1"
    cp -R "$base" "$case_root"
}

expect_failure() {
    local label=$1 expected=$2 output rc
    output="$(check "$case_root" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" -ne 0 && "$output" == *"$expected"* ]] || {
        printf 'env vocabulary mutation escaped (%s): %s\n' "$label" "$output" >&2; exit 1;
    }
}

expect_pass() {
    local label=$1 output
    output="$(check "$case_root" 2>&1)" || {
        printf 'env vocabulary rejected a clean case (%s): %s\n' "$label" "$output" >&2; exit 1;
    }
}

stray_rule="identifier outside the ${family} prefix"
undocumented_rule='name used under tools/ or scripts/ but absent from docs/ENGINE_ENV_VOCABULARY.md'
unused_rule='name documented in docs/ENGINE_ENV_VOCABULARY.md but unused under tools/ or scripts/'

# The report counts every documented name: the fixture's own count, not a pinned number.
new_case baseline
baseline_output="$(check "$case_root")"
[[ "$baseline_output" == "env vocabulary: ok (${#documented[@]} names, one ${family} prefix)" ]] || {
    printf 'env vocabulary baseline report is wrong: %s\n' "$baseline_output" >&2; exit 1;
}

# Rule 1: a second prefix anywhere in the scanned path set, which is every file but two.
new_case stray-prefix
printf '\n# %sRACK_BENCH_ROUND\n' "$retired_prefix" >>"$case_root/scripts/fixture-runner.sh"
expect_failure stray-prefix "$stray_rule"

new_case stray-prefix-in-tool
printf '\n// %sINTERCHANGE_CANDIDATE_COMMIT\n' "$retired_prefix" >>"$case_root/tools/bench/src/fixture.rs"
expect_failure stray-prefix-in-tool "$stray_rule"

new_case stray-prefix-in-crate
printf '\n// %sWEB_STRIP\n' "$retired_prefix" >>"$case_root/crates/engine/src/lib.rs"
expect_failure stray-prefix-in-crate "$stray_rule"

new_case stray-prefix-in-other-doc
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$case_root/docs/guide.md"
expect_failure stray-prefix-in-other-doc "$stray_rule"

# The rule-1 exemption is exactly two paths wide: the vocabulary itself and the issue specs.
new_case stray-prefix-in-vocabulary
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_pass stray-prefix-in-vocabulary

new_case stray-prefix-in-issue-spec
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$case_root/.github/ISSUE_SPECS/0001-fixture.md"
expect_pass stray-prefix-in-issue-spec

# Outside a work tree the path set is every regular file except `.git/` and the top-level
# `target/`, as the former `find` listing was; a nested `target/` is still read.
new_case stray-prefix-in-top-target
mkdir -p "$case_root/target"; printf '%sWEB_STRIP\n' "$retired_prefix" >"$case_root/target/spill.txt"
expect_pass stray-prefix-in-top-target

new_case stray-prefix-in-nested-target
mkdir -p "$case_root/crates/engine/target"
printf '%sWEB_STRIP\n' "$retired_prefix" >"$case_root/crates/engine/target/spill.txt"
expect_failure stray-prefix-in-nested-target "$stray_rule"

# Rule 2, forward: a name used but not documented. This is finding F2 -- the runner and the binary
# agreeing on a name nobody wrote down is exactly how they stopped agreeing.
new_case undocumented-name
printf '\nexport %sUNDECLARED=1\n' "$family" >>"$case_root/scripts/fixture-runner.sh"
expect_failure undocumented-name "$undocumented_rule"

new_case undocumented-name-in-tool
printf '\nconst NAME: &str = "%sUNDECLARED";\n' "$family" >>"$case_root/tools/bench/src/fixture.rs"
expect_failure undocumented-name-in-tool "$undocumented_rule"

# A synonym for a fact that already has a name: the collapse #104 phase C performed must stay
# collapsed.
new_case reintroduced-synonym
printf '\nexport %sFIXTURE_GOVERNOR=performance\n' "$family" >>"$case_root/scripts/fixture-runner.sh"
expect_failure reintroduced-synonym "$undocumented_rule"

# Rule 2, backward: a documented name nothing uses, one used only outside tools/ and scripts/, and
# a used name whose row is deleted or malformed.
new_case unused-row
printf '| `%sABANDONED` | nothing reads this. |\n' "$family" >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure unused-row "$unused_rule"

new_case used-only-in-a-crate
printf '| `%sCRATE_ONLY` | only a crate reads this. |\n' "$family" >>"$case_root/docs/ENGINE_ENV_VOCABULARY.md"
printf '\nconst NAME: &str = "%sCRATE_ONLY";\n' "$family" >>"$case_root/crates/engine/src/lib.rs"
expect_failure used-only-in-a-crate "$unused_rule"

new_case deleted-row
sed -i "/^| \`${cpu_model}\`/d" "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure deleted-row "$undocumented_rule"

new_case malformed-row
sed -i "0,/^| \`${cpu_model}\`/s/\` |/ |/" "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure malformed-row "$undocumented_rule"

# A family prefix written on its own (a fragment ending in `_`) is not a used name.
new_case fragment
printf '\nfamily=%sFIXTURE_\n' "$family" >>"$case_root/scripts/fixture-runner.sh"
expect_pass fragment

new_case missing-vocabulary
rm "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure missing-vocabulary 'missing vocabulary: docs/ENGINE_ENV_VOCABULARY.md'

new_case symlink-vocabulary
mv "$case_root/docs/ENGINE_ENV_VOCABULARY.md" "$case_root/docs/vocabulary-target.md"
ln -s vocabulary-target.md "$case_root/docs/ENGINE_ENV_VOCABULARY.md"
expect_failure symlink-vocabulary 'missing vocabulary: docs/ENGINE_ENV_VOCABULARY.md'

new_case missing-tools
rm -rf "$case_root/tools"
expect_failure missing-tools 'missing required source root: tools'

new_case missing-scripts
rm -rf "$case_root/scripts"
expect_failure missing-scripts 'missing required source root: scripts'

# In a work tree the path set is the tracked files plus the untracked files Git does not ignore
# (`git ls-files --cached --others --exclude-standard`).
new_case git-positive
printf '/target/\nignored.txt\n' >"$case_root/.gitignore"
(cd "$case_root" && git init -q && git add -A)
expect_pass git-positive
git_root="$case_root"
printf '%sWEB_STRIP\n' "$retired_prefix" >"$git_root/untracked.txt"
expect_failure git-untracked "$stray_rule"
rm "$git_root/untracked.txt"
printf '%sWEB_STRIP\n' "$retired_prefix" >"$git_root/ignored.txt"
mkdir -p "$git_root/target"; printf '%sWEB_STRIP\n' "$retired_prefix" >"$git_root/target/spill.txt"
expect_pass git-ignored
cp "$git_root/.github/ISSUE_SPECS/0001-fixture.md" "$scratch/issue-spec.md"
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$git_root/.github/ISSUE_SPECS/0001-fixture.md"
expect_pass git-issue-spec
cp "$scratch/issue-spec.md" "$git_root/.github/ISSUE_SPECS/0001-fixture.md"
cp "$git_root/crates/engine/src/lib.rs" "$scratch/lib.rs"
printf '\n%sWEB_STRIP\n' "$retired_prefix" >>"$git_root/crates/engine/src/lib.rs"
expect_failure git-tracked "$stray_rule"
cp "$scratch/lib.rs" "$git_root/crates/engine/src/lib.rs"
expect_pass git-restored
plain_root="$scratch/plain"; cp -R "$base" "$plain_root"

# Selective tool faults. Every external command the gate runs is failed on its own; in full mode the
# wrapper delegates first, so the gate receives the operation's complete real output and then the
# injected error, and must still fail and show that output. SCAN_STDERR is `git grep`'s own way of
# failing: it reports a file it cannot read on stderr and exits 0.
fault="$scratch/fault"; mkdir "$fault"
for command in git grep sort tr comm wc; do
    real_command="$(command -v "$command")"
    cat >"$fault/$command" <<EOF
#!/usr/bin/env bash
matched=0
case "\${ENV_STAGE:-}:$command" in
  CLASSIFY:git) [[ "\$1" == rev-parse ]] && matched=1 ;;
  SCAN:git|SCAN_STDERR:git) [[ "\$1" == grep ]] && matched=1 ;;
  STRAY_SORT:sort) [[ "\$*" == */stray ]] && matched=1 ;;
  PREFIX_FILTER:grep) [[ "\$1" == -v && "\$*" == *'^${family}'* ]] && matched=1 ;;
  USED_READ:grep) [[ "\$1" == -rhoE ]] && matched=1 ;;
  FRAGMENT_FILTER:grep) [[ "\$*" == *'_\$'* ]] && matched=1 ;;
  USED_SORT:sort) [[ "\$*" == */used-filtered ]] && matched=1 ;;
  VOCAB_GREP:grep) [[ "\$1" == -oE && "\$*" == *'ENGINE_ENV_VOCABULARY.md'* ]] && matched=1 ;;
  VOCAB_TR:tr) [[ "\$1" == -d && "\$2" == *'|'* ]] && matched=1 ;;
  DOCUMENTED_SORT:sort) [[ "\$*" == */documented-trimmed ]] && matched=1 ;;
  COMM23:comm) [[ "\$1" == -23 ]] && matched=1 ;;
  COMM13:comm) [[ "\$1" == -13 ]] && matched=1 ;;
  COUNT:wc) matched=1 ;;
  COUNT_TR:tr) [[ "\$*" == *"-d  "* ]] && matched=1 ;;
esac
if [[ "\$matched" == 1 ]]; then
  [[ "\${ENV_MODE:-error}" == full ]] && "$real_command" "\$@" || true
  if [[ "\$ENV_STAGE" == SCAN_STDERR ]]; then
    printf 'error: failed to stat %s: Permission denied\n' unreadable.txt >&2
    exit 0
  fi
  exit 7
fi
exec "$real_command" "\$@"
EOF
    chmod +x "$fault/$command"
done

# 86 is "the gate passed when it should have failed", so the counter-mutants below can tell that
# outcome from a wrong diagnostic.
assert_fault() {
    local checker=$1 case_dir=$2 expected=$3 partial=${4:-} output rc
    output="$(PATH="$fault:$PATH" bash "$checker" "$case_dir" 2>&1)" && rc=0 || rc=$?
    if [[ "$rc" == 0 ]]; then
        printf 'env checker unexpectedly succeeded (%s %s/%s)\n' "$expected" "${ENV_STAGE:-}" "${ENV_MODE:-}" >&2
        return 86
    fi
    [[ "$output" == *"$expected"* ]] || {
        printf 'env selective fault escaped (%s): %s\n' "$expected" "$output" >&2; return 1;
    }
    [[ -z "$partial" || "$output" == *"$partial"* ]] || {
        printf 'env selective fault dropped partial output (%s): %s\n' "$expected" "$output" >&2; return 1;
    }
}

count="${#documented[@]}"
for tree in git plain; do
    tree_root="$git_root"; scan_mode=--untracked
    [[ "$tree" == plain ]] && { tree_root="$plain_root"; scan_mode=--no-index; }
    while IFS='|' read -r stage diagnostic payload; do
        for mode in error full; do
            expected_payload=''; [[ "$mode" == full || "$stage" == SCAN_STDERR ]] && expected_payload="$payload"
            ENV_STAGE="$stage" ENV_MODE="$mode" assert_fault "$gate" "$tree_root" "$diagnostic" "$expected_payload"
        done
    done <<EOF
SCAN|source scan failed (git grep $scan_mode status 7)|$cpu_model
SCAN_STDERR|source scan failed (git grep $scan_mode status 0)|failed to stat unreadable.txt
STRAY_SORT|stray-name sort failed (sort status 7)|$cpu_model
PREFIX_FILTER|stray-name prefix filter failed (grep status 7)|
USED_READ|tools/scripts source scan failed (grep status 7)|$cpu_model
FRAGMENT_FILTER|used-name fragment filter failed (grep status 7)|$cpu_model
USED_SORT|used-name sort failed (sort status 7)|$cpu_model
VOCAB_GREP|vocabulary scan failed (grep status 7)|$cpu_model
VOCAB_TR|vocabulary delimiter removal failed (tr status 7)|$cpu_model
DOCUMENTED_SORT|documented-name sort failed (sort status 7)|$cpu_model
COMM23|undocumented-name comparison failed (comm status 7)|
COMM13|unused-name comparison failed (comm status 7)|
COUNT|documented-name count failed (wc status 7)|$count
COUNT_TR|documented-name count formatting failed (tr status 7)|$count
EOF
done
for mode in error full; do
    ENV_STAGE=CLASSIFY ENV_MODE="$mode" assert_fault "$gate" "$git_root" 'Git classification failed (status 7)' \
        "$([[ "$mode" == full ]] && printf true)"
done

# The assertion helper itself must be able to fail: a gate whose scan failure is swallowed, and one
# whose late comparison failure is swallowed, each pass under the same fault and are reported as
# the named unexpected success (86), not as a pass.
mutant="$scratch/check-env-mutant.sh"
cp "$gate" "$mutant"
[[ "$(grep -Fc 'fail "source scan failed (git grep $mode status $rc)"' "$mutant")" == 1 ]] || exit 1
sed -i 's/fail "source scan failed (git grep \$mode status \$rc)"/:/' "$mutant"
set +e; counter_output="$(ENV_STAGE=SCAN ENV_MODE=full assert_fault "$mutant" "$git_root" 'source scan failed' 2>&1)"; counter_rc=$?; set -e
if [[ "$counter_rc" != 86 || "$counter_output" != *'unexpectedly succeeded'* ]]; then
    printf 'scan-failure counter-mutant escaped the fault assertion: %s\n' "$counter_output" >&2; exit 1
fi
cp "$gate" "$mutant"
[[ "$(grep -Fc 'fail "unused-name comparison failed (comm status $rc)"' "$mutant")" == 1 ]] || exit 1
sed -i '/fail "unused-name comparison failed (comm status \$rc)"/s/fail .*/:; }/' "$mutant"
set +e; counter_output="$(ENV_STAGE=COMM13 ENV_MODE=error assert_fault "$mutant" "$git_root" 'unused-name comparison failed' 2>&1)"; counter_rc=$?; set -e
if [[ "$counter_rc" != 86 || "$counter_output" != *'unexpectedly succeeded'* ]]; then
    printf 'late comparison counter-mutant escaped the fault assertion: %s\n' "$counter_output" >&2; exit 1
fi

output="$(GIT_DIR="$scratch/invalid-git-dir" bash "$gate" "$git_root" 2>&1)" && rc=0 || rc=$?
[[ "$rc" -ne 0 && "$output" == *'Git classification failed (status 128)'* && "$output" == *'not a git repository'* ]] || {
    printf 'invalid configured Git repository escaped: %s\n' "$output" >&2; exit 1;
}

empty_root="$scratch/empty-populations"
mkdir -p "$empty_root"/{docs,scripts,tools}
printf 'table\n' >"$empty_root/docs/ENGINE_ENV_VOCABULARY.md"
printf 'plain source\n' >"$empty_root/scripts/fixture-runner.sh"
output="$(bash "$gate" "$empty_root" 2>&1)" && rc=0 || rc=$?
[[ "$rc" -ne 0 && "$output" == *'no environment names used under tools/ or scripts/'* ]] || {
    printf 'empty used population escaped: %s\n' "$output" >&2; exit 1;
}
printf '%sX\n' "$family" >"$empty_root/scripts/fixture-runner.sh"
output="$(bash "$gate" "$empty_root" 2>&1)" && rc=0 || rc=$?
[[ "$rc" -ne 0 && "$output" == *'no documented environment names'* ]] || { printf 'empty documented population escaped: %s\n' "$output" >&2; exit 1; }

printf 'env vocabulary mutations: ok\n'
