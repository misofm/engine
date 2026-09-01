#!/usr/bin/env bash
# One harness, three subjects: each deliberate forbidden operation must terminate unsuccessfully
# while the named subject's render path is armed. Formerly three near-identical scripts
# (test-realtime-audit-hooks.sh, test-builtins-audit-probes.sh, test-builtins-graph-audit-probes.sh)
# that differed only in subject name and operation list; one harness means a new probe operation
# gets added once, not three times out of sync.
set -euo pipefail

binary="${1:-}"
if [[ -z "$binary" ]]; then
    binary=target/release/audit
    [[ -x "$binary" ]] ||
        cargo build --locked --release -p audit >&2
fi
[[ -x "$binary" ]] || {
    printf 'missing audit binary: %s\n' "$binary" >&2
    exit 1
}

# realtime predates the feature-detection and panic-unwind probes (#84 phase A retired runtime ISA
# dispatch and #94 added the panic-unwind probe after realtime's operation list was pinned);
# builtins and builtins-graph both carry the full nine-operation set.
realtime_operations="allocation deallocation lock log file-io network-io syscall"
full_operations="allocation deallocation lock feature-detection log file-io network-io syscall panic-unwind"

run_subject() {
    local subject="$1"
    shift
    for operation in "$@"; do
        if "$binary" "$subject" --probe "$operation" >/dev/null 2>&1; then
            printf '%s audit probe unexpectedly survived: %s\n' "$subject" "$operation" >&2
            exit 1
        fi
    done
    printf '%s audit probes: ok\n' "$subject"
}

# shellcheck disable=SC2086
run_subject realtime $realtime_operations
# shellcheck disable=SC2086
run_subject builtins $full_operations
# shellcheck disable=SC2086
run_subject builtins-graph $full_operations

printf 'audit probe mutation tests: ok (realtime 7, builtins 9, builtins-graph 9)\n'
