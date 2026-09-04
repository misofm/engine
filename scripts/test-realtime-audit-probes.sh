#!/usr/bin/env bash
# One harness, three subjects: each deliberate forbidden operation must terminate unsuccessfully
# while the named subject's render path is armed.
#
# Folds together what were three near-identical scripts (`test-realtime-audit-hooks.sh`,
# `test-builtins-audit-probes.sh`, `test-builtins-graph-audit-probes.sh`) -- one harness three
# times over `tools/audit`'s `realtime`, `builtins` and `builtins-graph` subcommands. Two of the
# three subjects (`builtins`, `builtins-graph`) previously had no CI caller at all, so their nine
# probes each were not exercised; this wiring closes that gap rather than merely deduplicating the
# shell. All three subjects now run from CI.
set -euo pipefail

usage() {
    printf 'usage: %s <realtime|builtins|builtins-graph> [audit-binary]\n' "$0" >&2
    exit 2
}

subject="${1:-}"
[[ -n "$subject" ]] || usage

# Each subject's audit subcommand and its own probe vocabulary. `realtime` predates
# `feature-detection` and `panic-unwind` as probe operations; `builtins`/`builtins-graph` were
# added later with the full set. Keeping the lists distinct (rather than unioning them) preserves
# exactly what each script asserted before the merge.
case "$subject" in
    realtime)
        command=realtime
        operations=(allocation deallocation lock log file-io network-io syscall)
        ;;
    builtins)
        command=builtins
        operations=(allocation deallocation lock feature-detection log file-io network-io syscall panic-unwind)
        ;;
    builtins-graph)
        command=builtins-graph
        operations=(allocation deallocation lock feature-detection log file-io network-io syscall panic-unwind)
        ;;
    *)
        usage
        ;;
esac

binary="${2:-}"
if [[ -z "$binary" ]]; then
    binary=target/release/audit
    [[ -x "$binary" ]] ||
        cargo build --locked --release -p audit --bin audit >&2
fi
[[ -x "$binary" ]] || {
    printf 'missing audit binary: %s\n' "$binary" >&2
    exit 1
}

for operation in "${operations[@]}"; do
    if "$binary" "$command" --probe "$operation" >/dev/null 2>&1; then
        printf '%s audit probe unexpectedly survived: %s\n' "$subject" "$operation" >&2
        exit 1
    fi
done

printf '%s audit probe mutation tests: ok (%s operations)\n' "$subject" "${#operations[@]}"
