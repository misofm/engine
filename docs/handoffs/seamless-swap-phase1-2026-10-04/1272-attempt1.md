# #1272 attempt 1 verdict: PASS

PASS with minors: no BLOCKER and no MAJOR. Four MINOR findings and six NITs. The slice does what
D1-D6 freeze, every objective gate holds, and the key tests turn red under the defects they name.
The MINORs are a browser accounting gap the slice's authorized paths could not reach, and three
groups of untested branches in the cap and carry-rule code. That code is correct as written; my
probes prove each branch.

## Commit reviewed

- `git diff 1a911e31f 2a6f2c10c` (one commit, `2a6f2c10c`, on #1271's `1a911e31f`). Exported with
  `git archive 2a6f2c10c` to `/tmp/claude-1002/v1272/attempt1/`. Every build and test below ran
  there, with its own `target/`. Logs: `/tmp/claude-1002/v1272/attempt1/vlogs/`. Probes:
  `vlogs/zz_verify_1272.rs`. Mutation driver and logs: `vlogs/mut/`.
- Files: `crates/host-core/src/{prepare,source,lib,render_session}.rs`,
  `crates/host-core/tests/{successor_swap.rs,support/successor.rs,source_diagnostics.rs}`,
  `crates/capi/tests/resource_lifecycle.rs` and the slice spec. All are authorized except
  `source_diagnostics.rs` (NIT-1).

## Gates re-run (all in the export)

Debug builds used `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_{DEV,TEST}_DEBUG=0` because the shared
disk was full. The flags change only debug info, not semantics.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | exit 0 |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (56 regions in 15 files) / ok |
| `check-host-core-policy.sh` / `test-host-core-policy.sh` | ok / ok |
| `cargo test --locked -p host-core --features host-core/test-support,graph/test-support` | 27 binaries green, `successor_swap` 6/6 |
| `cargo test --locked -p host-web --features host-web/test-support` | 187 passed, 0 failed |
| `cargo test --locked -p capi` | lib 35, `resource_lifecycle` 11, green |
| `cargo test --locked -p console-workload` | green; digests kept |
| `check-capi-abi.sh` | ok (shared and static) |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 100000 calls; 0 allocations, 0 deallocations, 0 syscalls, 0 violations |
| `check-cross-targets.sh` | PASS. host-core iOS `memset_pattern16` = 4, at its ceiling of 4 |
| `build-web-audioworklet.sh --named-twin` | exit 0; module `feeb20c3…1d47`, matching the attempt record |
| `check-web-audioworklet.sh --without-metadata-regeneration` | exit 0 (see note) |
| `check-browser-expected-resources.py --artifacts` | exit 0; `sourceTotalBytes` 3294 of 3648 |
| `test-web-audioworklet.sh` | exit 0 (see note) |

Note: the first runs of `check-web-audioworklet.sh` and `test-web-audioworklet.sh` failed with
ENOSPC, because the shared filesystem reached 0 bytes free. I freed my own cross-target output and
re-ran both. Both were green.

ARTIFACT CHANGED is confirmed. The pin is correctly left alone (`docs/RELEASE.md`, "Between
releases").

Not run: CI's full `test-debug-a` workspace sweep and AArch64 (CI only). The focused crates and
every direct consumer of the changed public types (host-web, capi, console-workload) were run.

## #1271 review MINORs carried into this slice

- **MINOR 5: covered.** Gate 5 prepares both plans through host-core. A real `PcmSourceRing`
  consumer is carried through the real `SourceGraphSourceSetDriver::adopt_sources`, and the swap
  block reports `Carried`. My mutation M6 put a `Box` in `adopt_sources`
  (`crates/source/src/lib.rs:1702`). The gate went RED at block 6 with `(1, 0, 1)` when run alone,
  and the whole binary aborted under the default `Mode::Abort`.
- **MINOR 6: implemented, but not defended by a test (MINOR-2 below).** The bytes are charged
  against `maximum_graph_session_plus_plan_bytes` (`prepare.rs:1419`) and reported as
  `HostPrepareReport::carry_program_retained_bytes`. My probe binary-searched the smallest graph
  cap that admits the successor. It is exactly a fresh B's plus 8, so the charge is right.

## Implementer notes, judged

- **`source_diagnostics.rs` edit: necessary and minimal.** The file's header says its exhaustive
  `variant_index` is designed to fail the build until a new `SourceControlError` variant gets a
  row. D5 mandates `Vacated`, so the edit is forced. It adds one row and one arm and changes no
  assertion. The spec's path list missed it (NIT-1).
- **Program installed only when at least one source carries: acceptable under P11 for this
  slice.** A successor with no carried owner holds nothing a wrong predecessor could corrupt, and
  it renders exactly as a fresh plan. The cost is that such a mismatch goes unrecorded (NIT-2).
- **Vacated is checked before the rate, channel and region checks in `submit`/`seek`: correct.** A
  vacant set is the wrong set whatever the payload, and both paths refuse before touching anything.
  V8 (seek's `Vacated` replaced by another code) turns gate 1 red at both widths.
- **The capi oracle's fifth owner is correct.** `compile_live - (host_live - inventory)` is right
  while capi drops the inventory in `prepare_runtime` (`crates/capi/src/runtime/compile.rs:421-436`
  moves out only `sources` and `plan`). The new exact row is RED under `retained_bytes() + 1`
  (62 against 63) in `capi_retained_bytes_charge_every_byte_the_compile_retains`.
- **Harness via `#[path]`: fine.** It keeps the shared `support/mod.rs` untouched, and slices 7-15
  can include it the same way.
- **The P1 rule matches P1.3/P1.4 for sources.** `session::Source` is exactly `{id, content,
  channels, bit_depth, frames}` (`crates/session/src/model.rs:165-176`), with no float fields. So
  `**committed == *source` is the committed-model declaration comparison of P1.4, never prepared
  words. `row.ring == ring` is the P1.3 prepared-layout check against the predecessor's inventory.
  Track mappings are correctly ignored.

## Test value (one sentence per new or rewritten test)

- `an_added_muted_track_keeps_the_source_playing_at_eight_lanes` / `_four_lanes`: red when a
  successor plays a fresh ring for an unchanged source, or the carry or producer hand-over loses the
  queued block. Also red when a moved-out set reports anything but `Vacated` on submit or seek.
  Verified RED: M1, M7, V8.
- `a_removed_source_leaves_the_kept_source_playing`: red when either the carry program or
  `adopt_persisting` pairs sources by index instead of ID. Verified RED: M2 (hand-over by index)
  and M3 (program pairs `(index, index)`).
- `a_changed_source_gets_its_own_ring`: red when a source whose declaration changed is carried,
  or when `adopt_persisting` overwrites a successor entry that already has its own producer.
  Verified RED: M4 and V7.
- `a_successor_charges_allocated_and_carried_rings_to_the_source_cap`: red when the total source
  cap counts only allocated rings. Verified RED: M5.
- `the_swap_block_allocates_and_frees_nothing`: red when the real source carry, or anything else
  in the swap block, allocates or frees on the render thread. Verified RED: M6, `(1, 0, 1)` at
  block 6.
- `resource_lifecycle.rs`, inventory owner row: red when `inventory_retained_bytes` misreports
  what the inventory holds. Verified RED: +1 byte.
- `source_diagnostics.rs`, `Vacated` row: red when `Vacated`'s code or its
  backpressure/internal classification drifts. This is a compile-time-forced row of an existing
  pinned table.

## Extra probes (all green on the candidate)

- **Chain A → B → C at Simd8 and Simd4.** Two successive successor swaps, each adding a muted
  track. Both report `Carried`, and all 12 blocks are bit-identical to a fresh C fed from frame 0.
  A successor's own inventory therefore describes its carried ring correctly for the next swap.
- **Different `source_ring_frames`:** nothing carried.
- **Content-only change:** nothing carried.
- **Carry program charge:** the successor's minimal graph cap is exactly a fresh B's plus 8.
- **Overhead cap:** one byte below allocated plus carried overhead refuses with
  `host.source.resource.limit`.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **The browser now retains the inventory without charging it.** `AudioWorkletEngineHost` keeps
   the whole `PreparedHost` (`hosts/host-web/src/lib.rs:1579`, `host: PreparedHost`), so since this
   commit it holds `PlanStateInventory`: per source, a 40-byte row on wasm32 plus the ID text. The
   browser charges only `control_retained_bytes + session_model_bytes` (`:6688`).
   `exact_retained_bytes` (`:6525`) still documents its three rows as "the complete retained set"
   and refuses boot on it (`:2168`). That is now false by the inventory's bytes. No gate sees it:
   `check-browser-expected-resources.py` compares rows, and no host-web test measures live heap
   against the exact sum. The amount is small and the audio is unaffected, but it is a silent
   accounting regression in the shipped module, and capi got the matching oracle fix while the
   browser did not. The fix needs a path this slice was not authorized to touch, and the attempt
   record does not mention it.
   **Fix (coordinator):** add `engine.inventory_retained_bytes` to host-web's `ready_metadata`
   (one line, `:6689`), in #1273's batch or a tiny follow-up before B2. Alternatively, have
   host-web drop the inventory until B2 needs it. Either way, record it in B2's brief.
2. **MINOR 6's charge has no defending test.** V1 removes `carry_program_bytes` from the
   `maximum_graph_session_plus_plan_bytes` admission (`prepare.rs:1419`), and every committed test
   stays green. Gate 1's `carry_program_retained_bytes == 8` reads the reported row, which is walked
   from the installed plan and is not the admission sum.
   **Fix:** a successor case asserting that a graph cap admitting a fresh B refuses the successor
   with `host.graph.resource.limit`, or my probe's minimal-cap form
   (`zz_the_carry_program_is_charged_to_the_graph_cap`, RED under V1).
3. **Two halves of the D3 rule are untested.**
   - V3 drops `row.ring == ring` (`prepare.rs:1062`). Every committed test stays green, and the
     carried bytes would then be charged at the successor's ring configuration, not the ring
     actually carried.
   - V5 narrows the declaration comparison to `frames` and `channels`. Every committed test stays
     green, so a `SetSourceContent` with changed content would carry the old ring. The umbrella's
     Coordination section names exactly that trigger for slice 4's race.

   **Fix:** add two cases to `successor_swap.rs`: a successor prepared with a different
   `source_ring_frames`, and a content-only change. Each asserts `carried_source_total_bytes == 0`.
   My probes `zz_a_different_ring_config_is_not_carried` and `zz_a_content_change_is_not_carried`
   are RED under V3 and V5.
4. **The carried halves of the overhead cap and the largest-allocation fold are untested.**
   - V2 makes the overhead cap count allocated rings only (`prepare.rs:1445`), and every committed
     test stays green. My probe `zz_the_overhead_cap_counts_carried_overhead` is RED under it.
   - V4 drops `.max(carried_largest_bytes)` (`prepare.rs:1452`), and everything, probes included,
     stays green.

   **Fix:** extend gate 4 to the overhead cap (one more `required - 1` refusal). For the largest
   allocation, either assert that a successor whose only ring is carried reports
   `largest_engine_allocation_bytes` at least the carried ring's largest allocation, or delete the
   fold if it is not meant to be a claim.

### NIT

1. **The spec's authorized paths omitted `crates/host-core/tests/source_diagnostics.rs`.** D5's
   new variant forces that edit by design. The edit is fine; future briefs that add a
   `SourceControlError` variant should list the file.
2. **A wrong predecessor goes unrecorded when nothing carries.** The program is installed only
   when at least one source carries (`prepare.rs:1108`), so a mismatch on a no-carry successor
   returns `NotRequested`, not `PredecessorMismatch`, and `carry_mismatch_count` stays put.
   Harmless today. When slices 7-15 add state families, this condition must become "any owner
   carries". Installing an empty program whenever a base is given would cost zero heap bytes and
   make the counter a host-bug detector. Worth one sentence in slice 7's brief.
3. **Gate 1 pins a byte literal.** `assert_eq!(report.carry_program_retained_bytes, 8)`
   (`successor_swap.rs:188`). The claim is "one move", so
   `core::mem::size_of::<(u32, u32)>() as u64` states it without pinning a layout byte count.
4. **A stale rationale in `lib.rs`.** `crates/host-core/src/lib.rs:113-115` says width tests
   live in `src/` because `tests/` cannot see the width seam. A `test-support` width seam now
   exists, so the rationale is stale (the comment is still literally true of the `#[cfg(test)]`
   seam).
5. **Gate 5 runs only at `Backend::current()`.** That is Simd8 on x86 and Simd4 on AArch64 CI. The
   carry is width-independent, so this is acceptable. Gates 1 and 2 cover both widths.
6. **Gate 5 sets the process-global `Mode::Count` while sibling tests run.** It follows the
   established `RestoreMode` pattern (`crates/graph-compiler/tests/bypass_resources.rs`), so no
   action is needed.

## Hunt list, checked

- **Render-thread allocations, frees, locks and syscalls.** This commit changes no render code;
  everything in it is control-side. The swap block measures `(0, 0, 0)` and the audit `(0, 0)`.
  `audit capi` shows 0 allocations and 0 syscalls.
- **Lost acked edit or accepted PCM.** The queued block accepted before the swap plays at the
  swap block, and producer submits made after `adopt_persisting` reach the carried ring (gate 1
  is bit-exact). The forced-mismatch loss is documented on `adopt_persisting` as D6 and P11
  require.
- **Wrong predecessor.** The program names `plan_identity` from the predecessor's inventory. The
  chain probe shows a successor's inventory carries the right identity and indices onward.
- **State carried that P1 says must restart.** For sources, carry requires an equal committed
  declaration and an equal ring configuration (both proven by probes); a changed declaration
  restarts.
- **Continuity.** Bit-exact over 12 blocks at both widths, against fresh B from sample 0. The
  oracle can fail: a fresh successor differs at block 7.
- **Scope.** No C ABI or browser source change, no DSP carry, no anchored seek. Over-scoping:
  none.
