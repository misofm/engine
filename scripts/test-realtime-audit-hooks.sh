#!/usr/bin/env bash
# Each deliberate forbidden operation must terminate unsuccessfully while render is armed.
set -euo pipefail

binary="${1:-target/release/miso_engine_realtime_audit}"
[[ -x "$binary" ]] || {
    printf 'missing realtime audit binary: %s\n' "$binary" >&2
    exit 1
}

for operation in allocation deallocation lock log file-io network-io syscall; do
    if "$binary" --probe "$operation" >/dev/null 2>&1; then
        printf 'realtime audit hook unexpectedly survived: %s\n' "$operation" >&2
        exit 1
    fi
done

printf 'realtime audit hook mutation tests: ok\n'
