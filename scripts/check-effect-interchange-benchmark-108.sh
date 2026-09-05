#!/usr/bin/env bash
# Issue 108 benchmark source-authority checker.
#
# #104 phase A / #83 wave-4 decision W4-D2. This file used to open with an unrefreshable seal: the
# sha256 of `Cargo.lock`, the sha256 of the benchmark package manifest, the identity and payload of
# `fixtures/effect-interchange/v1/ACCEPTED.sha256` (which `check-effect-interchange-qualification.sh`
# already owns), and the byte identity of seven `target/issue081/` build artifacts that only ever
# existed in the branch that produced them. Every one of those went permanently red once waves 1-4
# touched the workspace lock and the effect-package sources, and none can be refreshed without
# re-running the sealed Issue-081 run. They are retired; the hashes stay in
# `.github/ISSUE_SPECS/108-*.md`.
#
# What survives is the live half: the benchmark source must still declare the four-rate migration
# envelope, the four workloads with their expected output digests, and the focused regression
# tokens. `scripts/test-effect-interchange-benchmark-108-policy.sh` proves each of those red.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -euo pipefail
fi

fail() { printf 'effect interchange benchmark 108 policy failure: %s\n' "$1" >&2; return 1; }

validate_benchmark_source() {
    local benchmark=$1
    python3 -I -B - "$benchmark" <<'PY'
import pathlib, re, sys
source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
expected_rates = ["44_100", "48_000", "88_200", "96_000"]
for layout in (1, 2, 3):
    match = re.search(
        rf"static MIGRATION_Q{layout}: \[QualityDescriptor; 4\] = \[(.*?)\];",
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
    local status=$?
    [[ "$status" -eq 0 ]] || return "$status"
}

validate_successor_namespace() {
    if rg -n 'artifact_dir=.*target/issue081|effect-interchange-benchmark-validator[.]py|preflight-effect-interchange-benchmark[.]sh|run-effect-interchange-benchmark[.]sh|test-effect-interchange-benchmark[.]sh' "$@"; then
        fail 'Issue-108 authority reaches the Issue-081 lifecycle or namespace'
        return 1
    else
        local status=$?
        if [[ "$status" -ne 1 ]]; then
            printf 'effect interchange benchmark 108 policy failure: Issue-108 namespace scan failed (rg status %s)\n' "$status" >&2
            return "$status"
        fi
    fi
}

if [[ "${BASH_SOURCE[0]}" != "$0" ]]; then
    return 0
fi

[[ $# -le 1 ]] || { printf 'usage: check-effect-interchange-benchmark-108.sh [root]\n' >&2; exit 2; }
root="$(cd "${1:-.}" && pwd)"
cd "$root"

benchmark=tools/bench/src/effect_interchange.rs
[[ -f "$benchmark" && ! -L "$benchmark" ]] || fail "missing benchmark source: $benchmark"
validate_benchmark_source "$benchmark"
for path in \
    scripts/effect-interchange-benchmark-108-validator.py \
    scripts/check-effect-interchange-benchmark-108.sh \
    scripts/test-effect-interchange-benchmark-108-policy.sh; do
    [[ -f "$path" && ! -L "$path" ]] || fail "missing Issue-108 authority: $path"
done

python3 -I -B - "$benchmark" \
    scripts/effect-interchange-benchmark-108-validator.py \
    scripts/check-effect-interchange-benchmark-108.sh <<'PY'
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

validate_successor_namespace scripts/effect-interchange-benchmark-108-validator.py
if [[ -e target/issue108 ]]; then
    issue108_entries=$(mktemp)
    trap 'rm -f -- "$issue108_entries"' EXIT
    issue108_error=$(mktemp)
    trap 'rm -f -- "$issue108_entries" "$issue108_error"' EXIT
    if find target/issue108 -mindepth 1 -maxdepth 1 -print >"$issue108_entries" 2>"$issue108_error"; then status=0; else status=$?; fi
    if [[ "$status" -ne 0 ]]; then cat "$issue108_entries" "$issue108_error" >&2; fail "Issue-108 artifact traversal failed (status $status)"; fi
    if [[ -s "$issue108_entries" ]]; then
        fail 'Issue-108 persistent artifact appeared before authorization'
    fi
fi
printf 'effect interchange benchmark 108 policy: ok counters=0/0/0/0\n'
