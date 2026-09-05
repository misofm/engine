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
        "$root/crates/effect-compiler/tests" \
        "$root/crates/effect-contract/src" \
        "$root/crates/effect-package/src" \
        "$root/crates/effect-package/tests" \
        "$root/crates/graph/src" \
        "$root/crates/rack/src" \
        "$root/crates/builtins/src" \
        "$root/crates/session/tests" \
        "$root/hosts/host-web/src" \
        "$root/hosts/host-web/tests" \
        "$root/tools/bench-support/src" \
        "$root/tools/audit/src" \
        "$root/tools/native-pcm-runner/src" \
        "$root/tools/bench/src" \
        "$root/sidecars"
    # The marked file set mirrors the real tree after #371 (RT-16/IO-14): twelve files and
    # forty-two regions across crates/ and hosts/, so the floors in the gate and the discovery
    # walk are exercised against the same shape the gate sees on main. Column-zero markers and
    # indented markers (as in the real `impl`-block regions) both appear.
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
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn pop() {}' \
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
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn exchange_plane() {}' \
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
        'fn console_process_mono() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn console_process() {}' \
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
    # Five regions, mirroring the real crates/builtins/src/lib.rs after #371.
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn input_process() {}' \
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
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn capi_boundary() {}' \
        >"$root/crates/capi/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for LifecycleAllocator {}' \
        'struct LifecycleAllocator;' \
        >"$root/crates/capi/tests/resource_lifecycle.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn descriptor_capi_boundary() {}' \
        >"$root/crates/effect-package/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for PackageAllocationAudit {}' \
        'struct PackageAllocationAudit;' \
        >"$root/crates/effect-package/tests/package_allocation.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for MigrationAllocationAudit {}' \
        'struct MigrationAllocationAudit;' \
        >"$root/crates/effect-compiler/tests/migration_terminal.rs"
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
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn frozen_c_abi_adapter() {}' \
        >"$root/tools/native-pcm-runner/src/lib.rs"
    # #104 phase B: the fourteen audited `GlobalAlloc` copies became one. `bench-support/src/alloc.rs`
    # is the only file under `tools/` that owns the allocator wrapper, and eleven tool paths left
    # this list because they no longer contain `unsafe` at all.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for AuditedAllocator {}' \
        'struct AuditedAllocator;' \
        >"$root/tools/bench-support/src/alloc.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn follow() {}' \
        >"$root/tools/bench/src/protocol.rs"
    printf '%s\n' \
        'fn measure() {}' \
        >"$root/tools/bench/src/rack.rs"
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
    "$empty_bodies/hosts/host-web/src/lib.rs"
bash "$policy_script" "$empty_bodies" >/dev/null

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
expect_failure unsafe-outside-native-pcm-runner-lib "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/native-pcm-runner/src/other.rs"'
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
expect_failure unsafe-outside-effect-package-ffi "$unsafe_class" \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/crates/effect-package/src/lib.rs"'
expect_failure unsafe-in-second-effect-package-ffi-path "$unsafe_class" \
    'mkdir -p "$root/crates/effect-package/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/effect-package/src/ffi/other.rs"'
expect_failure unsafe-outside-package-allocation-audit "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/effect-package/tests/other.rs"'
expect_failure unsafe-outside-migration-allocation-audit "$unsafe_class" \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/effect-compiler/tests/other.rs"'
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
# Deleting every marker of one file to silence the gate drops it out of the discovered set and
# trips the file floor instead of passing with less coverage.
expect_failure marked-file-count-floor 'expected at least twelve marked realtime files' \
    'sed -i "/REALTIME_POLICY/d" "$root/crates/builtins/src/lib.rs"'
expect_failure no-marked-files-uses-floor 'expected at least twelve marked realtime files' \
    'find "$root/crates" "$root/hosts" "$root/tools" -name "*.rs" -type f -exec sed -i "/REALTIME_POLICY/d" {} +'
# Deleting one marked region of a multi-region file leaves every marker matched and trips the
# region floor.
expect_failure marked-region-count-floor 'expected at least forty-two marked realtime regions' \
    'drop_first_marked_region "$root/crates/rack/src/lib.rs"'
# The unmatched-marker check reaches files outside the old root too: the region keeps its
# BEGIN and loses its END, so the per-file count check, not the floors, must red.
expect_failure unmatched-markers-outside-root 'unmatched realtime policy markers' \
    'sed -i "/REALTIME_POLICY_END/d" "$root/hosts/host-web/src/lib.rs"'

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
  unsafe-scan:rg) [[ "$joined" == *unsafe*crates*hosts*tools*sidecars* && "$joined" != '-v '* ]] && hit=1 ;;
  unsafe-filter:rg) [[ "$1" == '-v' ]] && hit=1 ;;
  marker-discovery:rg) [[ "$joined" == *'-l REALTIME_POLICY_BEGIN'* ]] && hit=1 ;;
  begin-count:rg) [[ "$joined" == *'-c REALTIME_POLICY_BEGIN'*runtime.rs* ]] && hit=1 ;;
  end-count:rg) [[ "$joined" == *'-c REALTIME_POLICY_END'*runtime.rs* ]] && hit=1 ;;
  final-predicate:rg) [[ "$joined" == *'Vec::'* ]] && hit=1 ;;
  marker-sort:sort) hit=1 ;;
  body-read:awk) [[ "$joined" == *runtime.rs* ]] && hit=1 ;;
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
    expect_tool_error "final-predicate-$partial" rg final-predicate 'marked realtime forbidden-body predicate' "$partial"
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
prove_realtime_mutant_rejected final-predicate '/scratch_file.*exit/s/exit.*$/true/' final-predicate

printf 'realtime policy mutation tests: ok\n'
