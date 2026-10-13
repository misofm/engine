# Prepare every browser preparation branch concurrently, as a successor too, in host-core

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11).
Split from *Replace the running browser session in the Rust host* (#1290): its two spectrum
successor wrappers. Code anchors verified on `main` at `6fb211594`.

## Product outcome

host-core can prepare every plan shape the browser boots, with concurrent control delivery, both
at boot and as a successor of a running plan: one spectrum capture, a spectrum collection, and
the browser's selected per-strip meters with no spectrum. A successor prepared through any of
them keeps every unchanged source's ring, exactly as the C ABI's successor does today. This is
what lets the browser's control plane prepare in the Worker while the worklet renders.

## Context

- `compile_ready` (`hosts/host-web/src/lib.rs:6617`) prepares through one of three host-core
  entries (`:6644-6684`):
  - `prepare_host_runtime_with_live_controls_and_spectrum`
    (`crates/host-core/src/prepare.rs:843`);
  - `prepare_host_runtime_with_live_controls_and_spectrum_collection` (`:870`);
  - `prepare_host_runtime_with_selected_meters_between_render_calls` (`:1051`), with one
    `PostMatrix` sample-peak meter per strip (`lib.rs:6626-6643`).
- All three funnel into `prepare_host_runtime_with_live_controls_policy_and_spectrum` (`:1136`),
  which already takes `spectrum_request`, `between_render_calls` and
  `successor: Option<SuccessorBase>` (`:1143-1144`). The three entries pass `successor: None`.
- The two spectrum entries pass `between_render_calls: false` (concurrent). The selected-meter
  entry passes `true`, and the policy routes every selected-meter request to
  `prepare_selected_session_builtins_between_render_calls` whatever that flag says
  (`prepare.rs:1503-1509`). builtins-compiler already has the concurrent form,
  `prepare_selected_session_builtins_with_live_controls`
  (`crates/builtins-compiler/src/lib.rs:3355`).
- The successor entries that exist are `prepare_host_runtime_with_live_controls_successor`
  (`prepare.rs:926`) and `prepare_host_runtime_with_live_lanes_successor` (`:818`). Neither takes a
  spectrum request or selected meters. `SuccessorBase` is at `:641`.
- `BetweenRenderCalls` fuses each strip's fader and matrix banks; concurrent delivery does not
  (`crates/builtins-compiler/src/lib.rs:1152-1169`; witness `test_only_fader_matrix_witness`, used
  at `crates/host-core/tests/live_lanes.rs:216-231`).
- *Give every browser plan live strip fader and mute lanes* (#1326) D3 gives the three browser
  entries a trailing `lanes: HostLiveLanes` argument.
- The successor tests and their helpers are `crates/host-core/tests/successor_swap.rs` and
  `crates/host-core/tests/support/successor.rs`.

## Decisions frozen for this slice

- **D1. Two spectrum successor entries**, beside `prepare_host_runtime_with_live_controls_successor`,
  each the boot entry with `successor: Some(base)` as the last argument:
  - `prepare_host_runtime_with_live_controls_and_spectrum_successor(compiled, caps,
    live_controls, spectrum, lanes, base)`;
  - `prepare_host_runtime_with_live_controls_and_spectrum_collection_successor(compiled, caps,
    live_controls, spectrum, lanes, base)`.

  Both are concurrent. They return what their boot entries return.
- **D2. The selected-meter branch, concurrent.** The browser's third branch needs the same
  successor, and it has no concurrent entry at all. This slice adds it, because the browser
  preparer (#1400) cannot prepare in the Worker without it:
  - `prepare_host_runtime_with_selected_meters(compiled, caps, live_controls, meters, lanes)` and
    `prepare_host_runtime_with_selected_meters_successor(.., lanes, base)`, both concurrent;
  - the policy routes a selected-meter request with `between_render_calls == false` to
    `prepare_selected_session_builtins_with_live_controls`. The `_between_render_calls` entry
    keeps its behaviour and its callers.
- **D3. One body.** All four new entries call
  `prepare_host_runtime_with_live_controls_policy_and_spectrum`. No preparation logic is
  duplicated.
- **D4. Spectrum on a successor.** The successor gets a fresh capture, sized and validated as at
  boot, and allocated on the control thread. Carrying capture state across the swap is *Carry
  spectrum capture state across a plan swap* (#1395). A spectrum target missing from the successor
  is refused with the boot entry's diagnostic, `host.spectrum.target`
  (`crates/host-core/tests/spectrum.rs:139`).
- **D5. No caller changes.** host-web keeps calling today's entries. *Prepare through an
  adapter-supplied preparer in the control-plane crate* (#1400) switches the browser to these.

## Deliverables

1. D1-D3 in `crates/host-core/src/prepare.rs`, re-exported from `crates/host-core/src/lib.rs`.
2. The integration test binary `crates/host-core/tests/browser_successor.rs`.

## Authorized paths

- `crates/host-core/src/prepare.rs` (the four entries and the D2 routing only),
  `crates/host-core/src/lib.rs` (re-exports), `crates/host-core/tests/browser_successor.rs` (new),
  `crates/host-core/tests/support/successor.rs` (a spectrum-aware helper only)
- `prepare.rs` is stream A's file; root merges this slice after stream A's open edits to it.

## Non-goals

- No host-web change (#1400). No spectrum carry (#1395). No meter carry (*Carry meter and effect
  observation state across a plan swap*, #1327).
- No change to any existing entry's behaviour, including the `_between_render_calls` ones.

## Hazards

- **Fusion and bits.** D2's concurrent selected-meter path forms no fused fader-matrix bank.
  AGENTS.md says regrouping lanes never moves a bit. If gate 3 goes red, stop and report it to
  root; do not loosen the comparison.

## Objective gates

1. **Gap-free successor, each branch.** In `browser_successor.rs`, for each of the three
   successor entries (single capture on track `eq0`, a two-target collection, selected meters):
   prepare session A, render 6 blocks, prepare B (A plus a muted track) as its successor, move
   the producers with `adopt_persisting`, swap through the plan exchange and render 6 more. The swap block reports `CarryOutcome::Carried`.
   Every block equals B prepared fresh through the matching boot entry and fed the same PCM from
   frame 0. Session A is the console-free shape of `successor_swap.rs`'s `session_a()`
   (`crates/host-core/tests/successor_swap.rs:46-67`): the `parametric-eq-nine-track` fixture cut
   to tracks `eq0` and `eq1` and their main routes, both session console sections and every
   track's console and inserts cleared, input filters off. `browser_successor.rs` builds it
   itself. The full fixture has a console EQ slot, whose state carries only after *Carry console
   effect lanes across a plan swap* (#1279), so a successor of it restarts that state and the gate
   would fail for a reason this slice does not own.
2. **The successor's capture works.** After the swap in gate 1's single-capture case, arming the
   successor's capture and rendering one window returns a completed window for the target.
3. **Concurrent, and the same bits.** The selected-meter boot entry forms no fused bank
   (`test_only_fader_matrix_witness().factory_calls == 0`), and its output over 64 blocks is
   bit-identical to `prepare_host_runtime_with_selected_meters_between_render_calls` on the same
   session and feed, with no record submitted.
4. **Refusal.** A successor whose spectrum target track is removed is refused with
   `host.spectrum.target`, and the predecessor's producers still feed it.
5. **Realtime.** In gate 1's single-capture case, every block after warm-up, the swap block
   included, reads `allocations == 0 && frees == 0` from `bench_support::alloc`'s thread-scoped
   counters (as `successor_swap.rs`'s `the_swap_block_allocates_and_frees_nothing` does).
6. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p host-web --features host-web/test-support` (unchanged callers)
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-cross-targets.sh`
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: turns red if a successor entry ignores its base (every ring restarts, the stem jumps),
  or prepares a different graph than its boot entry.
- Gate 2: turns red if the successor's capture is not wired to its plan's graph taps.
- Gate 3: turns red if the selected-meter path stays `BetweenRenderCalls` (the Worker would then
  push records while the worklet renders under a contract that forbids it), or if dropping the
  fusion moves a bit.
- Gate 4: turns red if the refusal runs after the producers moved.
- Gate 5: turns red if the swap block of a spectrum plan allocates or frees on render.

## Dependencies

- *Give every browser plan live strip fader and mute lanes* (#1326): the `lanes` argument.

Dependents: *Prepare through an adapter-supplied preparer in the control-plane crate* (#1400) and
*Replace the running browser session in the Rust host* (#1290).

**Decision 17 amendment (2026-10-13).** D4's refusal (`host.spectrum.target`) and gate 4 stay as host-core's invariant check. From #1593 on, the control plane never passes a missing target: a structural edit that removes a spectrum target's strip or output drops that entry from the session's spectrum configuration in the same commit, before the successor is prepared, so the edit is never refused for a spectrum target (#1593 D4).

## Decision record

Implemented on the batch 4 branch (decision 17 controls program). The spec predates decision 17 and
several landed slices; each adaptation of its intent to today's code:

- **A1. Signatures carry `superseded`.** D1 and D2 list `(compiled, caps, live_controls, spectrum,
  lanes, base)`. Since #1312 D2 every boot entry here takes the session's `live_values_superseded`
  count after `lanes`, so each new entry takes `(.., lanes, superseded, base)` (D2's selected-meter
  pair has `meters` before `lanes`). Each is its boot entry plus `base`, which is what D1 asks for.
- **A2. No `bound_cache`.** `prepare_host_runtime_with_live_lanes{,_successor}` take an
  `InputBoundCache`; the spectrum and selected-meter boot entries pass `None`, and
  `prepare_selected_session_builtins_with_live_controls` takes none. The new entries match their
  boot entries and take none. #1400 adds a cache argument to all of them together if it wants one.
- **A3. `lanes` exists already.** The dependency on #1326 is met (`HostLiveLanes` is an argument of
  every boot entry here). The cases pass `HostLiveLanes::ALL`; a caller passes what it chooses (the C
  ABI's choice is `controls::live_lanes`, decision 17 C6).
- **A4. Effective values.** The shared body now computes `controls::effective` once per preparation
  (decision 17 C7), before it picks a builtins form. The four new entries reach that code through the
  same body (D3), so they need no `effective` argument and no decision 17 change.
- **A5. D2's routing.** In the body, `selected_meters.is_some()` now picks
  `prepare_selected_session_builtins_between_render_calls` only when `between_render_calls` is true,
  else `prepare_selected_session_builtins_with_live_controls`. The one existing caller of the
  serialized selected entry passes `true`, so its behaviour and bits are unchanged.
- **A6. Test harness.** `support/successor.rs` gained the general form `swapped_run_with` (an
  already prepared predecessor and a closure that prepares the successor from its inventory),
  `reference_run_of`, and a `SwappedRun` that keeps the render owner and exchange halves so gate 2
  renders on past `BLOCKS`. `swapped_run` and `reference_run` delegate to them with unchanged
  behaviour (`successor_swap.rs` passes unchanged). That is wider than "a spectrum-aware helper
  only", but it is the one helper change that gates 1, 2 and 5 need.
- **A7. Gate 1 negative.** The test also runs each branch with a base-less successor and expects the
  first difference at the swap block, so the oracle can fail.
- **A8. Decision 17 amendment.** D4's refusal and gate 4 stay as host-core's invariant check, as the
  amendment says; the control plane never reaches it from #1593 on.
