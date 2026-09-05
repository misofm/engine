#!/usr/bin/env bash
# Synthetic mutations for the Issue 108 repair checker. No real workload is invoked.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: test-effect-interchange-benchmark-108-policy.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
source "$root/scripts/check-effect-interchange-benchmark-108.sh"
scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT

expect_failure() {
    local label=$1
    shift
    if "$@" >/dev/null 2>&1; then
        printf 'effect interchange benchmark 108 policy mutation escaped: %s\n' "$label" >&2
        exit 1
    fi
}

mutate_source() {
    local name=$1 from=$2 to=$3
    local candidate="$scratch/$name.rs"
    cp "$root/tools/bench/src/effect_interchange.rs" "$candidate"
    python3 -I -B - "$candidate" "$from" "$to" <<'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
if sys.argv[2] not in text:
    raise SystemExit("mutation source token missing")
path.write_text(text.replace(sys.argv[2], sys.argv[3], 1), encoding="utf-8")
PY
    expect_failure "$name" validate_benchmark_source "$candidate"
}

mutate_source stale-rate 'migration_quality(44_100, 1)' 'migration_quality(176_400, 1)'
mutate_source duplicate-rate 'migration_quality(88_200, 2)' 'migration_quality(48_000, 2)'
mutate_source missing-rate 'migration_quality(96_000, 3),' ''
mutate_source old-digest \
    '5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777' \
    '350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441'
mutate_source issue-81 '\"issue\":108' '\"issue\":81'

namespace="$scratch/effect-interchange-benchmark-108-validator.py"
cp "$root/scripts/effect-interchange-benchmark-108-validator.py" "$namespace"
printf '\n# artifact_dir=target/issue081\n' >>"$namespace"
expect_failure issue081-namespace validate_successor_namespace "$namespace"

# The sourced namespace function must preserve a real rg execution error even when its caller is
# a conditional subshell. The wrapper emits a sentinel and status 2 only for this candidate.
namespace_error="$scratch/effect-interchange-benchmark-108-validator-error.py"
cp "$root/scripts/effect-interchange-benchmark-108-validator.py" "$namespace_error"
printf '\n# namespace-error-sentinel\n' >>"$namespace_error"
rg_bin="$(command -v rg)"
mkdir -p "$scratch/namespace-bin"
cat >"$scratch/namespace-bin/rg" <<EOF
#!/usr/bin/env bash
if [[ "\${@: -1}" == *effect-interchange-benchmark-108-validator-error.py ]]; then
    output="\$(mktemp)"
    if "$rg_bin" "\$@" >"\$output"; then status=0; else status=\$?; fi
    if [[ "\$status" -ne 1 || -s "\$output" ]]; then
        printf 'namespace-wrapper-wrong-delegate status=%s bytes=%s\\n' "\$status" "\$(wc -c <"\$output")" >&2
        rm -f "\$output"; exit 72
    fi
    rm -f "\$output"
    printf '%s\\n' namespace-error-sentinel >&2
    exit 73
fi
exec "$rg_bin" "\$@"
EOF
chmod 755 "$scratch/namespace-bin/rg"
assert_namespace_error() {
    local checker=$1 log="$scratch/namespace-control.log" status
    if env PATH="$scratch/namespace-bin:$PATH" bash -c \
        'source "$1"; (validate_benchmark_source "$2" || exit $?; validate_successor_namespace "$3" || exit $?)' \
        _ "$checker" "$root/tools/bench/src/effect_interchange.rs" "$namespace_error" >"$log" 2>&1; then status=0; else status=$?; fi
    if [[ "$status" -eq 0 ]]; then
        printf 'effect interchange benchmark 108 policy: namespace status-loss unexpectedly succeeded\n' >&2
        return 97
    fi
    [[ "$status" -eq 73 ]] || return 96
    rg -F namespace-error-sentinel "$log" >/dev/null || return 96
    rg -F 'namespace scan failed (rg status 73)' "$log" >/dev/null || return 96
}
assert_namespace_error "$root/scripts/check-effect-interchange-benchmark-108.sh" || exit $?
mutant="$scratch/check-effect-interchange-benchmark-108-mutant.sh"
cp "$root/scripts/check-effect-interchange-benchmark-108.sh" "$mutant"
sed -i 's/if \[\[ "$status" -ne 1 \]\]; then/if [[ "$status" -eq 0 ]]; then/' "$mutant"
[[ "$(rg -F -c 'if [[ "$status" -ne 1 ]]; then' "$root/scripts/check-effect-interchange-benchmark-108.sh")" -eq 1 ]] || exit 96
[[ "$(rg -F -c 'if [[ "$status" -eq 0 ]]; then' "$mutant")" -eq 1 ]] || exit 96
[[ "$(rg -F -c 'if [[ "$status" -ne 1 ]]; then' "$mutant")" -eq 0 ]] || exit 96
if assert_namespace_error "$mutant"; then
    printf 'effect interchange benchmark 108 policy: namespace status-loss mutant did not escape\n' >&2
    exit 96
else
    mutant_status=$?
    [[ "$mutant_status" -eq 97 ]] || exit 96
fi
assert_namespace_error "$root/scripts/check-effect-interchange-benchmark-108.sh" || exit $?

# A failing first sourced function must not be overwritten by a clean namespace result in the
# actual conditional sequence used by the qualification checker.
conditional_log="$scratch/preceding-source-validation.log"
if bash -c 'source "$1"; (validate_benchmark_source /dev/null || exit $?; validate_successor_namespace "$2" || exit $?)' \
    _ "$root/scripts/check-effect-interchange-benchmark-108.sh" \
    "$root/scripts/effect-interchange-benchmark-108-validator.py" >"$conditional_log" 2>&1; then
    printf 'effect interchange benchmark 108 policy mutation escaped: preceding source validation\n' >&2
    exit 97
fi
rg -F 'missing four-row migration Q1' "$conditional_log" >/dev/null || exit 96

# Standalone tail: absence and an existing empty directory are both valid. For traversal faults,
# the wrapper proves the real find would return 0 with complete empty output before injecting the
# error; a simultaneous prohibited row must not displace the tool-failure diagnostic.
standalone="$scratch/standalone"
mkdir -p "$standalone/scripts" "$standalone/tools/bench/src"
cp "$root/scripts/check-effect-interchange-benchmark-108.sh" \
    "$root/scripts/effect-interchange-benchmark-108-validator.py" \
    "$root/scripts/test-effect-interchange-benchmark-108-policy.sh" "$standalone/scripts/"
cp "$root/tools/bench/src/effect_interchange.rs" "$standalone/tools/bench/src/"
bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >/dev/null
mkdir -p "$standalone/target/issue108"
bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >/dev/null

mkdir -p "$scratch/python-bin"
python_bin=$(command -v python3)
cat >"$scratch/python-bin/python3" <<EOF
#!/usr/bin/env bash
count=0
[[ ! -f "$scratch/python-count" ]] || read -r count <"$scratch/python-count"
count=\$((count + 1)); printf '%s\\n' "\$count" >"$scratch/python-count"
if [[ "\$count" -eq "\$MISO_ENGINE_INTERCHANGE_TEST_PYTHON_OCCURRENCE" ]]; then
    output="\$(mktemp)"; error="\$(mktemp)"
    if "$python_bin" "\$@" >"\$output" 2>"\$error"; then status=0; else status=\$?; fi
    if [[ "\$status" -ne 0 || -s "\$output" || -s "\$error" ]]; then
        printf 'standalone-python-wrapper-setup status=%s\\n' "\$status" >&2; exit 72
    fi
    printf 'standalone-python-error-sentinel\\n' >&2
    exit 75
fi
exec "$python_bin" "\$@"
EOF
chmod 755 "$scratch/python-bin/python3"
for row in '1|benchmark source validation failed (status 75)' '2|cross-file output authority validation failed (status 75)'; do
    IFS='|' read -r occurrence operation <<<"$row"
    printf '0\n' >"$scratch/python-count"
    python_log="$scratch/python-$occurrence.log"
    if MISO_ENGINE_INTERCHANGE_TEST_PYTHON_OCCURRENCE="$occurrence" PATH="$scratch/python-bin:$PATH" \
        bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >"$python_log" 2>&1; then
        printf 'effect interchange benchmark 108 Python fault unexpectedly succeeded: %s\n' "$occurrence" >&2; exit 97
    fi
    rg -F standalone-python-error-sentinel "$python_log" >/dev/null || exit 96
    rg -F "$operation" "$python_log" >/dev/null || exit 96
done

# The second Python authority read must be exercised by the unchanged validator program itself.
# The first invocation remains an unchanged success; only Path.read_text for the exact validator
# path is denied in a bootstrap around the captured stdin program.
hook_count="$scratch/python-hook-count"
hook_program="$scratch/python-hook-program.py"
hook_bootstrap="$scratch/python-read-hook.py"
hook_marker="$scratch/python-read-reached"
cat >"$scratch/python-bin/python3-hook" <<EOF
#!/usr/bin/env bash
count=0
[[ ! -f "$hook_count" ]] || read -r count <"$hook_count"
count=\$((count + 1)); printf '%s\\n' "\$count" >"$hook_count"
if [[ "\$count" -eq 2 ]]; then
    args=("\$@")
    cat >"$hook_program"
    exec "$python_bin" -I -B "$hook_bootstrap" "\${args[@]:3}"
fi
exec "$python_bin" "\$@"
EOF
chmod 755 "$scratch/python-bin/python3-hook"
cat >"$hook_bootstrap" <<'PY'
import os, pathlib, sys
target = pathlib.Path(os.environ["MISO_ENGINE_INTERCHANGE_TEST_READ_TARGET"]).resolve()
marker = pathlib.Path(os.environ["MISO_ENGINE_INTERCHANGE_TEST_READ_MARKER"])
original = pathlib.Path.read_text
def read_text(self, *args, **kwargs):
    if self.resolve() == target:
        marker.write_text("reached-read-hook\n", encoding="utf-8")
        raise PermissionError(f"Permission denied: {self.resolve()}")
    return original(self, *args, **kwargs)
pathlib.Path.read_text = read_text
sys.argv = ["-"] + sys.argv[1:]
program = pathlib.Path(os.environ["MISO_ENGINE_INTERCHANGE_TEST_READ_PROGRAM"]).read_text(encoding="utf-8")
exec(compile(program, "<stdin>", "exec"), {"__name__": "__main__", "__file__": "<stdin>"})
PY
: >"$hook_count"
rm -f "$hook_marker"
hook_log="$scratch/python-second-read.log"
rm -f "$scratch/python-bin/python3"
ln -s python3-hook "$scratch/python-bin/python3"
if MISO_ENGINE_INTERCHANGE_TEST_READ_TARGET="$standalone/scripts/effect-interchange-benchmark-108-validator.py" \
    MISO_ENGINE_INTERCHANGE_TEST_READ_MARKER="$hook_marker" MISO_ENGINE_INTERCHANGE_TEST_READ_PROGRAM="$hook_program" \
    PATH="$scratch/python-bin:$PATH" bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >"$hook_log" 2>&1; then
    printf 'effect interchange benchmark 108 Python read unexpectedly succeeded\n' >&2; exit 97
else hook_status=$?; fi
[[ "$hook_status" -eq 1 ]] || { cat "$hook_log" >&2; exit 96; }
rg -F 'cross-file output authority validation failed (status 1)' "$hook_log" >/dev/null || exit 96
rg -F 'PermissionError' "$hook_log" >/dev/null || exit 96
rg -F "$standalone/scripts/effect-interchange-benchmark-108-validator.py" "$hook_log" >/dev/null || exit 96
[[ -s "$hook_marker" ]] || exit 96
rg -F reached-read-hook "$hook_marker" >/dev/null || exit 96
rm -f "$scratch/python-bin/python3"
bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >/dev/null

precise_standalone_failure() {
    local label=$1 diagnostic=$2 log="$scratch/deletion-$1.log" status
    if bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >"$log" 2>&1; then
        printf 'effect interchange benchmark 108 deletion unexpectedly succeeded: %s\n' "$label" >&2; exit 97
    else status=$?; fi
    [[ "$status" -eq 1 ]] && rg -F "$diagnostic" "$log" >/dev/null || { cat "$log" >&2; exit 96; }
}
mv "$standalone/tools/bench/src/effect_interchange.rs" "$standalone/tools/bench/src/effect_interchange.rs.saved"
precise_standalone_failure benchmark-source 'missing benchmark source'
mv "$standalone/tools/bench/src/effect_interchange.rs.saved" "$standalone/tools/bench/src/effect_interchange.rs"
mv "$standalone/scripts/effect-interchange-benchmark-108-validator.py" "$standalone/scripts/effect-interchange-benchmark-108-validator.py.saved"
precise_standalone_failure validator 'missing Issue-108 authority'
mv "$standalone/scripts/effect-interchange-benchmark-108-validator.py.saved" "$standalone/scripts/effect-interchange-benchmark-108-validator.py"
mkdir -p "$scratch/find-bin"
find_bin=$(command -v find)
cat >"$scratch/find-bin/find" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == target/issue108 ]]; then
    output="\$(mktemp)"
    if "$find_bin" "\$@" >"\$output"; then status=0; else status=\$?; fi
    if [[ "\$status" -ne 0 ]]; then printf 'optional-find-wrapper-setup status=%s\\n' "\$status" >&2; rm -f "\$output"; exit 72; fi
    if [[ "\${MISO_ENGINE_INTERCHANGE_TEST_OPTIONAL_FIND_MODE:-empty}" == empty && -s "\$output" ]] || \
       [[ "\${MISO_ENGINE_INTERCHANGE_TEST_OPTIONAL_FIND_MODE:-empty}" == violation && ! -s "\$output" ]]; then
        printf 'optional-find-wrapper-wrong-shape\\n' >&2; rm -f "\$output"; exit 72
    fi
    cat "\$output"; rm -f "\$output"
    printf 'optional-find-error-sentinel\\n' >&2
    exit 74
fi
exec "$find_bin" "\$@"
EOF
chmod 755 "$scratch/find-bin/find"
for mode in empty violation; do
    if [[ "$mode" == violation ]]; then printf 'real optional violation\n' >"$standalone/target/issue108/actual-entry"; fi
    optional_log="$scratch/optional-find-$mode.log"
    if MISO_ENGINE_INTERCHANGE_TEST_OPTIONAL_FIND_MODE="$mode" PATH="$scratch/find-bin:$PATH" \
        bash "$standalone/scripts/check-effect-interchange-benchmark-108.sh" "$standalone" >"$optional_log" 2>&1; then
        printf 'effect interchange benchmark 108 policy mutation escaped: optional find %s\n' "$mode" >&2; exit 97
    fi
    rg -F optional-find-error-sentinel "$optional_log" >/dev/null || exit 96
    rg -F 'artifact traversal failed (status 74)' "$optional_log" >/dev/null || exit 96
    if [[ "$mode" == violation ]]; then
        rg -F 'target/issue108/actual-entry' "$optional_log" >/dev/null || exit 96
        rm "$standalone/target/issue108/actual-entry"
    fi
done

# The accepted-manifest identity and payload mutations moved to
# `scripts/test-effect-interchange-policy.sh` together with the manifest itself (#104 phase A):
# `check-effect-interchange-qualification.sh` owns that seal and this checker no longer duplicates
# it.

printf 'effect interchange benchmark 108 policy mutations: ok synthetic_only=1\n'
