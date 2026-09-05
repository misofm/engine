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
if [[ "\$*" == 'ls-files -z --cached --others --exclude-standard' ]]; then git.real "\$@"; exit 7; fi
exec git.real "\$@"
EOF
ln -s "$(command -v git)" "$scratch/git-list-fail/git.real"
chmod +x "$scratch/git-list-fail/git"
if PATH="$scratch/git-list-fail:$PATH" check "$case_root" >/dev/null 2>&1; then
    printf 'Git listing counter-control escaped\n' >&2; exit 1
fi

assert_fault() {
    local checker=$1 bin_dir=$2 expected=$3 partial=${4:-} output rc
    output="$(PATH="$bin_dir:$PATH" bash "$checker" "$case_root" 2>&1)" && rc=0 || rc=$?
    [[ "$rc" -ne 0 && "$output" == *"$expected"* ]] || {
        printf 'env selective fault escaped (%s): %s\n' "$expected" "$output" >&2; return 1;
    }
    [[ -z "$partial" || "$output" == *"$partial"* ]] || {
        printf 'env selective fault dropped partial output (%s): %s\n' "$expected" "$output" >&2; return 1;
    }
}

real_git="$(command -v git)"; real_grep="$(command -v grep)"; real_comm="$(command -v comm)"
mkdir "$scratch/classification-fail" "$scratch/source-fail" "$scratch/vocabulary-fail" "$scratch/late-comm-fail"
cat >"$scratch/classification-fail/git" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == rev-parse ]]; then printf 'classification partial\n'; exit 7; fi
exec "$real_git" "\$@"
EOF
cat >"$scratch/source-fail/grep" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *'scripts/run-rack-benchmark.sh'* ]]; then printf 'source partial\n'; exit 7; fi
exec "$real_grep" "\$@"
EOF
cat >"$scratch/vocabulary-fail/grep" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == -oE && "\$*" == *'ENGINE_ENV_VOCABULARY.md'* ]]; then printf '| \`${family}PARTIAL\`\n'; exit 7; fi
exec "$real_grep" "\$@"
EOF
cat >"$scratch/late-comm-fail/comm" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == -13 ]]; then exit 7; fi
exec "$real_comm" "\$@"
EOF
chmod +x "$scratch"/{classification-fail/git,source-fail/grep,vocabulary-fail/grep,late-comm-fail/comm}
assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/classification-fail" 'Git classification failed (status 7)' 'classification partial'
assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/source-fail" 'source scan failed for scripts/run-rack-benchmark.sh (grep status 7)' 'source partial'
assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/vocabulary-fail" 'vocabulary scan failed (grep status 7)' "${family}PARTIAL"
assert_fault "$case_root/scripts/check-env-vocabulary.sh" "$scratch/late-comm-fail" 'unused-name comparison failed'

# Actual callsite counters use the same assertions that reject the production checker.
mutant="$scratch/check-env-mutant.sh"
cp "$case_root/scripts/check-env-vocabulary.sh" "$mutant"
[[ "$(grep -Fc "fail 'Git file listing failed'" "$mutant")" == 1 ]] || exit 1
sed -i "s/fail 'Git file listing failed'/:/" "$mutant"
if counter_output="$(assert_fault "$mutant" "$scratch/git-list-fail" 'Git file listing failed' 2>&1)"; then
    printf 'Git listing same-assertion counter-mutant escaped\n' >&2; exit 1
fi
cp "$case_root/scripts/check-env-vocabulary.sh" "$mutant"
[[ "$(grep -Fc "comm -13 \"\$used\" \"\$documented\"" "$mutant")" == 1 ]] || exit 1
sed -i "/comm -13 / s/|| fail 'unused-name comparison failed'/|| true/" "$mutant"
if counter_output="$(assert_fault "$mutant" "$scratch/late-comm-fail" 'unused-name comparison failed' 2>&1)"; then
    printf 'late comparison same-assertion counter-mutant escaped\n' >&2; exit 1
fi

check "$case_root" >/dev/null

printf 'env vocabulary mutations: ok\n'
