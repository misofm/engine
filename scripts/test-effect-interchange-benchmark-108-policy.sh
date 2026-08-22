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
    cp "$root/tools/miso-engine-effect-interchange-bench/src/main.rs" "$candidate"
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

namespace="$scratch/run-effect-interchange-benchmark-108.sh"
cp "$root/scripts/run-effect-interchange-benchmark-108.sh" "$namespace"
python3 -I -B - "$namespace" <<'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
path.write_text(path.read_text(encoding="utf-8").replace("target/issue108", "target/issue081"), encoding="utf-8")
PY
expect_failure issue081-namespace validate_successor_namespace "$namespace"

refreshed="$scratch/ACCEPTED.sha256"
cp "$root/fixtures/effect-interchange/v1/ACCEPTED.sha256" "$refreshed"
printf '%s\n' '# refreshed' >>"$refreshed"
expect_failure accepted-manifest-refresh validate_manifest_identity "$refreshed"

product_root="$scratch/product"
mkdir -p "$product_root/crates/example"
printf 'accepted product\n' >"$product_root/crates/example/lib.rs"
(cd "$product_root" && sha256sum crates/example/lib.rs >ACCEPTED.sha256)
validate_manifest_payload "$product_root" ACCEPTED.sha256
printf 'mutated product\n' >"$product_root/crates/example/lib.rs"
expect_failure production-edit validate_manifest_payload "$product_root" ACCEPTED.sha256

printf 'effect interchange benchmark 108 policy mutations: ok synthetic_only=1\n'
