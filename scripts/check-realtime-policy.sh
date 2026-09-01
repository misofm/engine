#!/usr/bin/env bash
# Check the explicitly marked issue-003 realtime call graph and approved unsafe ownership boundary.
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
    printf 'realtime policy failure: %s\n' "$1" >&2
    exit 1
}

realtime_root="crates/engine/src/realtime"
[[ -d "$realtime_root" ]] || fail "missing realtime module"

# Issue #146 adds `crates/lane/src/fpenv.rs`, the canonical floating-point environment
# pinned at every native render entry. On `x86` it carries no `unsafe` of its own -- it reuses the
# already-listed `_mm_getcsr`/`_mm_setcsr` helpers of `softfma.rs` -- and its one unsafe site is the
# AArch64 `mrs`/`msr FPCR` pair, for which the standard library exposes no `core::arch` intrinsic
# (Arm Architecture Reference Manual for A-profile, `FPCR`, Floating-point Control Register).
# Issue #240's exact `boot_transient_budget.rs` fixture owns a `GlobalAlloc` forwarding wrapper so
# it can measure the parser/model-builder high-water mark; it changes no allocation operation and
# is the only non-FFI web-host file admitted here.
# `docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership", carries the full justification.
unsafe_matches="$({
    rg -n 'unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{' \
        crates hosts tools sidecars --glob '*.rs' || true
} | rg -v '^crates/engine/src/realtime/spsc.rs:|^crates/engine/src/realtime/disjoint.rs:|^crates/lane/src/softfma.rs:|^crates/lane/src/fpenv.rs:|^crates/builtins-compiler/tests/allocation_tracker.rs:|^crates/session/tests/allocation_budget.rs:|^crates/soft-clip/tests/allocation.rs:|^crates/transient-shaper/tests/allocation.rs:|^crates/capi/src/ffi.rs:|^crates/capi/tests/resource_lifecycle.rs:|^crates/effect-package/src/ffi.rs:|^crates/effect-package/tests/package_allocation.rs:|^crates/true-peak-limiter/tests/allocation.rs:|^crates/multiband-compressor/tests/no_alloc_render.rs:|^crates/effect-compiler/tests/migration_terminal.rs:|^hosts/host-web/src/ffi.rs:|^hosts/host-web/tests/boot_transient_budget.rs:|^tools/bench-support/src/alloc.rs:|^tools/audit/src/capi.rs:|^tools/native-pcm-runner/src/lib.rs:|^tools/bench/src/protocol.rs:|^tools/wasm-gate-guest/src/lib.rs:|^tools/wasm-console-guest/src/lib.rs:' || true)"
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
    'Vec::|vec!|Box::|String::|\.to_vec\(|\.collect\(|Arc::clone|Rc::clone|drop\(|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep\(|yield_now|spin_loop|std::fs|std::net|std::process|println!|eprintln!|format!|log::|tracing::|async[[:space:]]|\.await|File::|Tcp|Udp|\.expect\(|\.unwrap\(|panic!\(|unreachable!\(|todo!\(|unimplemented!\(' \
    "$scratch_file"; then
    fail "marked realtime code contains an allocation, lock, I/O, log, wait, syscall or panic surface"
fi

printf 'realtime policy: ok (%s marked regions)\n' "$marker_count"
