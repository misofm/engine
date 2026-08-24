#!/usr/bin/env bash
# Check the explicitly marked issue-003 realtime call graph and approved unsafe ownership boundary.
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
    printf 'realtime policy failure: %s\n' "$1" >&2
    exit 1
}

realtime_root="crates/miso-engine-core/src/realtime"
[[ -d "$realtime_root" ]] || fail "missing realtime module"

unsafe_matches="$({
    rg -n 'unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{' \
        crates hosts tools --glob '*.rs' || true
} | rg -v '^crates/miso-engine-core/src/realtime/spsc.rs:|^crates/miso-engine-core/src/realtime/disjoint.rs:|^crates/miso-engine-lane/src/softfma.rs:|^crates/miso-engine-builtins-compiler/tests/allocation_tracker.rs:|^crates/miso-engine-session/tests/allocation_budget.rs:|^crates/miso-engine-soft-clip/tests/allocation.rs:|^crates/miso-engine-transient-shaper/tests/allocation.rs:|^crates/miso-engine-capi/src/ffi.rs:|^crates/miso-engine-capi/tests/resource_lifecycle.rs:|^crates/miso-engine-effect-package/src/ffi.rs:|^crates/miso-engine-effect-package/tests/package_allocation.rs:|^crates/miso-engine-true-peak-limiter/tests/allocation.rs:|^crates/miso-engine-multiband-compressor/tests/no_alloc_render.rs:|^crates/miso-engine-effect-compiler/tests/migration_terminal.rs:|^hosts/miso-engine-host-web/src/ffi.rs:|^tools/miso-engine-capi-audit/src/main.rs:|^tools/miso-engine-native-pcm-runner/src/lib.rs:|^tools/miso-engine-realtime-audit/src/main.rs:|^tools/miso-engine-protocol-audit/src/main.rs:|^tools/miso-engine-protocol-bench/src/main.rs:|^tools/miso-engine-effect-contract-bench/src/main.rs:|^tools/miso-engine-graph-audit/src/(main|parametric_eq_main|delay_main|gate_expander_main|compressor_main).rs:|^tools/miso-engine-source-audit/src/main.rs:|^tools/miso-engine-scheduler-audit/src/main.rs:|^tools/miso-engine-scheduler-bench/src/main.rs:|^tools/miso-engine-builtins-audit/src/(main|graph_main).rs:|^tools/miso-engine-builtins-bench/src/main.rs:|^tools/miso-engine-rack-bench/src/main.rs:' || true)"
[[ -z "$unsafe_matches" ]] || {
    printf '%s\n' "$unsafe_matches" >&2
    fail "unsafe code exists outside the issue-approved ownership/audit files"
}

scratch_file="$(mktemp)"
trap 'rm -f -- "$scratch_file"' EXIT

marker_count=0
while IFS= read -r source; do
    begins="$(rg -c 'REALTIME_POLICY_BEGIN' "$source" || true)"
    ends="$(rg -c 'REALTIME_POLICY_END' "$source" || true)"
    [[ "$begins" == "$ends" ]] || fail "$source has unmatched realtime policy markers"
    marker_count=$((marker_count + begins))
    awk '
        /REALTIME_POLICY_BEGIN/ { inside = 1; next }
        /REALTIME_POLICY_END/ { inside = 0; next }
        inside { print FILENAME ":" FNR ":" $0 }
    ' "$source" >>"$scratch_file"
done < <(find "$realtime_root" -name '*.rs' -type f | sort)

[[ "$marker_count" -ge 4 ]] || fail "expected at least four marked realtime regions"

if rg -n \
    'Vec::|vec!|Box::|String::|\.to_vec\(|\.collect\(|Arc::clone|Rc::clone|drop\(|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep\(|yield_now|spin_loop|std::fs|std::net|std::process|println!|eprintln!|format!|log::|tracing::|async[[:space:]]|\.await|File::|Tcp|Udp' \
    "$scratch_file"; then
    fail "marked realtime code contains an allocation, lock, I/O, log, wait, or syscall surface"
fi

if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
    "$realtime_root" --glob '*.rs'; then
    fail "compiled track capacity is forbidden"
fi

printf 'realtime policy: ok (%s marked regions)\n' "$marker_count"
