# Fold only windows published since a tap's arm into the meter frame's gain reduction

DRAFT, not filed. Stream B of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). Successor to
*Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345), from root's
finding of 2026-10-12 on #1345 Amendment 6. Code anchors verified on `codex/d15-stream-b3` at
`374d347c2`. Amendment 6 is not in that commit, so the contract-sentence anchors are not verified:
find them with the grep in "Deliverables". Re-verify every anchor before you start.

Proposed file name:
`NNNN-fold-only-windows-published-since-a-taps-arm-into-the-meter-frames-gain-reduction.md`.

## Product outcome

An app shows a gain-reduction meter from the browser meter frame (`trackGrDb`, `submixGrDb`,
`masterGrDb`). When the app unsubscribes a tap and subscribes it again, or re-subscribes an armed
tap (the SDK's `update()`), the meter shows only reduction that the effect measured after that
subscribe. Today the first frames after the re-subscribe can show the last window from before the
unsubscribe, an old value that the app reads as the new subscription's value. After this issue the
slot reads `0` (and `masterGrDb` reads `null`) until the re-armed tap closes its first new window.
The meter frame then agrees with the selected read, which already reports `Pending` in that
interval.

## Context

- **The defect.** `poll_meters` (`hosts/host-web/src/lib.rs:3485`) rebuilds the gain-reduction
  section at `:3722-3787`. For each armed tap it takes `reader.read()` (`:3763`), acknowledges it
  (`:3766`) and folds it into the strip's slot (`:3778-3782`), `observation_present` (`:3783-3785`)
  and `observed_any` (`:3786`). It does not compare the window with the tap's arm sample.
- **The check that exists.** `read_observation_values` (`:6072-6126`) keeps a window only when
  `window.first_sample >= arm_sample` (`:6086-6098`), where `arm_sample` is
  `ready.observation_arm_samples[effect][tap_index]`, `0` when absent. A stale cell therefore reads
  `Pending`.
- **Where the arm sample comes from.** Admission of an `Observe` record sets the armed bit and
  writes `observation_arm_samples[effect][tap] = applied_at_sample` for an arm and for a disarm
  (`:1862-1891`, the write at `:1884-1888`). `applied_at_sample` is
  `status.next_absolute_sample` at the submit (`:3392`). The field doc is at `:1654-1658`.
- **Why the stale window is there.** An observation cell is conflating: it keeps its last window
  for ever (`crates/engine/src/realtime/observe.rs:1-29`; `read` at `:169-189`). A disarmed tap
  stops publishing but its cell still holds the last window.
- **Where a new window starts.** `arm` (`crates/effect-contract/src/live.rs:1251-1291`) opens a
  fresh window at the first sample of the block that drained the record, which is the subscribe's
  `applied_at_sample`. A re-arm of an armed tap also restarts the window. So the first window
  after a (re)arm has `first_sample == applied_at_sample`, and an older window has
  `first_sample < applied_at_sample`.
- **Realtime.** `poll_meters` is called from the worklet's `process()` after the render export.
  It must allocate nothing, take no lock, make no syscall and use no runtime-checked `[]` index
  (its own comment, `:3493-3498`; the shipped artifact's call-graph gate covers it).
- **Tests that exist** (`hosts/host-web/src/tests.rs`).
  - `observation_host` (`:5606`), `feed_and_render_tracks` (`:6571`), `observe` (`:6602`),
    `gain_reduction` (`:6633`, returns the track slots and `masterGrDb`), `strip_gain_word`
    (`:10835`).
  - `selected_observation_reads_are_bounded_stable_and_non_consuming` (`:6302`) proves the
    selected read's re-arm check (`:6362-6371`). Nothing proves it for the meter frame.
  - `the_meter_frame_carries_the_app_shaped_gain_reduction` (`:6654`) and
    `a_bus_compressor_reports_its_gain_reduction_in_the_bus_word` (`:10852`; its disarm leg is
    `:10918-10939`) cover arm and disarm. They never re-subscribe.
  - Reading a tap's cell from a test: `resolve_observation` plus
    `ready.effect_observations[effect].readers[tap].read()` (pattern at `:6466-6477`).
- **The contract text.** #1345 Amendment 6 adds this sentence group to both declaration twins
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `sdk/src/browser/shipped-host.d.ts`)
  and to the `lib.rs` docs: "The gain-reduction section is recomputed from the taps armed when each
  frame is polled. An unsubscribe disarms its tap when it is admitted, so every meter frame
  received after its `result === 0` acknowledgement leaves that tap out. A strip's slot
  (`trackGrDb`, `submixGrDb`) is the maximum over its still-armed taps' latest windows, and `0`
  when none is armed or none has published. `masterGrDb` is `null` when the designated strip has
  none." On the current code, "latest windows" includes a window from before a re-arm.

## Decisions frozen

- **D1. The predicate.** In `poll_meters`, a window read from an armed tap folds only when
  `window.first_sample >= arm_sample`. `arm_sample` is read exactly as `read_observation_values`
  reads it: `ready.observation_arm_samples.get(effect).and_then(|s| s.get(tap_index)).copied()
  .unwrap_or(0)`. Use `get`, never `[]`.
- **D2. Which window counts.** After any arm (a subscribe on a disarmed tap, or a subscribe that
  re-arms an armed tap), only a window with `first_sample >=` that subscribe's `applied_at_sample`
  counts. The first such window is the one `arm` opened (`first_sample == applied_at_sample`).
- **D3. No window since the arm.** A window that fails D1 is treated exactly as no window: it does
  not write the strip's slot (the slot stays `0`), does not set `observation_present` (so
  `masterGrDb` is `null` when the designated strip has no other fresh tap), and does not set
  `observed_any` (so `METER_VALID_GAIN_REDUCTION` is clear when no tap folded).
- **D4. Acknowledge.** A window that fails D1 is not acknowledged. `reader.acknowledge` runs only
  for a window that folds. (`acknowledge` is monotonic, so the first fresh window's acknowledge
  covers the stale sequence.)
- **D5. One shared predicate (root ruling, 2026-10-12).** The rule "a window counts only if
  `first_sample >= arm_sample`" lives in one private helper that both `poll_meters` and
  `read_observation_values` call, so the two readers cannot drift apart again. In
  `read_observation_values` the change is the call only: its behaviour does not move, and its
  existing tests stay green unchanged.
- **D5a. Re-arm (root ruling, 2026-10-12).** On the `update()` re-arm path the frame leaves the
  tap out (slot `0`, `masterGrDb` `null` when no other fresh tap) for up to one new window. This
  is the same state as the selected read's Pending, and it is accepted: an old window describes
  the arm before the change, and the frame must not show it.
- **D6. The contract sentence.** In every place Amendment 6 put it, replace the sentence group with
  this text (keep each file's comment markers and line wrapping; the two `.d.ts` files stay
  byte-identical):

  > The gain-reduction section is recomputed from the taps armed when each frame is polled. An
  > unsubscribe disarms its tap when it is admitted, so every meter frame received after its
  > `result === 0` acknowledgement leaves that tap out. A subscribe, also one that re-arms an armed
  > tap, restarts its tap's window at the acknowledgement's `appliedAtSample`, and the frame leaves
  > that tap out until a window that opens at or after that sample closes. A strip's slot
  > (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at
  > or after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is
  > `null` when the designated strip has none.

## Deliverables

1. The D1-D4 check in `poll_meters`' tap loop (`lib.rs:3756-3787`), through the D5 helper, which
   `read_observation_values` (`lib.rs:6086-6098`) also calls.
2. The D6 text. Find every place with
   `grep -n "recomputed from the taps armed" hosts/host-web/src/lib.rs hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts sdk/src/browser/shipped-host.d.ts`.
   If Amendment 6's wording landed differently, replace the group it landed as.
3. One new test in `hosts/host-web/src/tests.rs`, written first (gate 1), after
   `the_meter_frame_carries_the_app_shaped_gain_reduction`.

## Authorized paths

- `hosts/host-web/src/lib.rs`: the body and comments of `poll_meters`, the new D5 helper, the
  call to it in `read_observation_values` (no other change there), and the doc lines that carry
  the D6 text.
- `hosts/host-web/src/tests.rs`: the new test only; no helper changes.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and
  `sdk/src/browser/shipped-host.d.ts`: the D6 text only.
- This issue's own spec file.

## Non-goals

- `read_observation_values`, the admission path (`:1862-1891`) and the field docs.
- Plan replacement (`disarm_all`, new plan cells), the C ABI (it has no observation path), the SDK
  and the worklet JavaScript.
- Any change to the peak section, the header timestamps or the meter delivery policy.

## Objective gates

1. **Red-first test** `a_resubscribed_tap_leaves_the_meter_frame_until_its_first_new_window`.
   Write it before the fix, run it on the unchanged code, and record it red in the attempt record
   (the failing assertion and the stale value). Then apply the fix and record it green.
   1. `host = observation_host(128, 2, Some(0))`; `host.set_meter_lease(true) == RESULT_OK`;
      `observe(&mut host, 0, 1, 0, 1, 2, true) == RESULT_OK` (track 0's compressor, tap 1,
      window 2).
   2. Render 32 blocks with `feed_and_render_tracks(.., 0.5)`, then
      `while host.poll_meters() != 0 {}`. Assert `gain_reduction(&host).0[0] > 0.0` (the stale
      value `R` that the defect leaks).
   3. Unsubscribe: `observe(.., false) == RESULT_OK`. Render 2 blocks of `0.5` (a render must
      drain the record before the next one for this tap is admitted, #1345 Amendment 3), polling
      every frame. Every frame has track 0's slot bits `0` and `masterGrDb` `None`.
   4. Assert that the tap's cell still holds a window (`read()` is `Some`). Re-subscribe:
      `observe(&mut host, 0, 1, 0, 1, 8, true) == RESULT_OK`; `a = command_report().applied_at_sample`.
      Assert the cell's window has `first_sample < a` (a stale window exists to leak).
   5. Render up to 12 blocks of `0.5`, one at a time. After each block, read the cell (`fresh` is
      `read().is_some_and(|w| w.first_sample >= a)`), then poll every frame
      (`while host.poll_meters() != 0`). For each frame:
      - if not `fresh`: track 0's slot bits are `0` and `masterGrDb` is `None`;
      - if `fresh`: track 0's slot is `> 0.0` and `masterGrDb == Some(slot)`.
      At least one frame of each kind must occur. The first fresh window has
      `first_sample == a`.
   6. Leg 2, re-arm without an unsubscribe (the SDK `update()` path): with the tap fresh, send
      `observe(&mut host, 0, 1, 0, 1, 4, true) == RESULT_OK`, take `a2`, and repeat step 5 with
      `a2` and up to 8 blocks.
   - *Test value:* red if the meter frame folds a tap's window from before its latest subscribe,
     so a re-subscribed (or re-armed) tap shows its old reduction as new, or if the fold drops the
     first window the subscribe opened. No existing test re-subscribes a tap and then reads the
     meter frame.
   - *Mutations (PR evidence; record each red with the failing step):* M1, the bug's revert
     (remove the D1 check): red at step 5. M2, `>=` changed to `>`: red at step 5 (the first fresh
     window has `first_sample == a`, so no frame is fresh). M3, a local edit outside the authorized
     paths that writes the arm sample only on a disarmed-to-armed change (`lib.rs:1884-1888`): red
     at step 6 only.
2. **Twins.** `bash scripts/check-sdk-generated.sh` (the two `.d.ts` files are byte-identical).
3. **Realtime and worklet.** Same commands as `qualification.yml`:
   - `bash scripts/check-realtime-policy.sh`;
   - `bash scripts/build-web-audioworklet.sh --named-twin <empty dir T> <empty dir A>`, then
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration A T/miso-engine-v1-audio-worklet.simd128.named.wasm`
     (export set, call graph: no new panic or allocation path from `process()`);
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts A`;
   - `bash scripts/test-web-audioworklet.sh`.
4. **Workspace.**
   - `cargo test --locked -p host-web --lib a_resubscribed_tap_leaves_the_meter_frame_until_its_first_new_window`
   - `cargo test --locked -p host-web --all-features`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p host-web --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh`

## Dependencies

- #1345 on `main`, with Amendment 6's contract text (this issue rewrites that text). The code fix
  alone needs only `observation_arm_samples`, which is on `codex/d15-stream-b3` at `374d347c2`.
- H #1448 (`poll_meters`' bounded delivery) has landed; this issue edits only the gain-reduction
  loop.

## Placement (root edits `STREAMS.md`)

- Stream B, after #1345, before H #1332, in the next B batch (root ruling, 2026-10-12: not in
  `codex/d15-stream-b3`). Contract-sentence anchors are found by grep at the commit that holds
  #1345 Amendment 6.
- `hosts/host-web/src/lib.rs` row (`STREAMS.md:101`): add it to the B slices that land before
  H #1332, in any order with #1523, #1524 and #1525; the later slice rebases.
- `hosts/host-web/src/tests.rs` row (`:107`): in any order with the rest; before H #1332.
- `.d.ts` twins row (`:108`): B (this issue, the D6 text only) before H #1332 and C #1406.
- Stream B table: a new row after #1525, dependency #1345, "before H #1332".

## Attempt record

- Attempt 1: not started.
