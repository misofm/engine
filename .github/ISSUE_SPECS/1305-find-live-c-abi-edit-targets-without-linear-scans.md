# Find live C ABI edit targets without linear scans

Control-thread performance follow-up of *Apply value-only effect parameter edits to the running
C ABI plan* (#1264, slice of #1053), from its verdict's NIT 2 and NIT 3
(`docs/handoffs/live-updates-1053/1264-attempt1.md`, recorded as "#1264 N2 and N3" in that
directory's `README.md`). Weekly-performance-pass class: no render-thread change and no rendered
bit may move.

## Problem (verified on `main` at `d2fe0555a`)

A live C ABI edit (`commit_live`, `crates/capi/src/runtime/control.rs:1065`) does three kinds of
control-thread work whose cost grows with the session, not with the edit.

1. **Every track's effects are lowered twice, even when nothing in them changed (N2).**
   `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) calls `effect_records`
   (`:321`) for every track. `effect_records` lowers the track before and after
   (`current.lower_track(before)`, `next.lower_track(after)`, `:340-341`). Lowering builds a
   `Vec<Effect>` per console section and clones each slot's ID, identity and `params`
   (`crates/session/src/model.rs:440-460`). Only then does it skip instances whose `bypass` and
   `params` are equal (`:370-373`, `same_params` at `:505`). A fader-only edit on one track of a
   200-track session lowers all 200 tracks twice.
2. **Each effect instance's producer is found by a linear scan (N3).** `commit_live` finds it
   with `effects.iter().position(..)` over the whole producer table, per instance in the delta
   (`control.rs:1123-1128`).
3. **Each record's readback handle is found by a linear scan of the whole catalog (N3).**
   `commit_live` calls `SessionControlProvider::parameter_handle` per parameter record
   (`control.rs:1259-1265`). That is `parameter_metadata.iter().find(..)` over every row of every
   effect in the session (`crates/host-core/src/control_provider.rs:216-234`). An EQ alone has
   dozens of rows.

The strip lookup is already fine: it resumes a wrapping search where the last one stopped, and the
delta and the strip table share canonical track order (`control.rs:1094-1107`).

**The order both tables already have.** Both tables are built from the same
`EffectPreparedSession::entries`, which preparation sorts by `(track_id, rack, effect_id)`
(`crates/effect-compiler/src/prepare.rs:527-529`):

- the catalog: `SessionControlProvider::prepare_session(&effects.entries)`
  (`crates/host-core/src/prepare.rs:1382-1383`), whose builder appends each entry's rows
  contiguously, in entry order (`build_parameter_catalog`, `control_provider.rs:461`, loop at
  `:490-500`);
- the producers: `attach_effect_live_controls(&mut effects, depth)` (`prepare.rs:1390-1395`;
  `crates/effect-compiler/src/prepare.rs:1372`), which pushes one producer per entry in entry
  order, and returns an error, not a partial table, if any entry fails; capi keeps that order
  (`effect_controls.into_boxed_slice()`, `crates/capi/src/runtime/compile.rs:619`).

So both tables are sorted by `track_id` (byte order of the ID string). This is *not* the canonical
track order: `"t10"` sorts before `"t2"`. A strip's rows and producers form one contiguous run.

The verdict found all of this correct and fine at launch sizes. The classifier's whole-session
mask, clone and two canonical JSON strings (`live_delta.rs:228-262`) stay O(session) per
transaction; that is out of scope here.

## Decisions

- **D1. Skip unchanged tracks before lowering (N2).** In `effect_records`, before
  `lower_track`, return `Ok(())` when every console entry and every insert of `before` and `after`
  has bit-equal `params` (`same_params`) and equal `bypass`. Pair them by position, exactly as the
  step-3 mask does (`live_delta.rs:240-255`): the mask has already proved that everything else is
  equal. The per-instance check at `:370-373` stays.
- **D2. Producer lookup by strip run (N3).** Find a producer with `partition_point` on
  `track_id` over the producer table, then scan only that strip's run for the `address`. Put this
  in one small function in capi with a doc comment that names the sort it relies on (D4).
- **D3. Catalog lookup by strip run (N3).** Change `parameter_handle`'s body the same way:
  `partition_point` on `row.track_id`, then scan only that strip's rows for `rack`, `effect_id`,
  `parameter_id` and `channel`. Its signature, result and doc contract stay; update its doc
  comment ("A linear search over the catalog").
- **D4. State the order where it is made.** Add one sentence to the doc comments of
  `EffectPreparedSession::entries` (or the sort at `prepare.rs:527`), of
  `attach_effect_live_controls` and of `build_parameter_catalog`: the producer table and the
  catalog are in entry order, sorted by `track_id` first, and capi's lookups rely on it. Add a
  `debug_assert!` that the producer table is sorted by `track_id` where capi stores it (control
  thread, no allocation).
- **D5. No new retained memory.** No index, map or side table is added, so no resource row moves.
  If D2 or D3 cannot be done without one, stop and report; do not change resource rows here.

## Authorized paths

- `crates/host-core/src/live_delta.rs`: `effect_records` (D1) only.
- `crates/host-core/src/control_provider.rs`: `parameter_handle` and the `build_parameter_catalog`
  doc comment only.
- `crates/capi/src/runtime/control.rs`: the producer lookup in `commit_live` and its helper only.
- `crates/capi/src/runtime/compile.rs`: the D4 `debug_assert!` only.
- `crates/effect-compiler/src/prepare.rs`: D4 doc comments only.
- Tests: `crates/host-core/tests/live_delta.rs`, the `control_provider.rs` test module,
  `crates/capi/src/runtime/live_tests.rs`.
- This spec.

## Non-goals

- The classifier's whole-session mask and canonical-JSON comparison.
- `HostLiveControlHandles::effect_control_mut` (`crates/host-core/src/prepare.rs:476`), the
  browser's linear lookup. Record it in the evidence for a later pass.
- `build_parameter_catalog`'s per-row `find` over `initial_values` (`control_provider.rs:506-514`),
  which runs at preparation.
- A read-only provider accessor in `protocol` (the verdict noted `provider_mut()` for reads,
  `control.rs:1224`).
- Anything on the render thread.

## Hazards

- **Byte order, not canonical order.** Use the ID strings' `Ord`, the one the sort used. A search
  keyed on canonical track index or on strip position finds the wrong run.
- **Submix strips.** They are in both tables under their submix ID. The classifier does not emit
  live effect records for them today (`a_submix_effect_bypass_is_structural`,
  `submix_effect_params_are_structural`), but the lookups must not assume tracks only.
- **The console slot ID repeats on every strip.** Within one strip, `(rack, effect_id)` is unique;
  across strips it is not. Never search on `effect_id` before `track_id`.
- **D1's skip must still see a bypass-only change.** A skip on `params` alone loses the bypass
  record of #1266.

## Objective gates

1. **Catalog lookup equals the row (new host-core unit test).** Prepare a catalog for a session
   whose track IDs sort differently in byte order and in canonical order (for example `t2`, `t10`,
   `t1`), with a pre-insert and a post-insert console slot, two inserts with different effect IDs,
   one `PerLane` and one `Shared` parameter, and a submix strip with an insert. For every row,
   `parameter_handle(row.track_id, row.rack, row.effect_id, row.parameter_id, row.channel)` returns
   `row.handle`. A key with an unknown track, effect, parameter or channel returns `None`.
   *Test value: red if the search relies on an order the catalog does not have (canonical track
   order, or `effect_id` first), or stops at the wrong end of a strip's run, which the existing
   readback test (`the_parameter_readback_after_a_live_edit_equals_a_rebuilds`) misses because its
   fixture's track IDs (`eq` plus one digit, `crates/capi/src/runtime/tests.rs:134-155`) sort the
   same in both orders.*
2. **Producer lookup equals the table (new capi test).** On the same shape of session through
   the C ABI, for every producer in the newest epoch, the D2 lookup of
   `(producer.track_id, producer.address)` returns that producer's index; an unknown key returns
   `None`. *Test value: the same defect class as gate 1, on the producer table, which the existing
   live tests miss for the same reason.* If a C ABI end-to-end case is cheaper, a live edit on each
   of `t2`, `t10` and `t1` must render like the browser lane, as
   `live_effect_parameter_edits_render_like_the_browsers_lane` does.
3. **D1 keeps every record (mutation, PR evidence).** Make D1's skip ignore `bypass`:
   `a_live_bypass_flip_is_one_record_ahead_of_its_parameters` turns red. Make it ignore `params`:
   `a_live_parameter_change_is_one_record_on_its_lane` turns red. Revert both.
4. **No rendered bit moved.** Build `audit capi` at the base and at the head
   (`cargo build --locked --release -p audit -p capi`, then `target/release/audit capi`). Both
   report allocations, deallocations, locks, syscalls and `total_violations` 0, and the same
   `pcm_digest`. One-time PR evidence, not a committed pin.
5. **No resource row moved.** `cargo test --locked -p capi --test resource_lifecycle` passes, and
   its budget test's printed rows (`-- --nocapture`) equal the base's.
6. **Everything else.**
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-core --features control-provider,test-support`
   - `cargo test --locked -p effect-compiler --features test-support`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p host-core -p capi -p effect-compiler --all-targets --all-features
     -- -D warnings`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-capi-abi.sh` (no header change expected).
7. **Descriptive timing (PR evidence, not a gate, not committed).** Freeze one workload before
   timing: a one-track fader-plus-one-EQ-gain live edit through the C ABI on a generated session
   of 9 and of 256 tracks with one EQ console slot each. One invocation, one warmup, two measured
   rounds, at base and head, per `AGENTS.md`'s benchmark rules. Report the four numbers. Do not
   tune or retry.

## Evidence

- Gates 3, 4 and 7 as stated.
- The non-goal sites noted for a later pass.

## Dependencies

- None. #1264, #1265 and #1266 are on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- No allocation, lock or unbounded work enters the render thread; this issue changes none of it.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
