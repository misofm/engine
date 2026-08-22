#!/usr/bin/env bash
# Static Issue 108 benchmark-repair boundary and inherited-evidence checker.
set -euo pipefail

fail() { printf 'effect interchange benchmark 108 policy failure: %s\n' "$1" >&2; return 1; }

validate_manifest_identity() {
    local manifest=$1
    [[ -f "$manifest" && ! -L "$manifest" ]] || fail 'missing accepted manifest'
    [[ "$(sha256sum "$manifest" | awk '{print $1}')" == \
        6403ae6205dbc86a57483f44723cfc107f7f49654532fc648516b7cfed7ae3a5 ]] ||
        fail 'accepted manifest changed or was refreshed'
}

validate_manifest_payload() {
    local root=$1 manifest=$2
    (cd "$root" && sha256sum --check --strict "$manifest" >/dev/null) ||
        fail 'accepted product/reference/fixture bytes changed'
}

validate_benchmark_source() {
    local benchmark=$1
    python3 -I -B - "$benchmark" <<'PY'
import pathlib, re, sys
source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
expected_rates = ["44_100", "48_000", "88_200", "96_000"]
for layout in (1, 2, 3):
    match = re.search(
        rf"static MIGRATION_Q{layout}: \[QualityDescriptorV1; 4\] = \[(.*?)\];",
        source,
        re.S,
    )
    if match is None:
        raise SystemExit(f"missing four-row migration Q{layout}")
    rows = re.findall(r"migration_quality\(([^,]+),\s*([0-9]+)\)", match.group(1))
    if rows != [(rate, str(layout)) for rate in expected_rates]:
        raise SystemExit(f"migration Q{layout} rates/layout")
if source.count("5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777") != 1:
    raise SystemExit("new migration digest count")
if source.count("350acfa6e348c27a01afcb9efbd40c51a697aac8bbb6a5fe19dc1eb3c52bf441") != 1:
    raise SystemExit("terminal Issue-081 digest count")
if r'\"issue\":108' not in source or r'\"issue\":81' in source:
    raise SystemExit("benchmark issue routing")
for token in (
    "exact_four_rate_migration_envelope_without_timing",
    "assert_eq!(output.len(), 283);",
    "EXPECTED_MIGRATION_PAYLOAD",
    "UntimedMigration",
):
    if token not in source:
        raise SystemExit(f"missing focused regression token: {token}")
PY
}

validate_successor_namespace() {
    if rg -n 'artifact_dir=.*target/issue081|effect-interchange-benchmark-validator[.]py|preflight-effect-interchange-benchmark[.]sh|run-effect-interchange-benchmark[.]sh|test-effect-interchange-benchmark[.]sh' "$@"; then
        fail 'Issue-108 authority reaches the Issue-081 lifecycle or namespace'
    fi
}

if [[ "${BASH_SOURCE[0]}" != "$0" ]]; then
    return 0
fi

[[ $# -le 1 ]] || { printf 'usage: check-effect-interchange-benchmark-108.sh [root]\n' >&2; exit 2; }
root="$(cd "${1:-.}" && pwd)"
cd "$root"
manifest=fixtures/effect-interchange/v1/ACCEPTED.sha256
validate_manifest_identity "$manifest"
validate_manifest_payload "$root" "$manifest"
[[ "$(sha256sum Cargo.lock | awk '{print $1}')" == \
    4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a ]] ||
    fail 'Cargo.lock changed'
[[ "$(sha256sum tools/miso-engine-effect-interchange-bench/Cargo.toml | awk '{print $1}')" == \
    a2b691b19fe2088611882ac632aa0c2e03948925cf189c79c184a51c30675d81 ]] ||
    fail 'benchmark manifest changed'

while read -r name bytes digest; do
    path="target/issue081/$name"
    [[ -f "$path" && ! -L "$path" ]] || fail "missing inherited Issue-081 artifact: $name"
    [[ "$(stat -c %h "$path")" == 1 ]] || fail "linked inherited Issue-081 artifact: $name"
    [[ "$(wc -c <"$path")" == "$bytes" ]] || fail "Issue-081 artifact size: $name"
    [[ "$(sha256sum "$path" | awk '{print $1}')" == "$digest" ]] ||
        fail "Issue-081 artifact identity: $name"
done <<'EOF'
nonbenchmark.seal.json 833 6d08e2089e806dc366f5c1171398c241f8dfdc520f97808c4e2f6c7f6b83363c
miso_engine_effect_interchange_bench 827232 fad8e39ecd9efa6908b51e7e98c25984f9d97f88b32971581c9a880228758b4c
benchmark-preflight.seal.json 1577 da3c537c16d55b1e71b8aa9f8e4d011796b243e4c6c7969020097098a75035a3
benchmark.raw.jsonl 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
benchmark.stderr.log 361 442f071fb23e57a9cb4616c6df7683bee669d8114eacce43b16af812e86d1a93
benchmark.disposition.json 817 8c833293bb3e9f2e981e0be1d379819786d92706627b3fa3fbc64e93b188a5de
EOF
for name in benchmark.accepted.jsonl benchmark.prelaunch.disposition.json; do
    [[ ! -e "target/issue081/$name" && ! -L "target/issue081/$name" ]] ||
        fail "forbidden inherited Issue-081 artifact appeared: $name"
done

benchmark=tools/miso-engine-effect-interchange-bench/src/main.rs
validate_benchmark_source "$benchmark"
for path in \
    scripts/effect-interchange-benchmark-108-validator.py \
    scripts/check-effect-interchange-benchmark-108.sh \
    scripts/test-effect-interchange-benchmark-108-policy.sh \
    scripts/test-effect-interchange-benchmark-108.sh \
    scripts/preflight-effect-interchange-benchmark-108.sh \
    scripts/run-effect-interchange-benchmark-108.sh; do
    [[ -f "$path" && ! -L "$path" ]] || fail "missing Issue-108 authority: $path"
done

python3 -I -B - "$benchmark" \
    scripts/effect-interchange-benchmark-108-validator.py \
    scripts/check-effect-interchange-benchmark-108.sh \
    scripts/preflight-effect-interchange-benchmark-108.sh \
    scripts/run-effect-interchange-benchmark-108.sh \
    scripts/test-effect-interchange-benchmark-108.sh <<'PY'
import pathlib, re, sys
expected = [
    ("descriptor_verify_identity_a", "865a0a5a01ba157bea7f3279ad68cc17db0296655998a9b5307cf759c38656f1"),
    ("package_verify_cid_select_a", "02e944154ccdc0315b96a7f493a11f6c60f70993750fb26ed766bc3273685d0f"),
    ("state_verify_reencode_current", "b38a9abad3da50b0c38bd02b9de19b641e79f9a8f48099fbb67d1ec3d481cf48"),
    ("migration_two_step_bank_restore", "5f23e630182137426fdfe01b74861bdff779b6738bfae8f670359ad0e9ea2777"),
]
source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
workloads = re.search(r"const WORKLOADS:.*?= \[(.*?)\];", source, re.S)
digests = re.search(r"const EXPECTED_OUTPUT_SHA256:.*?= \[(.*?)\];", source, re.S)
quoted = lambda value: re.findall(r'"([^"]+)"', value)
if workloads is None or digests is None or list(zip(quoted(workloads.group(1)), quoted(digests.group(1)))) != expected:
    raise SystemExit("tool output authority")
for path_text in sys.argv[2:]:
    text = pathlib.Path(path_text).read_text(encoding="utf-8")
    for workload, digest in expected:
        compact = f'"{workload}":"{digest}"'
        spaced = f'"{workload}": "{digest}"'
        if compact not in text and spaced not in text and not (workload in text and digest in text):
            raise SystemExit(f"output authority missing: {path_text}: {workload}")
PY

validate_successor_namespace scripts/effect-interchange-benchmark-108-validator.py \
    scripts/preflight-effect-interchange-benchmark-108.sh \
    scripts/run-effect-interchange-benchmark-108.sh \
    scripts/test-effect-interchange-benchmark-108.sh
if find target/issue108 -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null | grep -q .; then
    fail 'Issue-108 persistent artifact appeared before authorization'
fi
printf 'effect interchange benchmark 108 policy: ok counters=0/0/0/0\n'
