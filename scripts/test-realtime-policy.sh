#!/usr/bin/env bash
# Mutation tests proving the marked realtime policy and unsafe allowlist are enforced.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-realtime-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_fixture() {
    local root="$1"
    mkdir -p "$root/crates/engine/src/realtime" \
        "$root/crates/lane/src" \
        "$root/crates/capi/src" \
        "$root/crates/capi/tests" \
        "$root/crates/effect-contract/src" \
        "$root/crates/graph/src" \
        "$root/crates/rack/src" \
        "$root/crates/builtins/src" \
        "$root/crates/builtins-compiler/src" \
        "$root/crates/session/tests" \
        "$root/hosts/host-web/src" \
        "$root/hosts/host-web/tests" \
        "$root/tools/bench-support/src" \
        "$root/tools/audit/src" \
        "$root/tools/bench/src"
    # The marked file set mirrors the real tree after #371 (RT-16/IO-14), #664's complete
    # LocalRing removal and #1253's builtins-compiler drains: thirteen files and forty-three
    # regions across crates/ and hosts/, padded at the end of this function to the gate's current
    # floors, so the floors and the discovery walk are exercised against the counts the gate sees
    # on main. Column-zero markers and indented markers (as in the real `impl`-block regions) both
    # appear.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn render() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/engine/src/realtime/buffer.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for Allowed {}' \
        'struct Allowed;' \
        '// REALTIME_POLICY_BEGIN' \
        'fn push() {}' \
        'impl<T> Consumer<T> {' \
        '    pub fn try_pop(&mut self) -> Result<T, QueueEmpty> {' \
        '        self.take_front()' \
        '    }' \
        '}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/engine/src/realtime/spsc.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Sync for DisjointArena {}' \
        'struct DisjointArena;' \
        '// REALTIME_POLICY_BEGIN' \
        'fn queue() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/engine/src/realtime/disjoint.rs"
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn buffer() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/engine/src/realtime/observe.rs"
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn exchange() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/engine/src/realtime/plan.rs"
    # `enter_block` is the real single pop. In `retire_one` and `retire_lanes` a pop follows a closed
    # `for` statement, in a bare block or an `if let`: that statement ends there, so the pop is in no
    # loop (#1302 attempt 3).
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn exchange_plane() {}' \
        'impl Exchange {' \
        '    fn enter_block(&mut self) {' \
        '        if self.pending.is_none()' \
        '            && let Ok(candidate) = self.publication.try_pop()' \
        '        {' \
        '            self.pending = Some(candidate);' \
        '        }' \
        '    }' \
        '    fn retire_one(&mut self) {' \
        '        for lane in 0..self.lanes {' \
        '            self.clear(lane);' \
        '        }' \
        '        {' \
        '            let Ok(retired) = self.retirement.try_pop() else {' \
        '                return;' \
        '            };' \
        '            self.retire(retired);' \
        '        }' \
        '    }' \
        '    fn retire_lanes(&mut self) {' \
        '        for lane in 0..self.lanes {' \
        '            self.clear(lane);' \
        '        }' \
        '        if let Ok(retired) = self.retirement.try_pop() {' \
        '            self.retire(retired);' \
        '        }' \
        '    }' \
        '}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/engine/src/realtime/plan_exchange.rs"
    # Indented markers, the shape the real region in crates/graph/src/lib.rs carries.
    printf '%s\n' \
        'struct GraphPlan;' \
        '' \
        'impl GraphPlan {' \
        '    // REALTIME_POLICY_BEGIN' \
        '    fn lowered() {}' \
        '    // REALTIME_POLICY_END' \
        '}' \
        >"$root/crates/graph/src/lib.rs"
    # Eight regions, mirroring the real crates/graph/src/runtime.rs after #371; the
    # `execute_op` region keeps its indented markers, as in the real `impl Runtime` block.
    printf '%s\n' \
        'struct Runtime;' \
        '' \
        'impl Runtime {' \
        '    // REALTIME_POLICY_BEGIN' \
        '    fn execute_op() {}' \
        '    // REALTIME_POLICY_END' \
        '}' \
        '' \
        '// REALTIME_POLICY_BEGIN' \
        'fn reduce_plane() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn compensation_delay_process() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn track_delay_process() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn publish_observations() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn buffer_mut() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn arena_members_fold_plane() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn observe() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/graph/src/runtime.rs"
    # Two regions, mirroring the real crates/effect-contract/src/live.rs after #371: the
    # pre-existing ObservationLane region and the new impl EffectControlLane region.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'impl ObservationLane {' \
        '    fn accumulate() {}' \
        '}' \
        '// REALTIME_POLICY_END' \
        '' \
        '// REALTIME_POLICY_BEGIN' \
        'impl EffectControlLane {' \
        '    fn stage() {}' \
        '    pub fn stage_records(&mut self) -> usize {' \
        '        let available = self' \
        '            .control' \
        '            .as_ref()' \
        '            .map_or(0, Consumer::available_at_entry);' \
        '        let mut staged = 0_usize;' \
        '        let mut remaining = available;' \
        '        // A comment line between the alias and the loop is not code.' \
        '        while remaining != 0 {' \
        '            remaining -= 1;' \
        '            let Some(Ok(record)) = self.control.as_mut().map(Consumer::try_pop) else {' \
        '                break;' \
        '            };' \
        '            staged += admit(record);' \
        '        }' \
        '        staged' \
        '    }' \
        '}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/effect-contract/src/live.rs"
    # Eighteen regions, mirroring the real crates/rack/src/lib.rs after #371: the chain's run,
    # every gather*/scatter* body, accumulate_aux and the BankStage process bodies.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn gather_lane() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn gather_lane_left() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn scatter_lane() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn tile_gather() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn tile_scatter() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn effect_bank_process_mono() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn effect_bank_process() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn live_control_process_mono() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn live_control_process() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn process_inner() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn run() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn gather_mono() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn gather_mono_tiled() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn accumulate_aux() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn gather() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn gather_tiled() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn scatter() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn scatter_tiled() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/rack/src/lib.rs"
    # Five regions, mirroring the real crates/builtins/src/lib.rs after #371. The first also holds
    # bounded drains inside finite outer loops (#1302): an `array::from_fn` closure, with a comment
    # naming the count between its binding and its loop; `0..2` over an indexed receiver;
    # `0..self.lanes` with a count capped by `.min(..)`, read by a comparison and a
    # `debug_assert!` before an attributed loop; `0..LANES`; a `.zip(..)`; and a slice parameter.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn input_process() {}' \
        'fn input_lane_states() -> [[u8; 4]; 2] {' \
        '    let gains: [u8; 4] = core::array::from_fn(|lane| lane as u8);' \
        '    let trims: [u8; 4] = std::array::from_fn(|lane| lane as u8);' \
        '    [gains, trims]' \
        '}' \
        'fn drain_lane_pairs(control: &mut Consumer<Record>) -> [usize; 2] {' \
        '    core::array::from_fn(|lane| {' \
        '        let available = control.available_at_entry();' \
        '        // `available` is fixed at entry: a push during this drain waits for the next block.' \
        '        let mut applied = 0;' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            applied += apply(lane, record);' \
        '        }' \
        '        applied' \
        '    })' \
        '}' \
        'fn drain_two_lanes(controls: &mut [Consumer<Record>; 2]) {' \
        '    for lane in 0..2 {' \
        '        let available = controls[lane].available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = controls[lane].try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(lane, record);' \
        '        }' \
        '    }' \
        '}' \
        'impl LaneBank {' \
        '    fn drain_lane_fields(&mut self) {' \
        '        for lane in 0..self.lanes {' \
        '            let control = &mut self.controls[lane];' \
        '            let available = control.available_at_entry().min(MAX_RECORDS_PER_BLOCK);' \
        '            if available == 0 {' \
        '                continue;' \
        '            }' \
        '            debug_assert!(available <= MAX_RECORDS_PER_BLOCK);' \
        '            #[allow(clippy::needless_range_loop)]' \
        '            for _ in 0..available {' \
        '                let Ok(record) = control.try_pop() else {' \
        '                    break;' \
        '                };' \
        '                apply(lane, record);' \
        '            }' \
        '        }' \
        '    }' \
        '}' \
        'fn drain_lane_constant(controls: &mut [Consumer<Record>; LANES]) {' \
        '    for lane in 0..LANES {' \
        '        let available = controls[lane].available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = controls[lane].try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(lane, record);' \
        '        }' \
        '    }' \
        '}' \
        'fn drain_zipped_lanes(controls: &mut [Consumer<Record>], gains: &[f32]) {' \
        '    for (control, gain) in controls.iter_mut().zip(gains) {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply_scaled(*gain, record);' \
        '        }' \
        '    }' \
        '}' \
        'fn drain_lane_slice(controls: &mut [Consumer<Record>]) {' \
        '    for control in controls {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(0, record);' \
        '        }' \
        '    }' \
        '}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn input_process_mono() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn fader_process() {}' \
        'fn fader_process_plane() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn matrix_process() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn meter_observe() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/builtins/src/lib.rs"
    # One region, mirroring the real hosts/host-web/src/lib.rs `render_next` after #371.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn render_next() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/hosts/host-web/src/lib.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn read_mxcsr() {}' \
        >"$root/crates/lane/src/softfma.rs"
    # Issue #146: the AArch64 FPCR pair of the canonical render-entry environment.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn write_fpcr() {}' \
        >"$root/crates/lane/src/fpenv.rs"
    # #1047: the C ABI's production code projects plan fields and never borrows the whole plan.
    # Each allowed shape is here: a header or helper after the dereference, a field projection, a
    # comment, and the test module, which is not production code.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn capi_boundary() {}' \
        'pub unsafe extern "C" fn miso_engine_v1_render_f32_planar(plan: *mut Plan) -> u32 {' \
        '    let _ = unsafe { &*plan.cast::<HandleHeader>() };' \
        '    let _ = unsafe { &mut *plan_state(plan) };' \
        '    let _ = unsafe { &(*plan).queries };' \
        '    // Never `&*plan` or `&mut *plan`: a query may run while render holds the state.' \
        '    0' \
        '}' \
        '#[cfg(test)]' \
        'mod tests {' \
        '    fn fixture(plan: *mut Plan) { let _ = unsafe { &*plan }; }' \
        '}' \
        >"$root/crates/capi/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for LifecycleAllocator {}' \
        'struct LifecycleAllocator;' \
        >"$root/crates/capi/tests/resource_lifecycle.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for CountingAllocator {}' \
        'struct CountingAllocator;' \
        >"$root/crates/session/tests/allocation_budget.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn web_boundary() {}' \
        >"$root/hosts/host-web/src/ffi.rs"
    # Issue #240: the exact peak fixture's forwarding allocator is a measured audit boundary.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for PeakAllocator {}' \
        'struct PeakAllocator;' \
        >"$root/hosts/host-web/tests/boot_transient_budget.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for CapiAudit {}' \
        'struct CapiAudit;' \
        >"$root/tools/audit/src/capi.rs"
    # #104 phase B: the fourteen audited `GlobalAlloc` copies became one. `bench-support/src/alloc.rs`
    # is the only file under `tools/` that owns the allocator wrapper, and eleven tool paths left
    # this list because they no longer contain `unsafe` at all.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for AuditedAllocator {}' \
        'struct AuditedAllocator;' \
        >"$root/tools/bench-support/src/alloc.rs"
    printf '%s\n' \
        'fn measure() {}' \
        >"$root/tools/bench/src/console.rs"
    # Two regions, mirroring the real crates/builtins-compiler/src/lib.rs after #1253: the
    # bounded fader and matrix drains.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn drain_fader_controls() {}' \
        'fn drain_fader_records(controls: &mut [Option<Consumer<Record>>]) {' \
        '    for (lane, control) in controls.iter_mut().enumerate() {' \
        '        let Some(control) = control.as_mut() else {' \
        '            continue;' \
        '        };' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(lane, record);' \
        '        }' \
        '    }' \
        '}' \
        '// REALTIME_POLICY_END' \
        '' \
        '// REALTIME_POLICY_BEGIN' \
        'fn drain_matrix_controls() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/builtins-compiler/src/lib.rs"
    # The floors rose with #1269 phase 1 (the swap carry and effect-restore regions) and #1053
    # (#1253's drains) to the merged tree's 25 files and 89 regions, and with decision-15 stream B
    # batch 1 (#1314 Amendment 1) to 26 files and 93 regions. The thirteen files and forty-three
    # regions above keep the shapes the mutations below exercise; these thirteen files hold the
    # other fifty regions (one file of fourteen, twelve of three), so the fixture again sits
    # exactly on both floors.
    mkdir -p "$root/crates/floor/src"
    local pad region regions
    for pad in $(seq 1 13); do
        regions=3
        [[ "$pad" -eq 1 ]] && regions=14
        for region in $(seq 1 "$regions"); do
            printf '%s\n' \
                '// REALTIME_POLICY_BEGIN' \
                "fn floor_${pad}_${region}() {}" \
                '// REALTIME_POLICY_END'
        done >"$root/crates/floor/src/pad_${pad}.rs"
    done
}

expect_failure() {
    local name="$1"
    local expected_class="$2"
    local mutation="$3"
    local root="$scratch_root/$name"
    local output
    create_fixture "$root"
    eval "$mutation"
    if output="$(bash "$policy_script" "$root" 2>&1)"; then
        printf 'realtime policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    # The failure must be the class this mutation exists to catch, not merely non-zero:
    # a gate that reds on an unrelated assertion is not the gate the row claims.
    if ! printf '%s\n' "$output" | rg -qF -- "$expected_class"; then
        printf 'realtime policy mutation failed with the wrong class: %s\n%s\n' "$name" "$output" >&2
        exit 1
    fi
}

# Drop the first marked region of a fixture file, leaving every other marker matched, so the
# per-file check passes and only the region floor can red.
drop_first_marked_region() {
    local file="$1"
    awk '
        !started && /REALTIME_POLICY_BEGIN/ { started = 1; dropping = 1; next }
        dropping && /REALTIME_POLICY_END/ { dropping = 0; next }
        dropping { next }
        { print }
    ' "$file" >"$file.tmp"
    mv -- "$file.tmp" "$file"
}

alloc_class='marked realtime forbidden-body predicate'
whole_plan_class='the C ABI forms a reference to a whole Plan'
unsafe_class='unsafe code exists outside the issue-approved ownership/audit files'

valid="$scratch_root/valid"
create_fixture "$valid"
bash "$policy_script" "$valid" >/dev/null
(cd "$scratch_root" && bash "$policy_script" valid >/dev/null)

empty_bodies="$scratch_root/empty-bodies"
create_fixture "$empty_bodies"
sed -i -E '/^[[:space:]]*fn [a-z_]+\(\) \{\}[[:space:]]*$/d' \
    "$empty_bodies/crates/engine/src/realtime/"*.rs \
    "$empty_bodies/crates/graph/src/"*.rs \
    "$empty_bodies/crates/effect-contract/src/live.rs" \
    "$empty_bodies/crates/rack/src/lib.rs" \
    "$empty_bodies/crates/builtins/src/lib.rs" \
    "$empty_bodies/crates/builtins-compiler/src/lib.rs" \
    "$empty_bodies/hosts/host-web/src/lib.rs"
bash "$policy_script" "$empty_bodies" >/dev/null
# A region left open by out-of-order markers ends with its file: the unmarked lines that open the
# next file (here an unbounded test-only drain at the top of each marked file without inner
# attributes) are not read as marked code (#1302 attempt 3).
open_tail_valid="$scratch_root/open-tail-valid"
create_fixture "$open_tail_valid"
while IFS= read -r marked; do
    [[ "$(head -c 2 -- "$marked")" == '#!' ]] && continue
    { printf '%s\n' \
        'fn oracle_drain(control: &mut Consumer<Record>) {' \
        '    while let Ok(record) = control.try_pop() {' \
        '        apply(record);' \
        '    }' \
        '}'
      cat -- "$marked"; } >"$marked.tmp"
    mv -- "$marked.tmp" "$marked"
done < <(rg -l REALTIME_POLICY_BEGIN "$open_tail_valid" --glob '*.rs')
printf '%s\n' '// REALTIME_POLICY_END' '// REALTIME_POLICY_BEGIN' 'fn render_tail() {}' >>"$open_tail_valid/crates/floor/src/pad_1.rs"
bash "$policy_script" "$open_tail_valid" >/dev/null

# The forbidden-surface rules, on the file the gate has always scanned.
expect_failure allocation "$alloc_class" \
    'sed -i "s/fn render() {}/fn render() { let _ = Vec::new(); }/" "$root/crates/engine/src/realtime/buffer.rs"'
expect_failure lock "$alloc_class" \
    'sed -i "s/fn queue() {}/fn queue() { let _ = Mutex::new(0); }/" "$root/crates/engine/src/realtime/disjoint.rs"'
expect_failure log "$alloc_class" \
    'sed -i "s/fn buffer() {}/fn buffer() { println!(\"bad\"); }/" "$root/crates/engine/src/realtime/observe.rs"'
# #84 phase B (F12): a panic path is a realtime violation like an allocation is. `LocalRing`'s
# `.take().expect("prepared local ring slot")` was the only hit inside a marked region; it is gone,
# and the regex now keeps it gone.
expect_failure panic-path-expect "$alloc_class" \
    'sed -i "s/fn exchange() {}/fn exchange() { None::<u8>.expect(\"x\"); }/" "$root/crates/engine/src/realtime/plan.rs"'
expect_failure panic-path-macro "$alloc_class" \
    'sed -i "s/fn exchange() {}/fn exchange() { unreachable!(); }/" "$root/crates/engine/src/realtime/plan.rs"'
expect_failure unsafe-scope "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >>"$root/crates/engine/src/realtime/buffer.rs"'
expect_failure unsafe-outside-exact-allowlist "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/bench/src/other.rs"'
expect_failure unsafe-outside-capi-audit-main "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/audit/src/other.rs"'
# #1033 deleted `tools/native-pcm-runner`; its unsafe exemption went with it, so unsafe code
# re-appearing at its old library path is rejected like any other unlisted file.
expect_failure unsafe-in-deleted-native-pcm-runner-lib "$unsafe_class" \
    'mkdir -p "$root/tools/native-pcm-runner/src"; printf "%s\n" "unsafe fn bad() {}" >"$root/tools/native-pcm-runner/src/lib.rs"'
# #1075 deleted `tools/bench/src/protocol.rs` with the protocol benchmark; its unsafe exemption went
# with it, so unsafe code re-appearing at that path is rejected like any other unlisted file.
expect_failure unsafe-in-deleted-bench-protocol "$unsafe_class" \
    'printf "%s\n" "unsafe fn follow() {}" >"$root/tools/bench/src/protocol.rs"'
# #84 phase A deleted `crates/engine/src/arch/`; its unsafe exemption went with it, so
# unsafe code re-appearing under that path is now rejected like any other unlisted file.
expect_failure unsafe-in-deleted-core-arch "$unsafe_class" \
    'mkdir -p "$root/crates/engine/src/arch"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/engine/src/arch/x86.rs"'
expect_failure unsafe-outside-rack-benchmark-main "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/bench/src/other.rs"'
expect_failure unsafe-outside-capi-ffi "$unsafe_class" \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/crates/capi/src/lib.rs"'
expect_failure unsafe-in-second-capi-ffi-path "$unsafe_class" \
    'mkdir -p "$root/crates/capi/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/capi/src/ffi/other.rs"'
expect_failure unsafe-outside-capi-lifecycle-audit "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/capi/tests/other.rs"'
expect_failure unsafe-outside-session-allocation-budget "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/session/tests/other.rs"'
expect_failure unsafe-outside-web-ffi "$unsafe_class" \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/hosts/host-web/src/lib.rs"'
expect_failure unsafe-in-second-web-ffi-path "$unsafe_class" \
    'mkdir -p "$root/hosts/host-web/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/hosts/host-web/src/ffi/other.rs"'
expect_failure unsafe-outside-web-peak-audit "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/hosts/host-web/tests/other.rs"'
expect_failure unsafe-outside-disjoint-arena "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/engine/src/realtime/disjoint_extra.rs"'
expect_failure unsafe-outside-lane-softfma "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/lane/src/kernels.rs"'
expect_failure unsafe-outside-lane-fpenv "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/lane/src/fpenv_extra.rs"'

# #371 (RT-16/IO-14): the walk is root-agnostic. A marked region outside
# crates/engine/src/realtime is now scanned, in the marker shapes the real files carry.
expect_failure marked-outside-realtime-root "$alloc_class" \
    'sed -i "s/fn render_next() {}/fn render_next() { let _ = vec![0u8; 1]; }/" "$root/hosts/host-web/src/lib.rs"'
# RT-16's verification gate, in the harness: the per-block render body's marked region.
expect_failure marked-runtime-execute-op "$alloc_class" \
    'sed -i "s/fn execute_op() {}/fn execute_op() { let _ = vec![0u8; 1]; }/" "$root/crates/graph/src/runtime.rs"'
# IO-14's verification gate, in the harness: the live parameter-application body's marked region.
expect_failure marked-effect-control-lane-stage "$alloc_class" \
    'sed -i "s/fn stage() {}/fn stage() { let _ = vec![0u8; 1]; }/" "$root/crates/effect-contract/src/live.rs"'
# The discovery set reaches every root, not just crates/ and hosts/: a newly marked tools/ file
# with a violation is found and red.
expect_failure marked-tools-root-scanned "$alloc_class" \
    'printf "%s\n" "// REALTIME_POLICY_BEGIN" "fn tool() { let _ = vec![0u8; 1]; }" "// REALTIME_POLICY_END" >"$root/tools/audit/src/marker_probe.rs"'
# #1253: an unbounded `while let Ok(..) = ..try_pop()` drain inside a marked region is refused
# with a message naming the bounded form.
expect_failure marked-unbounded-try-pop-drain 'bound it with available_at_entry' \
    'sed -i "s/fn drain_fader_controls() {}/fn drain_fader_controls() { while let Ok(record) = control.try_pop() { apply(record); } }/" "$root/crates/builtins-compiler/src/lib.rs"'
# #1302: the drain rule is structural. Each mutation below is an unbounded drain, or an escape
# from the rule, that a one-line spelling match or a region-level `available_at_entry` check
# passes. Replace one exact fixture line with the given lines; a missing anchor fails here so a
# drifted fixture cannot pass a case vacuously.
replace_line() {
    local file="$1" anchor="$2" text
    shift 2
    text="$(printf '%s\n' "$@")"
    # The text goes through the environment, which awk reads verbatim (`-v` would expand a `\`).
    REPLACE_TEXT="$text" awk -v anchor="$anchor" '
        $0 == anchor { print ENVIRON["REPLACE_TEXT"]; found = 1; next }
        { print }
        END { if (!found) exit 3 }
    ' "$file" >"$file.tmp" || { printf 'fixture anchor missing in %s: %s\n' "$file" "$anchor" >&2; exit 1; }
    mv -- "$file.tmp" "$file"
}
drain_class='bound it with available_at_entry'
builtins_compiler='crates/builtins-compiler/src/lib.rs'
mutate_wrapped_while_let() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl MatrixBank {' \
        '    fn drain_matrix_controls(&mut self) {' \
        '        while let Ok(record) = self' \
        '            .some_long_receiver_name' \
        '            .control_consumer_for_this_lane' \
        '            .try_pop()' \
        '        {' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
mutate_let_else_loop() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    loop {' \
        '        let Ok(record) = control.try_pop() else { break };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_path_pop() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    while let Ok(record) = Consumer::try_pop(control) {' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_constant_bound() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    for _ in 0..64 {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_bound_not_at_entry() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.capacity();' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# The fader region already holds the bounded `drain_fader_records`.
mutate_second_loop_in_bounded_region() {
    replace_line "$root/$builtins_compiler" 'fn drain_fader_controls() {}' \
        'fn drain_fader_controls(control: &mut Consumer<Record>) {' \
        '    loop {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_loop_inside_bounded_for() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.available_at_entry();' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '        loop {' \
        '            let Ok(extra) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(extra);' \
        '        }' \
        '    }' \
        '}'
}
mutate_from_fn() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    core::iter::from_fn(|| control.try_pop().ok()).for_each(apply);' \
        '}'
}
mutate_repeat_with() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    core::iter::repeat_with(|| control.try_pop())' \
        '        .map_while(Result::ok)' \
        '        .for_each(apply);' \
        '}'
}
# Beyond the spec's eight: the shapes of an entry count that is not one when the loop runs.
mutate_bounded_drain_inside_outer_loop() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    loop {' \
        '        let available = control.available_at_entry();' \
        '        if available == 0 {' \
        '            break;' \
        '        }' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# The parameter is not the earlier function's entry count, though that binding precedes it in
# the same region.
mutate_count_from_another_function() {
    local file="$root/$builtins_compiler"
    awk '
        /REALTIME_POLICY_END/ && !done {
            print "fn drain_fader_tail(control: &mut Consumer<Record>, available: usize) {"
            print "    for _ in 0..available {"
            print "        let Ok(record) = control.try_pop() else {"
            print "            break;"
            print "        };"
            print "        apply(record);"
            print "    }"
            print "}"
            done = 1
        }
        { print }
    ' "$file" >"$file.tmp"
    mv -- "$file.tmp" "$file"
}
mutate_shadowed_count() {
    replace_line "$root/$builtins_compiler" '        for _ in 0..available {' \
        '        let available = usize::MAX;' \
        '        for _ in 0..available {'
}
mutate_reassigned_count() {
    replace_line "$root/$builtins_compiler" '        let available = control.available_at_entry();' \
        '        let mut available = control.available_at_entry();' \
        '        available = usize::MAX;'
}
mutate_while_without_decrement() {
    sed -i '/^            remaining -= 1;$/d' "$root/crates/effect-contract/src/live.rs"
}
mutate_while_decrement_not_first() {
    replace_line "$root/crates/effect-contract/src/live.rs" '            remaining -= 1;' \
        '            if staged == 0 {' \
        '                remaining -= 1;' \
        '            }'
}
mutate_labelled_loop() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        "    'drain: loop {" \
        '        let Ok(record) = control.try_pop() else {' \
        "            break 'drain;" \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_loop_expression() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) -> usize {' \
        '    let mut applied = 0;' \
        '    let last = loop {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break applied;' \
        '        };' \
        '        applied += apply(record);' \
        '    };' \
        '    last' \
        '}'
}
# #1302 attempt 2: rustfmt opens the block of a wrapped header on a line of its own or on the
# header's last line. Each shape below hides its loop from a walk that takes the first
# lower-indented line as the block's opener.
mutate_wrapped_while_body() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl LaneDrain {' \
        '    fn drain_matrix_controls(&mut self) {' \
        '        while self' \
        '            .control_lane_consumer_for_this_strip' \
        '            .has_pending_records_for_the_current_block()' \
        '            && self.enabled' \
        '        {' \
        '            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {' \
        '                break;' \
        '            };' \
        '            self.apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# Its first line, `for _ in 0..available`, is bounded; the whole wrapped header adds the ring's
# capacity to the entry count.
mutate_wrapped_for_bound() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl LaneDrain {' \
        '    fn drain_matrix_controls(&mut self) {' \
        '        let available = self' \
        '            .control_lane_consumer_for_this_strip' \
        '            .available_at_entry();' \
        '        for _ in 0..available' \
        '            + self' \
        '                .control_lane_consumer_for_this_strip' \
        '                .capacity_of_the_ring_in_records_total()' \
        '        {' \
        '            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {' \
        '                break;' \
        '            };' \
        '            self.apply(record);' \
        '        }' \
        '    }' \
        '}'
}
mutate_wrapped_outer_while() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl LaneDrain {' \
        '    fn drain_matrix_controls(&mut self) {' \
        '        while self' \
        '            .control_lane_consumer_for_this_strip' \
        '            .has_pending_records_for_the_current_block()' \
        '            && self.enabled' \
        '        {' \
        '            let available = self' \
        '                .control_lane_consumer_for_this_strip' \
        '                .available_at_entry();' \
        '            for _ in 0..available {' \
        '                let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {' \
        '                    break;' \
        '                };' \
        '                self.apply(record);' \
        '            }' \
        '        }' \
        '    }' \
        '}'
}
mutate_paren_led_header() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl LaneDrain {' \
        '    fn drain_matrix_controls(&mut self) {' \
        '        for _ in 0..usize::from(' \
        '            self.control_lane_consumer_for_this_strip' \
        '                .capacity_of_the_ring_in_records(),' \
        '        ) {' \
        '            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {' \
        '                break;' \
        '            };' \
        '            self.apply(record);' \
        '        }' \
        '    }' \
        '}'
}
mutate_continuation_led_header() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl LaneDrain {' \
        '    fn drain_matrix_controls(&mut self) {' \
        '        for _ in [' \
        '            self.first_lane_consumer_count,' \
        '            self.second_lane_consumer_count,' \
        '            self.third_lane_consumer_count,' \
        '        ]' \
        '        .into_iter()' \
        '        .take(64)' \
        '        {' \
        '            let Ok(record) = self.control_lane_consumer_for_this_strip.try_pop() else {' \
        '                break;' \
        '            };' \
        '            self.apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# A plain `use core::iter::from_fn;` outside the region leaves only the bare call inside it.
mutate_bare_from_fn() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    from_fn(|| control.try_pop().ok()).for_each(apply);' \
        '}'
}
# `available_at_entry` inside a longer identifier is not an entry count, in a binding or a header.
mutate_entry_name_in_binding() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let cap = control.not_available_at_entry_cap();' \
        '    for _ in 0..cap {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_entry_name_in_header() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    for _ in 0..control.not_available_at_entry_cap() {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A closed `{}` body ends the header: the next line's `available_at_entry` does not bound the
# 64 pops of this one.
mutate_header_past_closed_body() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    for _ in 0..[(); 64].map(|()| control.try_pop()).len() {}' \
        '    if control.available_at_entry() != 0 {' \
        '        apply(control.peek());' \
        '    }' \
        '}'
}
# `core::array::from_fn` stays allowed, but it runs its closure a constant number of times: a pop
# inside it is a constant-bound drain.
mutate_array_from_fn_pop() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let records: [Result<Record, Empty>; 4] = core::array::from_fn(|_| control.try_pop());' \
        '    records.into_iter().flatten().for_each(apply);' \
        '}'
}
# A `let` pattern rebinding the count after its entry binding is the nearest binding.
mutate_pattern_rebound_count() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, limit: Option<usize>) {' \
        '    let available = control.available_at_entry();' \
        '    let Some(available) = limit else {' \
        '        return;' \
        '    };' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A parenthesised open range is still open: the outer `for` re-drains without bound.
mutate_paren_open_range_outer() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    for _ in (0..) {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# #1302 attempt 3: the drain rule reads tokens, not lines. A `match` closed inside a loop
# header and continued by `.method()` (J2-1) is still that header.
mutate_while_match_body() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'impl LaneDrain {' \
        '    fn drain_matrix_controls(&mut self, control: &mut Consumer<Record>) {' \
        '        while match self.lane_selector_for_this_block {' \
        '            Lane::Left => &self.left_consumer,' \
        '            Lane::Right => &self.right_consumer,' \
        '        }' \
        '        .has_records()' \
        '        {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            self.apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# A `{` after a closed block opens the loop body when the block was part of the header.
mutate_while_match_brace() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, lane: Lane) {' \
        '    while match lane {' \
        '        Lane::Left => control.left_ready(),' \
        '        Lane::Right => control.right_ready(),' \
        '    } {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# After a closed block, `in`, `else` and `as` continue the header: a struct pattern, an
# `if`/`else` condition and a cast.
mutate_struct_pattern_for() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, lanes: &[Lane]) {' \
        '    for Lane { left, right } in lanes.iter().cycle() {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record, left, right);' \
        '    }' \
        '}'
}
mutate_if_else_condition() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    while if control.left_first() {' \
        '        control.left_ready()' \
        '    } else {' \
        '        control.right_ready()' \
        '    } {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_as_cast_condition() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, lane: Lane) {' \
        '    while match lane {' \
        '        Lane::Left => 0_u8,' \
        '        Lane::Right => 1_u8,' \
        '    } as usize' \
        '        != control.cursor()' \
        '    {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A `{` after an operator opens a block operand of the header, not the loop body.
mutate_block_operand_condition() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    while control.ready() && {' \
        '        control.refresh();' \
        '        control.more()' \
        '    } {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# Braces and `;` inside literals and comments are not code.
mutate_brace_in_string_header() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    while control.name() != "};{" {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_brace_in_char_header() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        "    while control.tag() != '}' {" \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_brace_in_raw_string_header() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    while control.name() != r#"}";{"# {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
mutate_brace_in_block_comment() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    while control.more() {' \
        '        /* } closes nothing: a comment */' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# Between its binding and its loop the count is named nowhere else: a `for` pattern rebinding
# it is not an entry count (J2-4).
mutate_count_rebound_by_for_pattern() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, limits: &[usize]) {' \
        '    let available = control.available_at_entry();' \
        '    for &available in limits.iter() {' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# The binding takes the count from `available_at_entry` and nothing else.
mutate_inflated_count() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.available_at_entry() + control.capacity();' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A `while <count> != 0` body names its count once, in its first statement.
mutate_while_count_raised_in_body() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let mut remaining = control.available_at_entry();' \
        '    while remaining != 0 {' \
        '        remaining -= 1;' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        if apply(record) {' \
        '            remaining += 1;' \
        '        }' \
        '    }' \
        '}'
}
# A loop around a bounded drain must be finite by its form (J2-5).
mutate_outer_repeat() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    for _ in core::iter::repeat(()) {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# Markers out of order leave a region open where its file ends; that tail is still checked (J2-2).
append_open_tail() {
    printf '%s\n' '// REALTIME_POLICY_END' '// REALTIME_POLICY_BEGIN' \
        'fn drain_tail(control: &mut Consumer<Record>) {' \
        '    loop {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}' >>"$1"
}
mutate_open_tail_last_file() {
    append_open_tail "$root/hosts/host-web/src/lib.rs"
}
# Raw C strings and raw strings with more than sixteen `#` are literals too (J3-1): read as code,
# each would blank the drain below it up to the comment's quote.
mutate_raw_c_string_literals() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let _tags = (r#################"x"#################, cr#"a"b"#, cr"\");' \
        '    loop {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '    // "################' \
        '}'
}
# A region that ends inside a block comment or a literal was not read as code.
mutate_region_ends_in_comment() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls() {}' \
        '/* A note that runs past the marker:' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        '*/'
}
# A read of the count before its loop is a comparison outside any macro call but `debug_assert!`:
# a macro may expand its tokens into a new binding of the count.
mutate_count_in_user_macro() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.available_at_entry();' \
        '    rebind!(available == 0);' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A count may be capped by `.min(..)`, never raised by another method.
mutate_count_raised_by_max() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.available_at_entry().max(MAX_RECORDS_PER_BLOCK);' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A bare parameter is a finite outer loop only as a slice or an array.
mutate_outer_over_iterator_parameter() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, rounds: RangeFrom<u32>) {' \
        '    for _ in rounds {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# ... and only while nothing rebinds it before the loop.
mutate_outer_over_rebound_slice_parameter() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>, rounds: &[u32]) {' \
        '    let rounds = 0_u32..;' \
        '    for _ in rounds {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '    }' \
        '}'
}
# A `.zip(..)` ends with its receiver; an adapter after it may not repeat it.
mutate_outer_zip_then_cycle() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(controls: &[Consumer<Record>], gains: &[f32]) {' \
        '    for (control, gain) in controls.iter().zip(gains).cycle() {' \
        '        let available = control.available_at_entry();' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply_scaled(*gain, record);' \
        '        }' \
        '    }' \
        '}'
}
# A block item shadows the `let` of an enclosing block, even when declared after the loop (J3-5).
mutate_count_shadowed_by_block_static() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.available_at_entry();' \
        '    {' \
        '        for _ in 0..available {' \
        '            let Ok(record) = control.try_pop() else {' \
        '                break;' \
        '            };' \
        '            apply(record);' \
        '        }' \
        '        static available: usize = usize::MAX;' \
        '    }' \
        '}'
}
# A `let` made at the outermost level of a region that starts inside a function body leaves
# scope when that function closes (J3-5); here the loop reads an unmarked constant instead.
mutate_count_from_closed_region_function() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls() {}' \
        '// REALTIME_POLICY_END' \
        '#[allow(non_upper_case_globals)]' \
        'const available: usize = usize::MAX;' \
        'fn prepare_matrix(control: &mut Consumer<Record>) {' \
        '    // REALTIME_POLICY_BEGIN' \
        '    let available = control.available_at_entry();' \
        '}' \
        'fn drain_matrix(control: &mut Consumer<Record>) {' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# A block item shadows a slice parameter for its whole block too, even when it is declared after
# the loop: here the outer loop iterates an infinite range (stream-J batch verdict MINOR-1).
mutate_outer_over_slice_parameter_shadowed_by_block_const() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(rounds: &[u8], control: &mut Consumer<Record>) {' \
        '    {' \
        '        for _ in rounds {' \
        '            let available = control.available_at_entry();' \
        '            for _ in 0..available {' \
        '                let Ok(record) = control.try_pop() else {' \
        '                    break;' \
        '                };' \
        '                apply(record);' \
        '            }' \
        '        }' \
        '        #[allow(non_upper_case_globals)]' \
        '        const rounds: core::ops::RangeFrom<u32> = 0..;' \
        '    }' \
        '}'
}
# A count binding under `#[cfg(..)]` may not be compiled, leaving the earlier binding as the count
# (stream-J batch verdict MINOR-2).
mutate_count_under_cfg() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = usize::MAX;' \
        '    #[cfg(debug_assertions)]' \
        '    let available = control.available_at_entry();' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
# `r#while!` is a user macro, not the keyword, and may rebind the count (stream-J batch verdict
# NIT-1).
mutate_count_in_raw_identifier_macro() {
    replace_line "$root/$builtins_compiler" 'fn drain_matrix_controls() {}' \
        'macro_rules! r#while {' \
        '    ($x:ident $($rest:tt)*) => {' \
        '        let $x = usize::MAX;' \
        '    };' \
        '}' \
        '' \
        'fn drain_matrix_controls(control: &mut Consumer<Record>) {' \
        '    let available = control.available_at_entry();' \
        '    r#while!(available == 0);' \
        '    for _ in 0..available {' \
        '        let Ok(record) = control.try_pop() else {' \
        '            break;' \
        '        };' \
        '        apply(record);' \
        '    }' \
        '}'
}
for drain_case in wrapped_while_let let_else_loop path_pop constant_bound bound_not_at_entry \
    second_loop_in_bounded_region loop_inside_bounded_for from_fn repeat_with \
    bounded_drain_inside_outer_loop count_from_another_function shadowed_count reassigned_count \
    while_without_decrement while_decrement_not_first labelled_loop loop_expression \
    wrapped_while_body wrapped_for_bound wrapped_outer_while paren_led_header \
    continuation_led_header bare_from_fn entry_name_in_binding entry_name_in_header \
    header_past_closed_body array_from_fn_pop pattern_rebound_count \
    paren_open_range_outer while_match_body while_match_brace \
    struct_pattern_for if_else_condition as_cast_condition block_operand_condition \
    brace_in_string_header brace_in_char_header brace_in_raw_string_header brace_in_block_comment \
    count_rebound_by_for_pattern inflated_count while_count_raised_in_body outer_repeat \
    open_tail_last_file raw_c_string_literals region_ends_in_comment count_in_user_macro \
    count_raised_by_max outer_over_iterator_parameter outer_over_rebound_slice_parameter \
    outer_zip_then_cycle count_shadowed_by_block_static count_from_closed_region_function \
    outer_over_slice_parameter_shadowed_by_block_const count_under_cfg \
    count_in_raw_identifier_macro; do
    expect_failure "drain-${drain_case//_/-}" "$drain_class" "mutate_$drain_case"
done
# Deleting every marker of one file to silence the gate drops it out of the discovered set and
# trips the file floor instead of passing with less coverage.
expect_failure marked-file-count-floor 'expected at least twenty-six marked realtime files' \
    'sed -i "/REALTIME_POLICY/d" "$root/crates/builtins/src/lib.rs"'
expect_failure no-marked-files-uses-floor 'expected at least twenty-six marked realtime files' \
    'find "$root/crates" "$root/hosts" "$root/tools" -name "*.rs" -type f -exec sed -i "/REALTIME_POLICY/d" {} +'
# Deleting one marked region of a multi-region file leaves every marker matched and trips the
# region floor.
expect_failure marked-region-count-floor 'expected at least ninety-three marked realtime regions' \
    'drop_first_marked_region "$root/crates/rack/src/lib.rs"'
# The unmatched-marker check reaches files outside the old root too: the region keeps its
# BEGIN and loses its END, so the per-file count check, not the floors, must red.
expect_failure unmatched-markers-outside-root 'unmatched realtime policy markers' \
    'sed -i "/REALTIME_POLICY_END/d" "$root/hosts/host-web/src/lib.rs"'

# #1047 (moved from capi's `ffi_never_forms_a_whole_plan_reference`): each whole-plan borrow form
# in a production FFI function is refused, and the scan cannot pass by losing its region.
for form in '\&*plan' '\&mut *plan' '\&(*plan)' '\&mut (*plan)' '\&mut  *plan'; do
    expect_failure "whole-plan-${form//[^a-z]/_}" "$whole_plan_class" \
        "sed -i 's|^    0\$|    let _ = unsafe { $form };\n    0|' \"\$root/crates/capi/src/ffi.rs\""
done
expect_failure whole-plan-at-line-end "$whole_plan_class" \
    'sed -i "s|^    0\$|    let whole = unsafe {\n        \&*plan\n    };\n    0|" "$root/crates/capi/src/ffi.rs"'
expect_failure whole-plan-after-a-comment-line "$whole_plan_class" \
    'sed -i "s|^    0\$|    // a comment line above is not an exemption\n    let _ = unsafe { \&*plan };\n    0|" "$root/crates/capi/src/ffi.rs"'
expect_failure capi-ffi-missing 'missing crates/capi/src/ffi.rs' \
    'rm -f "$root/crates/capi/src/ffi.rs"'
expect_failure capi-render-entry-only-in-tests 'has no production render entry point to scan' \
    'sed -i "s/^pub unsafe extern \"C\" fn miso_engine_v1_render_f32_planar/fn moved_render/; s/^mod tests {$/mod tests {\n    fn miso_engine_v1_render_f32_planar() {}/" "$root/crates/capi/src/ffi.rs"'

# Selective executable-tool failures prove late statuses are observed after useful output. Each
# shim delegates every unrelated invocation to the physical tool.
expect_tool_error() {
    local name="$1" tool="$2" mode="$3" expected="$4" partial="$5"
    local root="$scratch_root/tool-$name" shim="$scratch_root/shim-$name" output
    create_fixture "$root"
    mkdir -p "$shim"
    cat >"$shim/$tool" <<'SHIM'
#!/usr/bin/env bash
set -u
joined="$*"
hit=0
case "$INJECT_MODE:$TOOL_NAME" in
  unsafe-scan:rg) [[ "$joined" == *unsafe*crates*hosts*tools* && "$joined" != '-v '* ]] && hit=1 ;;
  unsafe-filter:rg) [[ "$1" == '-v' ]] && hit=1 ;;
  marker-discovery:rg) [[ "$joined" == *'-l REALTIME_POLICY_BEGIN'* ]] && hit=1 ;;
  begin-count:rg) [[ "$joined" == *'-c REALTIME_POLICY_BEGIN'*runtime.rs* ]] && hit=1 ;;
  end-count:rg) [[ "$joined" == *'-c REALTIME_POLICY_END'*runtime.rs* ]] && hit=1 ;;
  final-predicate:rg) [[ "$joined" == *'Vec::'* ]] && hit=1 ;;
  marker-sort:sort) hit=1 ;;
  body-read:awk) [[ "$joined" == *runtime.rs* && "$joined" != *available_at_entry* ]] && hit=1 ;;
  drain-bound:awk) [[ "$joined" == *available_at_entry* ]] && hit=1 ;;
  capi-region:awk) [[ "$joined" == *capi/src/ffi.rs* ]] && hit=1 ;;
  whole-plan-scan:rg) [[ "$joined" == *'plan|'* ]] && hit=1 ;;
esac
if (( hit )); then
    if [[ "$PARTIAL" == 1 ]]; then "$REAL_TOOL" "$@" || true; fi
    printf 'injected-%s-error\n' "$INJECT_MODE" >&2
    exit 2
fi
exec "$REAL_TOOL" "$@"
SHIM
    chmod +x "$shim/$tool"
    if output="$(env PATH="$shim:$PATH" TOOL_NAME="$tool" REAL_TOOL="$(command -v "$tool")" INJECT_MODE="$mode" PARTIAL="$partial" bash "$policy_script" "$root" 2>&1)"; then
        printf 'realtime injected failure unexpectedly passed: %s\n' "$name" >&2; exit 1
    fi
    printf '%s\n' "$output" | rg -qF "injected-$mode-error" || { printf 'missing injected diagnostic: %s\n%s\n' "$name" "$output" >&2; exit 1; }
    printf '%s\n' "$output" | rg -qF "$expected" || { printf 'wrong injected failure class: %s\n%s\n' "$name" "$output" >&2; exit 1; }
}

for partial in 0 1; do
    expect_tool_error "unsafe-scan-$partial" rg unsafe-scan 'unsafe source scan' "$partial"
    expect_tool_error "unsafe-filter-$partial" rg unsafe-filter 'unsafe source exclusions' "$partial"
    expect_tool_error "marker-discovery-$partial" rg marker-discovery 'realtime marker discovery failed' "$partial"
    expect_tool_error "marker-sort-$partial" sort marker-sort 'realtime marker discovery sort errored' "$partial"
    expect_tool_error "begin-count-$partial" rg begin-count 'BEGIN marker count failed' "$partial"
    expect_tool_error "end-count-$partial" rg end-count 'END marker count failed' "$partial"
    expect_tool_error "body-read-$partial" awk body-read 'realtime body extraction failed' "$partial"
    expect_tool_error "drain-bound-$partial" awk drain-bound 'realtime drain-bound scan failed' "$partial"
    expect_tool_error "final-predicate-$partial" rg final-predicate 'marked realtime forbidden-body predicate' "$partial"
    expect_tool_error "capi-region-$partial" awk capi-region 'capi production-region extraction failed' "$partial"
    expect_tool_error "whole-plan-scan-$partial" rg whole-plan-scan 'whole-plan reference scan errored' "$partial"
done

# Counter-mutants must fail at this suite's unexpected-success assertion. These disposable
# copies prove that the assertions distinguish a swallowed producer status from the hardened gate.
prove_realtime_mutant_rejected() {
    local name="$1" edit="$2" mode="$3" partial="${4:-0}"
    local mutant_dir="$scratch_root/mutant-$name" output status
    mkdir -p "$mutant_dir/lib"; cp "$policy_script" "$mutant_dir/check.sh"
    ln -s "$script_directory/lib/gate.sh" "$mutant_dir/lib/gate.sh"
    sed -i "$edit" "$mutant_dir/check.sh"
    set +e
    output="$(policy_script="$mutant_dir/check.sh"; expect_tool_error "mutant-$name" "${5:-rg}" "$mode" ignored "$partial" 2>&1)"
    status=$?
    set -e
    [[ $status == 1 ]] && printf '%s\n' "$output" | rg -qF 'unexpectedly passed' || {
        printf 'realtime counter-mutant did not reach intended assertion: %s\n%s\n' "$name" "$output" >&2; exit 1;
    }
}
prove_realtime_mutant_rejected unsafe-status 's/)" || exit \$?/)" || true/' unsafe-scan
prove_realtime_mutant_rejected marker-discovery \
  '/marked_files_rc -le 1.*fail/c\[[ $marked_files_rc -le 1 ]] || marked_files_raw="${marked_files_raw%$'\''\n'\''injected-marker-discovery-error}"' \
  marker-discovery 1
prove_realtime_mutant_rejected per-file-read \
  '/realtime body extraction failed/c\    '\'' "$source" 2>\&1)"; then :; else :; fi' \
  body-read 1 awk
prove_realtime_mutant_rejected drain-bound \
  '/realtime drain-bound scan failed/s/else rc=.*fi$/else drain_hits=""; fi/' \
  drain-bound 1 awk
prove_realtime_mutant_rejected final-predicate '/scratch_file.*exit/s/exit.*$/true/' final-predicate

printf 'realtime policy mutation tests: ok\n'
