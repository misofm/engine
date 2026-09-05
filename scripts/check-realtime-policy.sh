#!/usr/bin/env bash
# Check the explicitly marked issue-003 realtime call graph and approved unsafe ownership boundary.
set -euo pipefail

workspace_root="${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GATE_FAILURE_PREFIX='realtime policy failure'
source "$script_directory/lib/gate.sh"
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
unsafe_raw="$(gate_scan_collect 'unsafe source scan' 'unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{' '*.rs' crates hosts tools sidecars)" || exit $?
unsafe_matches="$(gate_filter_exclude 'unsafe source exclusions' '^crates/engine/src/realtime/spsc.rs:|^crates/engine/src/realtime/disjoint.rs:|^crates/lane/src/softfma.rs:|^crates/lane/src/fpenv.rs:|^crates/builtins-compiler/tests/allocation_tracker.rs:|^crates/session/tests/allocation_budget.rs:|^crates/soft-clip/tests/allocation.rs:|^crates/transient-shaper/tests/allocation.rs:|^crates/capi/src/ffi.rs:|^crates/capi/tests/resource_lifecycle.rs:|^crates/effect-package/src/ffi.rs:|^crates/effect-package/tests/package_allocation.rs:|^crates/true-peak-limiter/tests/allocation.rs:|^crates/multiband-compressor/tests/no_alloc_render.rs:|^crates/effect-compiler/tests/migration_terminal.rs:|^hosts/host-web/src/ffi.rs:|^hosts/host-web/tests/boot_transient_budget.rs:|^tools/bench-support/src/alloc.rs:|^tools/audit/src/capi.rs:|^tools/native-pcm-runner/src/lib.rs:|^tools/bench/src/protocol.rs:|^tools/wasm-gate-guest/src/lib.rs:|^tools/wasm-console-guest/src/lib.rs:' "$unsafe_raw")" || exit $?
[[ -z "$unsafe_matches" ]] || {
    printf '%s\n' "$unsafe_matches" >&2
    fail "unsafe code exists outside the issue-approved ownership/audit files"
}

scratch_file="$(mktemp)"
trap 'rm -f -- "$scratch_file"' EXIT

# Issue #371 (RT-16/IO-14): the scan is root-agnostic. Every file that carries a
# REALTIME_POLICY_BEGIN marker is scanned, wherever it sits under `crates hosts tools
# sidecars`, instead of only the files under crates/engine/src/realtime. Markers in a file
# outside that directory are no longer decorative: the unmatched-marker check and the
# forbidden-surface regex both reach them. The directory-existence assertion above stays on
# its own: the issue-003 realtime module is the one directory that must exist whether or not
# anything else is marked.
marker_count=0
marked_file_count=0
if marked_files_raw="$(rg -l 'REALTIME_POLICY_BEGIN' crates hosts tools sidecars --glob '*.rs' 2>&1)"; then marked_files_rc=0; else marked_files_rc=$?; fi
[[ $marked_files_rc == 1 ]] && marked_files_raw=''
[[ $marked_files_rc -le 1 ]] || { printf '%s\n' "$marked_files_raw" >&2; fail "realtime marker discovery failed (rg exit $marked_files_rc)"; }
marked_files="$(gate_sort_lines 'realtime marker discovery' "$marked_files_raw")" || exit $?
while IFS= read -r source; do
    if begins="$(rg -c 'REALTIME_POLICY_BEGIN' "$source" 2>&1)"; then :; else rc=$?; [[ $rc == 1 ]] && begins=0 || { printf '%s\n' "$begins" >&2; fail "BEGIN marker count failed for $source (rg exit $rc)"; }; fi
    if ends="$(rg -c 'REALTIME_POLICY_END' "$source" 2>&1)"; then :; else rc=$?; [[ $rc == 1 ]] && ends=0 || { printf '%s\n' "$ends" >&2; fail "END marker count failed for $source (rg exit $rc)"; }; fi
    [[ "$begins" == "$ends" ]] || fail "$source has unmatched realtime policy markers"
    marker_count=$((marker_count + begins))
    marked_file_count=$((marked_file_count + 1))
    if body="$(awk '
        /REALTIME_POLICY_BEGIN/ { inside = 1; next }
        /REALTIME_POLICY_END/ { inside = 0; next }
        inside { print FILENAME ":" FNR ":" $0 }
    ' "$source" 2>&1)"; then :; else rc=$?; printf '%s\n' "$body" >&2; fail "realtime body extraction failed for $source (awk status $rc)"; fi
    [[ -z "$body" ]] || printf '%s\n' "$body" >>"$scratch_file" || fail "realtime body persistence failed for $source"
done <<<"$marked_files"

# The floors are the tree's own counts when #371 landed the last marker: twelve files and
# forty-two regions. Deleting a marker to silence the gate now fails here -- the file leaves
# the discovered set or a region leaves the marked set -- instead of passing with less
# coverage. Raising a floor is part of the change that adds a marker.
[[ "$marked_file_count" -ge 12 ]] || fail "expected at least twelve marked realtime files"
[[ "$marker_count" -ge 42 ]] || fail "expected at least forty-two marked realtime regions"

gate_scan_forbidden 'marked realtime forbidden-body predicate' \
    'Vec::|vec!|Box::|String::|\.to_vec\(|\.collect\(|Arc::clone|Rc::clone|drop\(|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep\(|yield_now|spin_loop|std::fs|std::net|std::process|println!|eprintln!|format!|log::|tracing::|async[[:space:]]|\.await|File::|Tcp|Udp|\.expect\(|\.unwrap\(|panic!\(|unreachable!\(|todo!\(|unimplemented!\(' '' "$scratch_file" || exit $?

# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh (P12): it scans the whole
# {crates,hosts,tools,sidecars} tree, of which the realtime module is a part, rather than one of
# five copies of the same regex over five different root lists.

printf 'realtime policy: ok (%s marked regions in %s files)\n' "$marker_count" "$marked_file_count"
