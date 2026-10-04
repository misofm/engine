Verdict: PASS

# #1266 attempt 1 -- adversarial verdict (Sol)

Commit under review: `5c8de7880` (parent `b055a48d4`), branch `codex/1053-live-updates`.
Reviewed from the export `/tmp/claude-1002/v1266-a1` (deleted after review); logs kept in
`/tmp/claude-1002/v1266-a1-logs/`.
Held against: the slice spec, umbrella #1053 D6-D9, D14 and G1, decision 14 (F4, F7), and
`AGENTS.md`'s realtime rules and test-value rule.

No BLOCKER and no MAJOR. The classifier, the capi push and the docs do what D1-D3 ask. Every
gate I re-ran is green. The two mutations I re-ran (M3, M5) go red and then green again. The
findings below are about missing tests and wording.

## Answers to the brief's questions

**(a) Is the bypass latency-preserving? Is gate 2's window correct?** Yes to both.
- The record drives the rack's `BypassShunt` (`crates/effect-contract/src/live.rs:820-985`).
- The shunt's latency line is fed on every block whenever the effect declares latency
  (`feeds_line`; rack `crates/rack/src/lib.rs:1341-1350`; per-node `graph/src/runtime.rs:3537`).
  The wet path always runs.
- A lowered effect is prepared with `bypass = false` (`effect-compiler/src/prepare.rs:450-455`).
  So a rebuild with the new bypass gets the same latency, PDC, tail, bank cohort and shunt
  allocation as the running plan. On the C ABI every instance has a live channel, so the shunt
  is charged either way (`graph-compiler/src/estimate.rs:294`).
- State continuity neither fakes nor hides equality for the switched instance. The oracle is a
  plan fed from sample 0 with the committed bypass, and its wet path also ran from sample 0. So
  the compressor's envelope is the same bits in both, and nothing is restarted.
- The lift half of gate 2 is what tests the continued wet path. Its compared blocks are wet
  output, compared against a reference whose compressor never paused.
- I measured the window with a scratch probe (not committed). Live equals the oracle from exactly
  E + ceil(latency/quantum) on:
  - 1 track: latency 31, equal from E+1;
  - 10 tracks at 44.1 kHz: latency 478, equal from E+4;
  - 10 tracks at 96 kHz: latency 997, equal from E+8.

  The blocks before that are the upstream switch passing through downstream latency lines (the
  soft-clip's and PDC). So K, which adds one block of slack, is correct and hides no lasting
  difference.
- One exception, which affects the wording only (MINOR 2): the oracle holds only because nothing
  downstream of the switched instance keeps state longer than the window.

**(b) Is `lowers_session_bypass` the right predicate?** Yes.
- It is the same function preparation uses to decide whether a session bypass is baked into the
  effect (`prepare.rs:450-455`, `:518-520`) and how the lane is seeded (`:1431-1438`). The
  classifier and preparation cannot drift apart.
- I went through every launch factory in `launch_native_effect_registry` (`prepare.rs:202-213`):
  EQ, compressor, gate/expander, multiband, true-peak limiter, soft-clip, transient shaper and
  delay. Only `miso.delay` (`NEVER_BANKED_EFFECTS`) and `miso.multiband-compressor`
  (`PREPARED_BYPASS_EFFECTS`) keep a prepared bypass.
- Every other read of `metadata.bypass` sees the prepared value, which is `false` for a lowered
  effect: compressor `silent_bypass`, gate and transient-shaper kernels, the EQ response, and the
  graph response snapshot (`graph/src/runtime.rs:1508`, not exposed on the C ABI). So live and
  rebuild agree.
- No other place in host-core, graph-compiler or builtins-compiler reads the session bypass.

**(c) The acked-batch question.** I found no other push or publish after the predicate that can
fail.
- **Preflight.** `preflight(Bypass)` passes on an owner producer, because only `PreparedTarget`,
  and `Parameter` when the producer needs targets, are refused (`prepare.rs:712-720`).
- **Room.** Step 4 checks `bypass + targets <= available`, and a render pop only grows the room.
  So both the bypass push and `owner.publish` succeed. `publish` re-runs `preflight_publication`
  against an open candidate, with room of at least the targets (`effect-compiler/src/control.rs:352-404`).
- **Over-capacity rebuild.** It runs before any owner is begun, and it counts
  `records.len()` (bypass included) or `bypass + targets`. So an edit that could never fit
  rebuilds, never an endless `BACKPRESSURE`. That code is correct but untested for the EQ (MINOR 1).

**(d) The order of an EQ's bypass and its targets (M6).** The order does not matter, so it is
not a real contract.
- Records the render drains at one block entry all apply at that boundary.
- A render that races the push can split a transaction across two blocks. D2 already allows
  that, in either order.
- The better order would depend on the direction anyway:
  - bypassing: the bypass before the new targets hides the new gain for that block;
  - lifting: the targets before the lift avoid one block of the old gain.

  A fixed "ahead of" is therefore not a meaningful promise (NIT 1).

**(e) Readback (D9).** It still follows the committed model.
- The committed bypass equals the prepared `initial_bypass` plus every `Bypass` record pushed
  since.
- `SessionSnapshotGet` returns the committed model. Gate 2 relies on this: it renders
  `rig.snapshot()` as its oracle, so a stale snapshot bypass would turn it red.
- The protocol has no bypass readback. `SetEffectBypass` edits only the model
  (`protocol/src/model.rs:663-665`) and the provider catalog carries no bypass row, so no row
  needs an update.

**(f) G1.** A submix strip's bypass stays structural. The mask loops over tracks only
(`live_delta.rs:237-254`), and `a_submix_effect_bypass_is_structural` holds it (M8).

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

1. **The EQ bypass term in the over-capacity rebuild check is untested.**
   - Where: `crates/capi/src/runtime/control.rs:1102-1105`.
   - The mutation (my M9): `Some(targets) if producer.has_owner() => targets.len()` (drop
     `bypass_records`). It survives the whole capi suite (62 + 13 passed).
   - What the defect does: an EQ bypass beside an edit whose targets exactly fill the queue
     always returns `BACKPRESSURE` and never rebuilds. That is the endless `BACKPRESSURE` that
     #1264 D3, #1265 D2 and #1053 D14 forbid.
   - Fix: add the boundary case. A ready test is in
     `/home/bl/misofm/submix-verdicts/1266-attempt1-verifier-scratch.rs`. It uses the EQ fixture
     at `maximum_automation_spans_per_block: 4`, `edits(3)` (four targets) plus
     `bypass_edit("eq0", Console, "eq", true)`, and asserts `RESULT_OK` with one candidate
     pending and room 4.
   - I ran it: red under M9 (result 6, `BACKPRESSURE`), green on `5c8de7880`. It can also be
     folded into `an_eq_edit_designing_more_targets_than_its_queue_rebuilds` (`live_tests.rs:2206`).
   - Record it as a mutation in the attempt record.

2. **"Equal to a rebuild" is stated more generally than it holds.**
   - Where: `docs/C_ABI_V1_QUALIFICATION.md:396-399`, and the gate 2 rationale in the spec
     (`1266-...md:72-76`).
   - The claim says the equality holds "because the wet path keeps running". That explains the
     switched instance's own state, but not the state of a stateful effect downstream of it,
     which keeps the live history (as after any live edit).
   - My probe: add a second compressor insert (`comp2`) after `comp` (1 track, 48 kHz), then
     bypass `comp` live. Live never matches the from-sample-0 oracle within 42 blocks, in either
     direction. The largest difference after the bypass is 3.0e-4, and it decays.
   - The gate is valid on its fixture only because the only thing downstream of the switched
     instances is the soft-clip, whose memory is about 31 samples.
   - The behaviour is correct (a real rebuild resets all state anyway). The claim is not.
   - Fix (wording only): scope the bullet. For example: "the switched instance's own state is
     the same either way; an effect downstream of it keeps the live history, so the
     from-sample-0 oracle holds only while the downstream memory fits the window, as on the gate
     fixture (a compressor insert, a soft-clip console slot)". Note the same in the attempt
     record's gate-2 test value.

3. **The public C header and `docs/CONTROL_PROTOCOL_SEMANTICS.md` are now wrong about bypass.**
   - Where: `crates/capi/include/miso_engine_v1.h:34-41`, and the #1257 paragraph of
     `docs/CONTROL_PROTOCOL_SEMANTICS.md`.
   - Both still say that only fader, mute and pan/matrix (plus model-only edits) avoid a
     replacement, and that "every other field" rebuilds. After this commit an effect bypass is
     live too (except the delay's and the multiband's).
   - This is outside #1266's authorized paths and is already on the batch ledger as 1264 M1. So
     it is not charged to this attempt.
   - Fix: extend that follow-up to name live bypass and its two exceptions.

### NIT

1. **The documented order promises nothing anyone can observe (M6).**
   - Where: `docs/C_ABI_V1_QUALIFICATION.md:383-384` ("ahead of the instance's parameter records
     or, for a parametric EQ, ahead of its targets") and `control.rs:1024-1026`.
   - Per (d), the order is unobservable within a block, and D2 allows either order across a
     racing block.
   - Fix: say the bypass record "rides with" the instance's parameter records or targets, under
     D2's one-quantum allowance. Or keep the classifier's order (a deterministic output, pinned by
     M4) and drop the host-facing promise.
2. **Wrong issue number.** `crates/host-core/tests/live_delta.rs:1464` says a submix effect's
   bypass "stays structural until #1225". Umbrella G1 gives submix effects to #1267.
3. **Line too long.** `docs/C_ABI_V1_QUALIFICATION.md:330` is 105 columns; rewrap it.
4. **The audit does not cover bypass.** The `tools/audit/src/capi.rs` live editor drives only
   `SetTrackFader` and `SetTrackMatrixOrPan`, so the 0-allocation and 0-syscall audit never
   renders a C ABI lane switched by a live `Bypass` record. Extend ledger item 1265 M2 (effect
   parameter and EQ edits) to include a bypass toggle.
5. **No capi PCM test for a bypass-only EQ edit.** The path with `targets: None` on an owner
   producer is tested only at the classifier
   (`a_live_bypass_flip_is_one_record_ahead_of_its_parameters`). It reuses the compressor's push
   path and preflight, so this is optional.

## Test value (one sentence per new or rewritten test)

- **`a_live_bypass_flip_is_one_record_ahead_of_its_parameters`** (host-core): red if the step-3
  mask leaves out a track entry's or insert's bypass (the flip turns into `Structure`), or if the
  flip is not exactly one `Bypass(post)` at the instance's address before its parameter records
  (M2, M3, M4).
- **`an_eq_bypass_beside_a_gain_change_rides_ahead_of_its_targets`** (host-core): red if an EQ's
  `Bypass` record is fed to the target designer (which rebuilds on a non-`Parameter` record) or
  changes the targets designed from its parameter edits.
- **`a_prepared_bypass_change_needs_a_rebuild`** (host-core): red if a delay or multiband
  bypass flip, either way, is classified live, which would be an acked edit that is never heard
  (M1).
- **`a_submix_effect_bypass_is_structural`** (host-core): red if the bypass mask is extended to
  submix strips (M8, G1).
- **`an_insert_reorder_is_structural`** (rewritten, host-core): it lost only the
  bypass-beside-parameter assertion. That case is now live and moved into the first test, which
  is the right handling of a superseded assertion. The remaining reorder case is unchanged from
  #1264.
- **`live_effect_bypass_edits_render_like_a_rebuild`** (capi, gate 2): red if a live `Bypass`
  record carries anything other than what preparation bakes for the committed bypass, or lands
  on another instance or bank lane. I confirmed M3 red at `live_tests.rs:2382`.
- **`a_live_eq_bypass_beside_a_gain_change_renders_like_the_browsers_lane`** (capi): red if capi
  drops an EQ's `Bypass` record because the instance has targets (M7), or treats it as an owner
  edit. M3 also turns it red.
- **`an_eq_bypass_and_targets_without_room_for_both_refuse_before_anything_changes`** (capi):
  red if the step-4 room check leaves out the EQ's `Bypass` record. The transaction is then
  admitted and the publication panics after the bypass push. I confirmed M5 red, with a panic at
  `control.rs:1301`.
- **`a_prepared_bypass_change_rebuilds_and_renders_the_committed_model`** (capi, gate 3): red if
  the C ABI pushes a multiband bypass change live instead of preparing a candidate (M1).

No test greps source or prose, and no test adds a digest or byte pin.

## Gates I re-ran (from the export, `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053`)

**Tests**
- `cargo test --locked -p host-core --all-features`: pass. `--test live_delta`: 29 passed.
- `cargo test --locked -p capi`: pass (62 + 13 passed, 0 failed).

**Release, audit and ABI**
- `cargo build --locked --release -p audit -p capi`, then `audit capi`: calls 100000,
  allocations 0, deallocations 0, locks 0, syscalls 0, total_violations 0.
- `scripts/check-capi-abi.sh`, against the four-package build: ok (shared and static linkage).
- `check-capi-abi.sh` again, letting it build `-p capi` alone (the shipped feature set, as in
  CI): ok.
- `scripts/check-scalar-oracle-absent.py --native libcapi.so`:
  - on the shipped `-p capi` build: pass;
  - on the audit-unified build: fails, because audit's `graph/test-support` is unified in. CI
    runs this check only after the `-p capi`-alone build (`qualification.yml:817-821`), so that
    failure is expected and is not a finding.

**Lint and policy**
- `cargo clippy --locked -p host-core -p capi --all-targets --all-features -- -D warnings`: pass.
- `cargo fmt --all -- --check`: pass.
- `check-{host-core,realtime,workspace,protocol-control}-policy.sh`: pass.
- `test-{host-core,realtime,protocol-control}-policy.sh`: pass.

**Mutations, each applied in the export and then reverted (byte-compared to the commit)**
- M3 (`Bypass(!bypass)`): capi gate 2 and the EQ render test red; host-core 2 failed.
- M5 (room check counts the targets only): the room test red, with a publish panic.
- M9 (mine; over-capacity check drops the EQ's bypass): survives; see MINOR 1.

**Scratch probes (not committed)**
- The gate-2 window measurement, under (a).
- The downstream-compressor probe, under MINOR 2.

**Authorized paths:** only the six authorized paths changed. `control.rs` was needed, because
without it a `Bypass` record would hit the `_ => Internal` arm.

**Not re-run here:**
- the #1257 gate-6 workspace test command;
- `cargo test -p protocol`;
- `cargo doc`;
- `check-cross-targets.sh`;
- the worklet chain. The implementer's identical module hash is consistent with my reading:
  `host-web` never references `classify_live_delta`.
- 4-lane (NEON) is CI-only.
