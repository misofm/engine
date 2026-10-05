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
unsafe_raw="$(gate_scan_collect 'unsafe source scan' 'unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{' '*.rs' crates hosts tools)" || exit $?
unsafe_matches="$(gate_filter_exclude 'unsafe source exclusions' '^crates/engine/src/realtime/spsc.rs:|^crates/engine/src/realtime/disjoint.rs:|^crates/lane/src/softfma.rs:|^crates/lane/src/fpenv.rs:|^crates/builtins-compiler/tests/allocation_tracker.rs:|^crates/session/tests/allocation_budget.rs:|^crates/soft-clip/tests/allocation.rs:|^crates/transient-shaper/tests/allocation.rs:|^crates/capi/src/ffi.rs:|^crates/capi/tests/resource_lifecycle.rs:|^crates/capi/tests/plan_swap_race.rs:|^crates/true-peak-limiter/tests/allocation.rs:|^crates/multiband-compressor/tests/no_alloc_render.rs:|^hosts/host-web/src/ffi.rs:|^hosts/host-web/tests/boot_transient_budget.rs:|^tools/bench-support/src/alloc.rs:|^tools/audit/src/capi.rs:|^tools/wasm-gate-guest/src/lib.rs:' "$unsafe_raw")" || exit $?
[[ -z "$unsafe_matches" ]] || {
    printf '%s\n' "$unsafe_matches" >&2
    fail "unsafe code exists outside the issue-approved ownership/audit files"
}

scratch_file="$(mktemp)"
trap 'rm -f -- "$scratch_file"' EXIT

# Issue #371 (RT-16/IO-14): the scan is root-agnostic. Every file that carries a
# REALTIME_POLICY_BEGIN marker is scanned, wherever it sits under `crates hosts tools`, instead
# of only the files under crates/engine/src/realtime. Markers in a file
# outside that directory are no longer decorative: the unmatched-marker check and the
# forbidden-surface regex both reach them. The directory-existence assertion above stays on
# its own: the issue-003 realtime module is the one directory that must exist whether or not
# anything else is marked.
marker_count=0
marked_file_count=0
if marked_files_raw="$(rg -l 'REALTIME_POLICY_BEGIN' crates hosts tools --glob '*.rs' 2>&1)"; then marked_files_rc=0; else marked_files_rc=$?; fi
[[ $marked_files_rc == 1 ]] && marked_files_raw=''
[[ $marked_files_rc -le 1 ]] || { printf '%s\n' "$marked_files_raw" >&2; fail "realtime marker discovery failed (rg exit $marked_files_rc)"; }
marked_files="$(gate_sort_lines 'realtime marker discovery' "$marked_files_raw")" || exit $?
if [[ -n "$marked_files" ]]; then
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
fi

# The floors are the merged tree's own counts after #1269 phase 1 (the swap carry and the
# #1071/#1278 effect-restore regions) and #1053 (#1253's builtins-compiler fader and matrix
# drains): twenty-five files and eighty-nine regions (they were twelve and forty-one after #664's
# complete LocalRing removal). Deleting a marker to silence the gate fails here -- the file leaves
# the discovered set or a region leaves the marked set -- instead of passing with less coverage.
# Raising a floor is part of the change that adds a marker.
[[ "$marked_file_count" -ge 25 ]] || fail "expected at least twenty-five marked realtime files"
[[ "$marker_count" -ge 89 ]] || fail "expected at least eighty-nine marked realtime regions"

gate_scan_forbidden 'marked realtime forbidden-body predicate' \
    'Vec::|vec!|Box::|String::|\.to_vec\(|\.collect\(|Arc::clone|Rc::clone|drop\(|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep\(|yield_now|spin_loop|std::fs|std::net|std::process|println!|eprintln!|format!|log::|tracing::|async[[:space:]]|\.await|File::|Tcp|Udp|\.expect\(|\.unwrap\(|panic!\(|unreachable!\(|todo!\(|unimplemented!\(' '' "$scratch_file" || exit $?

# #1253, #1302: a render-thread drain pops at most the records present at block entry. A loop
# that pops until the queue is empty keeps popping whatever a concurrent producer publishes while
# it runs (the C ABI prepares with `Concurrent` delivery) -- data-dependent, unbounded render
# work. What this gate proves, per marked region, reading rustfmt's layout (four-space indents, a
# wrapped loop header's `{` ending its last line; `cargo fmt --all -- --check` is required):
#   - every `try_pop` that sits in a loop (the `fn try_pop` definition never does) has an
#     innermost loop bounded by a count taken from `available_at_entry`: `for _ in 0..<count> {`,
#     `for _ in 0..<expr naming available_at_entry> {`, or `while <count> != 0 {` (or `> 0 {`)
#     whose body's first statement is `<count> -= 1;`. `<count>` is the nearest in-scope
#     `let [mut] <count> = ..;` (comments dropped, the statement joined up to its `;`) naming
#     `available_at_entry`, or a plain one-step alias `let [mut] <count> = <entry count>;`, with no
#     other assignment to it between the binding and the loop, nor inside a `while` body;
#   - every loop enclosing that innermost loop, up to the enclosing `fn`, is a `for .. in` over
#     something other than an open range: a `while` or `loop` around a bounded drain re-drains
#     without bound;
#   - a line with `while`, `loop` or `for .. in` anywhere in its code is a loop opener, and only
#     one that starts the line (after an optional `'label:`) can be bounded, so a one-line
#     `while let Ok(..) = ..try_pop() {` body or a `let x = loop {` is refused;
#   - `iter::from_fn`, `iter::repeat_with` and `iter::successors` are refused outright: they run a
#     pop in a loop without a loop keyword.
# It is not a Rust parser. Recursion, a pop inside a closure handed to some other repeating
# adapter, a `use` that renames an iterator constructor, a count inflated by arithmetic inside its
# own `available_at_entry` statement, and a count mutated through a `&mut` borrow stay outside it.
gate_scan_forbidden 'marked realtime unbounded try_pop drain (bound it with available_at_entry)' \
    '(^|[^[:alnum:]_])iter::(from_fn|repeat_with|successors)([^[:alnum:]_]|$)' '' "$scratch_file" || exit $?
if [[ -n "$marked_files" ]]; then
    mapfile -t marked_list <<<"$marked_files"
    if drain_hits="$(awk -v q="'" '
        function code(s,  i) { i = index(s, "//"); if (i) s = substr(s, 1, i - 1); sub(/[ \t]+$/, "", s); return s }
        function ind(s) { match(s, /^ */); return RLENGTH }
        function trim(s) { sub(/^ +/, "", s); return s }
        function word(x) { return "(^|[^A-Za-z0-9_])" x "([^A-Za-z0-9_]|$)" }
        function label() { return q "[A-Za-z_][A-Za-z0-9_]*: *" }
        function unlabel(s) { s = trim(s); sub("^" label(), "", s); return s }
        # A loop keyword anywhere on the line makes it a loop opener; only an opener that starts
        # its line (after an optional label) can have a bounded header.
        function isloop(s) { return s ~ word("while") || s ~ word("loop") || s ~ word("for [^{}]* in") }
        function isfn(s) { return trim(s) ~ /^(pub(\([a-z: ]+\))? +)?((const|async|unsafe|default|extern( +"[^"]*")?) +)*fn +[A-Za-z_]/ }
        # The scope chain of line `from`: walking back, a line indented less than every line seen
        # so far opens a block enclosing `from`. Fills chain[1..chain_n], innermost first, up to
        # and including the enclosing function line.
        function enclosing(from,  k, m) {
            chain_n = 0; m = ind(c[from])
            for (k = from - 1; k >= 1; k--) {
                if (c[k] ~ /^ *$/ || ind(c[k]) >= m) continue
                m = ind(c[k]); chain[++chain_n] = k
                if (isfn(c[k])) return
            }
        }
        # A loop header: its opener joined up to the first line ending in `{`, whitespace collapsed.
        function header(l,  j, h) {
            h = ""
            for (j = l; j <= n; j++) { h = h " " trim(c[j]); if (c[j] ~ /\{$/) break }
            header_end = j; gsub(/ +/, " ", h); sub(/^ /, "", h); return unlabel(h)
        }
        function assigns(x, s) {
            if (trim(s) ~ ("^let +(mut +)?" x "([ :=]|$)")) return 0
            return s ~ ("(^|[^A-Za-z0-9_.])" x " *([-+*/%&|^]|<<|>>)?=([^=>]|$)")
        }
        # Whether `x` is an entry count at line `before`: its nearest in-scope binding names
        # available_at_entry, or aliases such a binding in one step, and nothing assigns it between.
        function counted(x, before, depth,   k, m, b, j, st) {
            if (depth > 1) return 0
            b = 0; m = ind(c[before])
            for (k = before - 1; k >= 1 && !b; k--) {
                if (c[k] ~ /^ *$/ || ind(c[k]) > m) continue
                if (ind(c[k]) < m) { m = ind(c[k]); if (isfn(c[k])) return 0; continue }
                if (trim(c[k]) ~ ("^let +(mut +)?" x "( *:[^=]*)? *=([^=]|$)")) b = k
            }
            if (!b) return 0
            for (j = b + 1; j < before; j++) if (assigns(x, c[j])) return 0
            st = ""
            for (j = b; j <= n; j++) { st = st " " trim(c[j]); if (c[j] ~ /;$/) break }
            gsub(/ +/, " ", st); sub(/^ /, "", st)
            if (st ~ /available_at_entry/) return 1
            if (st ~ ("^let (mut )?" x "( ?:[^=]*)? = [A-Za-z_][A-Za-z0-9_]*;$")) {
                sub(/^.*= /, "", st); sub(/;$/, "", st); return counted(st, b, depth + 1)
            }
            return 0
        }
        function bounded(l,  h, x, j, first) {
            h = header(l)
            if (h ~ /^for [^{}]* in 0\.\.[^{}]*available_at_entry[^{}]* \{$/) return 1
            if (h ~ /^for [^{}]* in 0\.\.[A-Za-z_][A-Za-z0-9_]* \{$/) {
                x = h; sub(/^.* in 0\.\./, "", x); sub(/ \{$/, "", x); return counted(x, l, 0)
            }
            if (h ~ /^while [A-Za-z_][A-Za-z0-9_]* (!= 0|> 0) \{$/) {
                x = h; sub(/^while /, "", x); sub(/ .*/, "", x)
                if (!counted(x, l, 0)) return 0
                first = 0
                for (j = header_end + 1; j <= n; j++) {
                    if (c[j] ~ /^ *$/) continue
                    if (ind(c[j]) <= ind(c[l])) break
                    if (!first) { first = j; if (trim(c[j]) != x " -= 1;") return 0; continue }
                    if (assigns(x, c[j])) return 0
                }
                return first != 0
            }
            return 0
        }
        function check(  i, k, l, ok) {
            for (i = 1; i <= n; i++) {
                if (c[i] !~ word("try_pop")) continue
                enclosing(i); l = 0; ok = 1
                if (isloop(c[i])) { l = i; ok = bounded(i) }
                for (k = 1; k <= chain_n; k++) {
                    if (!isloop(c[chain[k]])) continue
                    if (!l) { l = chain[k]; ok = bounded(l) }
                    else if (header(chain[k]) !~ /^for [^{}]* in [^{}]* \{$/ || header(chain[k]) ~ /\.\. \{$/) ok = 0
                }
                if (!ok) print FILENAME ":" ln[i] ":" c[i]
            }
        }
        /REALTIME_POLICY_BEGIN/ { inside = 1; n = 0; next }
        /REALTIME_POLICY_END/ { inside = 0; check(); next }
        inside { n++; c[n] = code($0); ln[n] = FNR }
    ' "${marked_list[@]}" 2>&1)"; then :; else rc=$?; printf '%s\n' "$drain_hits" >&2; fail "realtime drain-bound scan failed (awk status $rc)"; fi
    [[ -z "$drain_hits" ]] || {
        printf '%s\n' "$drain_hits" >&2
        fail 'marked realtime unbounded try_pop drain (bound it with available_at_entry)'
    }
fi

# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh (P12): it scans the whole
# {crates,hosts,tools} tree, of which the realtime module is a part, rather than one of five copies
# of the same regex over five different root lists.

# Issue #1047 moved this scan here from capi's `ffi_never_forms_a_whole_plan_reference` test. No
# workflow runs Miri, so it is the only guard of the C ABI's control/render split: the render
# thread holds `&mut PlanState` while any thread may query the same handle, so the production
# code of `crates/capi/src/ffi.rs` never forms a reference to the whole `Plan`. It projects the
# fields it needs with `&raw const`/`&raw mut` (`plan_state`, `plan_error_slot`, `plan_queries`).
# `&*plan.cast::<HandleHeader>()` borrows only the shared header, `&*plan_state(..)` names another
# item, and `&(*plan).field` borrows one field, so a projection after the dereference is not a
# hit. Comment lines and the test module are not production code. The render entry point must be
# in the scanned region, so a renamed file or a moved test-module marker cannot pass as clean.
capi_ffi='crates/capi/src/ffi.rs'
[[ -f "$capi_ffi" ]] || fail "missing $capi_ffi"
if capi_production="$(awk '
    previous == "#[cfg(test)]" && $0 == "mod tests {" { exit }
    { previous = $0 }
    /^[[:space:]]*\/\// { next }
    { print FILENAME ":" FNR ":" $0 }
' "$capi_ffi" 2>&1)"; then :; else rc=$?; printf '%s\n' "$capi_production" >&2; fail "capi production-region extraction failed (awk status $rc)"; fi
[[ "$capi_production" == *miso_engine_v1_render_f32_planar* ]] ||
    fail "$capi_ffi has no production render entry point to scan"
whole_plan="$(gate_scan_text_collect 'whole-plan reference' '&(mut[[:space:]]+)?(\*plan|\(\*plan\))([^[:alnum:]_.]|$)' "$capi_production")" || exit $?
[[ -z "$whole_plan" ]] || {
    printf '%s\n' "$whole_plan" >&2
    fail "the C ABI forms a reference to a whole Plan; project the field with &raw const or &raw mut"
}

printf 'realtime policy: ok (%s marked regions in %s files)\n' "$marker_count" "$marked_file_count"
