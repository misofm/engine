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
if "$rg_bin" -q namespace-error-sentinel "\${@: -1}" >/dev/null 2>&1; then
    printf '%s\\n' namespace-error-sentinel >&2
    exit 2
fi
exec "$rg_bin" "\$@"
EOF
chmod 755 "$scratch/namespace-bin/rg"
assert_namespace_error() {
    if "$@"; then
        return 97
    else
        local status=$?
    fi
    [[ "$status" -eq 1 || "$status" -eq 2 ]] || return 96
}
assert_namespace_error env PATH="$scratch/namespace-bin:$PATH" bash -c \
    'source "$1"; validate_successor_namespace "$2"' _ \
    "$root/scripts/check-effect-interchange-benchmark-108.sh" "$namespace_error"
mutant="$scratch/check-effect-interchange-benchmark-108-mutant.sh"
cp "$root/scripts/check-effect-interchange-benchmark-108.sh" "$mutant"
sed -i 's/local status=\$?/local status=1/' "$mutant"
if assert_namespace_error env PATH="$scratch/namespace-bin:$PATH" bash -c \
    'source "$1"; validate_successor_namespace "$2"' _ "$mutant" "$namespace_error"; then
    printf 'effect interchange benchmark 108 policy: namespace status-loss mutant escaped\n' >&2
    exit 96
else
    mutant_status=$?
    [[ "$mutant_status" -eq 97 ]] || exit 96
fi

# The accepted-manifest identity and payload mutations moved to
# `scripts/test-effect-interchange-policy.sh` together with the manifest itself (#104 phase A):
# `check-effect-interchange-qualification.sh` owns that seal and this checker no longer duplicates
# it.

printf 'effect interchange benchmark 108 policy mutations: ok synthetic_only=1\n'
