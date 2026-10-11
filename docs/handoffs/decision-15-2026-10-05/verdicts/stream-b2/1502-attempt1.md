PASS

# #1502 attempt 1: verdict (adversarial verifier, 2026-10-11)

Commits d1e0a69dd, 07d5c60ae, fec3af7e4 on `codex/d15-stream-b2` (base 48a13bbc1). I exported fec3af7e4
to `/tmp/claude-1002/v1502/tree` and built it there. I did not build in the worktree or edit it.

The attempt meets D1-D9 and gates 1-7. There is no BLOCKER and no MAJOR. Two MINOR findings are for
root: one is a spec gap and one is a wrong sentence in the evidence.

## Findings

### MINOR

- **m1. The browser's "exact" retained total does not count the gate's allocation (a spec gap).**
  `PreparedRenderPlan::prepare` now makes one `Arc<RevisionGate>` for every plan
  (`crates/engine/src/realtime/plan.rs:701`). This is 16 bytes on wasm32 and 24 bytes on x86-64.
  D1 charges it only in the control plane (`crates/control-plane/src/compile.rs`, the nested
  `with_carried`). The browser copies host-core's `graph_session_plus_plan_bytes` unchanged
  (`hosts/host-web/src/lib.rs:6821`). `exact_retained_bytes` (`hosts/host-web/src/lib.rs:6560`),
  which its doc calls "Exact aggregate retained bytes", therefore leaves out one real allocation
  per browser plan. No gate catches this. `expected.json` does not move (29842 of 35648), and its
  rows are ceilings. The implementer followed frozen D1, so this is not an attempt defect. Root
  should choose one of two fixes:
  - charge `plan_revision_gate_retained_bytes()` once in host-core's graph row, so both hosts count
    it, and remove the control plane's addition; or
  - record the browser charge under #1382.

- **m2. The attempt record claims more L3 coverage than exists.** Choice 3 says that L3's defect
  class "is held by the new loom model's `snapshot == 1` assertion". That is not correct. The model
  picks the gate in its own code (`crates/engine/src/realtime/spsc.rs:1230`,
  `claimed.as_ref().map_or(&running_gate, ...)`). Thus the assertion at `:1254` can go red only
  when the model itself is changed. No production mutation can make it red. The argument that
  actually holds is structural:
  - the reader no longer has a cached index;
  - `RealtimePlanOwner` changes its active plan only when `claim` returns `Some`;
  - the mailbox models keep their "adopted and withdrawn both" assertions.

  Correct that sentence. Also amend the line in gate 4 that says "L3 stays red" (spec `:211`) to
  say that L3 has no object any more. The coverage of what remains is sufficient.

### NIT

- **n1. Edits outside the authorized paths. All three are justified, and none breaks a rule.**
  - `scripts/check-web-audioworklet-callgraph.py:179,184` updates two entries. D2 requires
    `begin_live_block` to be a trait method on the boxed executor, so the second `call_indirect`
    in `render_inner` comes from the spec. The `Ms2`->`Ms4` change is the impl index that moved in
    the mangled name. Each updated entry gives its reason inline, which the script requires.
  - `docs/C_ABI_V1_QUALIFICATION.md`: the sentence named the deleted revision words. The new text
    is accurate.
  - `crates/capi/tests/resource_lifecycle.rs:1652`: the assertion is an accounting identity of
    computed terms (row = host row + carry program + `plan_revision_gate_retained_bytes()`). It
    is not a literal byte count or a digest, so AGENTS.md's digest and byte rule permits it. It is
    the only test that catches an uncharged gate (M9 below).

  Root should add these three paths to the spec's "Authorized paths" in the record.

- **n2. The `# Panics` doc of `plan_exchange_at_revision`
  (`crates/engine/src/realtime/plan_exchange.rs:373`) claims too much.** It says that a revision
  not above the gate's published one panics. That check is only `GatePin::publish`'s
  `debug_assert`, so a release build does not panic. The foreign-handle check
  (`assert!`, `:384`) is acceptable:
  - it runs only on the control path;
  - the C ABI cannot reach it, because `compile.rs:645` takes the handle from the same plan that
    `:776` passes;
  - `catch_unwind` does not apply, because release builds use `panic = "abort"`;
  - `republish` already uses the same pattern.

- **n3.** Keeping I7 as a tombstone (`spsc.rs:487`) so that I8 keeps its name is acceptable.

## Points I was asked to judge

- **`RenderReport::live_snapshot`** is a public field. D2 requires it. `RenderReport` is not part
  of the ABI, and `check-capi-abi.sh` is ok.
- **Control-plane publishes with no stamp, until #1312.** I found nothing that is published
  against the gate's rules. On each handle the revisions strictly increase:
  - the initial plan's `R0` goes on a fresh gate at 0;
  - a candidate's fresh gate gets `r` after the protocol commit and before
    `reservation.commit()`, and all steps between are infallible;
  - live commits add `r+1` and later on the newest epoch's handle;
  - a rebuild is refused while a candidate is pending.

  `publish`'s debug assertions ran in every capi and control-plane test (debug), and all were
  green. No cell exists yet, so no protected slot can be overwritten. The order that root's D7
  ruling relies on is present: D1 (the handle is held in its `ProviderEpoch` and moves only with
  it) and D6 (`r` is published on the candidate's handle before `reservation.commit()`).
- **Watermark and errors.** The watermark advances only after an `Ok` block
  (`render_active`). `live_block_errors` counts only failures after the snapshot. The shape check
  and the clock check (`OutputShape`, `TimeOverflow`, `TimeDiscontinuity`) come before the
  snapshot and are not counted (M3 and M4 below). The mirror stores the count on the error path.
- **Every live drain runs every block.** `begin_live_block` runs for every block that has an
  executor, with no silence branch in front of it.
- **The acked-batch question: can an ack ever precede a drop? No.** No queue is added, and the
  mailbox loses its revision words. Each commit's publication is still its last write, and for a
  rebuild it is the last step before `reservation.commit()`. A withdrawn candidate carries its
  own gate (the loom model checks that the gate reads 8 after withdrawal). The retirement queue is
  unchanged.

## Test value, one sentence per new or rewritten test

- `each_block_takes_its_snapshot_from_its_own_plans_gate`: goes red when the executor receives a
  snapshot that is not its own plan's, for example `ALL` (M1) or an unpublished initial revision
  (M13). No earlier test could see `S`.
- `the_direct_render_paths_take_their_own_gates_snapshot`: goes red when the direct
  `render`/`render_contiguous`, or the no-executor branch, reports or hands out a snapshot that is
  not the plan's own (M1). It is the only check of `report.live_snapshot` without an executor.
- `a_block_the_executor_fails_after_the_snapshot_is_counted`: goes red when a block that fails
  after the snapshot is not counted (M3) or advances the watermark.
- `a_block_that_errors_publishes_nothing` (extended): goes red when a block that the shape check
  refuses is counted as a live-block error, or calls the executor (M4).
- `a_plan_hands_out_its_gate_handle_once`: goes red when a second control handle (a second
  writer and pin) is handed out. This is trivial but it is the only check.
- `the_initial_revision_is_published_on_the_initial_plans_gate`: goes red when a foreign handle
  is accepted (M8).
- `the_live_block_error_count_reaches_the_any_thread_query` (control-plane): goes red when the
  count does not leave the render thread (M10), or when a refused block is counted (M4).
- `spsc_loom_plan_mailbox_snapshot_follows_the_claimed_plan_gate`: goes red when a candidate's
  revision is published after its commit, so that a claim can snapshot the candidate below its
  content (L1). It replaces `..._revision_follows_the_pending_candidate`, which was deleted with
  the words it tested.
- The three rewritten #1314 tests keep their assertions on plan gates. Only the
  `RevisionTarget` assertions are gone, together with the type.

## Mutations I ran again (one at a time, then reverted; logs in `/tmp/claude-1002/v1502/evidence/mut/`)

| Mutation | Result |
|---|---|
| M1: executor receives `LiveSnapshot::ALL` | RED: gate-1 tests (both), gate 5 |
| M2 (#1314 M14): watermark from a snapshot taken after the executor rendered | engine GREEN (single-thread, as expected); `a_watermark_advance_names_a_block_that_applied_the_edit` (capi, release) RED, "block 8: the watermark advanced to edit 8 in a block that did not apply it" |
| M3: failure after the snapshot not counted | RED: `a_block_the_executor_fails_after_the_snapshot_is_counted` |
| M4: failure before the snapshot counted | RED: `a_block_that_errors_publishes_nothing`; control-plane mirror test |
| M6: live commit publishes on the current epoch, not the newest | RED: `a_live_edit_on_a_pending_candidate_completes_at_the_swap` |
| M7: rebuild publishes `r` on the running plan's gate | RED: the same, and `a_scheduled_candidate_survives_service_until_render_adopts_it` |
| M8: ownership `assert!` disabled | RED: `the_initial_revision_is_published_on_the_initial_plans_gate` |
| M9: gate bytes not charged | RED: `double_live_oracle_drives_exact_and_one_below_c_caps` |
| M10: mirror stores 0 | RED: `the_live_block_error_count_reaches_the_any_thread_query` |
| M13: initial revision not published | engine RED (gate 1); capi GREEN, which is harmless until #1504 because no cell exists |
| L1 (loom): candidate's 7 published after the commit | RED: `spsc_loom_plan_mailbox_snapshot_follows_the_claimed_plan_gate` |

## Gates I ran on fec3af7e4 (`/tmp/claude-1002/v1502/evidence/u1/`)

| Gate | Result |
|---|---|
| loom `spsc_loom` (qualification.yml command) | 13 passed |
| the same with `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true` | 13 passed |
| `cargo test --locked -p engine --features realtime-audit` | 80+4+1+2 passed |
| `cargo test --locked --release -p engine --lib latest_cell` | ok |
| `cargo test --locked -p control-plane --features test-support` | ok |
| `cargo test --locked -p capi` | ok |
| `cargo test --locked --release -p capi --test plan_swap_race` | ok |
| `audit capi` | 0 allocations, deallocations, locks, syscalls; `total_violations` 0 |
| `check-realtime-policy.sh` and `test-realtime-policy.sh` | ok (96 regions in 27 files) |
| `check-workspace-policy.sh` and `test-workspace-policy.sh` | ok |
| `check-capi-abi.sh` | ok (the script reads `target/release`, so I linked it to the export target) |
| `check-cross-targets.sh` | PASS |
| fmt, clippy `-D warnings`, doc `-D warnings` | ok |
| worklet chain: build `--named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` (with a private TMPDIR, left empty) | all ok |

I also ran the qualification workspace debug tests and the doctests at the batch head 2340d7d8d,
which contains this unit. Both were green (see the #1432 n3 verdict).
