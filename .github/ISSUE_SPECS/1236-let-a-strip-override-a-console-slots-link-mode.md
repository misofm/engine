# Let a strip override a console slot's link mode

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E4, D15-1, D15-7).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Any strip, track or submix, runs a console slot at its own detector link mode, as a DAW lets each
plugin instance choose stereo-linked or multi-mono, without splitting the slot's bank. Changing
that mode on a playing session is a live update: the detector glides from one link law to the
other over the edit's ramp or the session default, with no rebuild and no click. This is a small
umbrella. It records the owner's direction and the frozen design, and names its slices: #1368,
#1369, #1370 and #1371, plus the multiband follow-on #1367.

## Owner direction

Owner question Q1 of decision 13 (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, "Owner
questions", `:185`), answered on 2026-10-03: a strip's console entry may override the slot's
`link_mode`, following modern DAWs. This reverses the planner's recommendation in DESIGN 8.2 Q1
(`docs/handoffs/submix-sends-2026-10-02/DESIGN.md`). It is outside *Submix strips and live aux
sends* (#1196) and outside the VCA umbrella.

## Context

- **Declaration.** The link mode is declared once per console slot: `ConsoleSlot.link_mode`
  (`crates/session/src/model.rs:272-283`). A strip's `ConsoleEntry` carries only `slot`, `bypass`
  and `params` (`:285-293`). The parser refuses `link_mode` on an entry
  (`CONSOLE_ENTRY_EFFECT_FIELD`, `crates/session/src/parse.rs:904`, `:965-969`). Lowering copies the
  slot's mode into every strip's effect (`lower_section`, `model.rs:452-466`).
- **Bank key.** The prepared link mode is part of `EffectProgramKey`
  (`crates/effect-contract/src/lib.rs:1171-1186`, field `:1179`), and a rack chain's bank program
  is the list of its slots' keys (`crates/graph-compiler/src/banks.rs:186-194`). Two strips that
  ran one slot at different modes would form two cohorts: a split bank, where decision 12 promises
  that a console slot always banks as one.
- **The kernels already treat link as lane data, chosen per block.**
  - Compressor: `Invariants::new` splats `linked` and `averaged` masks from one `LinkMode`
    (`crates/compressor/src/kernel.rs:290-313`); `link_frame` selects per lane (`:316-339`). The
    cheaper `dual_mono` settled arm is chosen once per block (`:595`, `:610`), beside the `wet` arm
    that is already chosen per block from lane masks (`every_lane`, `:596-598`; #982).
  - Gate-expander: `link_max` and `link_avg` are already per-lane vector words
    (`crates/gate-expander/src/lib.rs:425-426`).
  - Transient shaper: the link law is a const generic (`link`, `crates/transient-shaper/src/lib.rs:286`),
    dispatched once per block from the prepared mode (`:494-498`).
  - True-peak limiter: `LimiterCoef.link_max` is one bool per bank
    (`crates/true-peak-limiter/src/lib.rs:408-416`), turned into a lane mask in the per-lane body
    (`:2741`). The linked-pair fast path (#990) is a per-block decision:
    `link_max && gain_linked && designed_gain_agree` (`:3588-3594`).
- **Console-eligible effects** (`CONSOLE_ELIGIBLE_EFFECTS`, `crates/effect-compiler/src/prepare.rs:223-230`)
  that support more than `dual_mono`: the compressor (`LinkModeSet::ALL`), the gate-expander
  (`ALL`), the transient shaper (`ALL`) and the true-peak limiter (`dual_mono` and `maximum`). The
  EQ and the soft-clip are `dual_mono` only.
- **The precedent.** Issue #1087 lowered a session `bypass` to per-lane shunt state: the effect is
  prepared `bypass = false`, so a bypassed and an enabled instance share one `EffectProgramKey`
  and one bank, and the lane carries the real bit (`prepare.rs:40-55`).
- **Live today.** The C ABI classifier compares an effect's link mode structurally, so any change
  rebuilds (`crates/host-core/src/live_delta.rs:150-156`). Decision 15 D15-9 lists a link-mode
  change among the edits that duck-swap a strip (*Duck-swap a strip whose state cannot continue
  across a plan swap*, #1324). Decision 15 records the qualifier: after #1371, a link-mode change
  on these four effects is live, and the duck-swap covers a link-mode change only where the link
  stays prepared.

## Decision: live, not prepared (decision 14 rule 3, decision 15 D15-13 E4)

The old D2 said a per-strip link-mode change "is structural (a recompile), never a live control",
and gave no reason. Superseded by decision 15 D15-13 E4: the change is live. The evidence, from the
code on `main`:

- **No optimisation needs a constant mode.** Every fast path that depends on the link is already
  chosen per block: the compressor's `dual_mono` arm (`kernel.rs:610`), the transient shaper's
  dispatch (`:494-498`) and the limiter's linked-pair decision (`:3588-3594`). With per-lane state,
  each becomes the same per-block test over the bank's lanes ("every lane `dual_mono`", "every
  lane `maximum` and agreeing"), one mask reduction per block, as #982's `wet` test already is. A
  bank whose lanes share one mode keeps today's arm and bits. L1 measures it (2% rule below).
- **No glitch is forced.** The link changes only the detector input. Each of the four effects
  feeds that input through its own smoothed gain path (compressor and gate envelopes,
  transient-shaper envelopes, the limiter's box-ramped gain), so the output gain stays continuous.
  D15-1 still applies: the detector law itself glides over the ramp (D4).
- **No correctness reason.** The limiter's ceiling proof needs `g[n] <= r_own[n - N]` per channel.
  Any blend between a channel's own peak and the linked `max` peak is at least the own peak, so
  every required gain in the window stays at or below that channel's own requirement, whatever
  mix of modes the window holds. The proof survives a switch in either direction. L3a tests it.
- **Memory.** After L1 the mode is per-lane words in the plan: no new memory (decision 14's note
  on #1236).

## Frozen design

- **D1. Per-lane link state (L1).** For each of the four effects above, the link mode leaves the
  program key on the #1087 model: the bank is prepared with a lowered key mode, and each member
  request's own mode becomes that lane's state (the compressor's `linked`/`averaged` masks, the
  gate's `link_max`/`link_avg` words, new per-lane masks in the transient shaper and the limiter).
  A bank whose lanes all share one mode keeps today's arm and bits (class A for every existing
  session). A mixed bank takes the general arm for the whole bank and never moves a lane's bits.
  The same lowering applies to an insert of these effects: inserts that differ only by link mode
  now share a bank, which regroups lanes and moves no bit.
- **D2. Grammar (L2).** V1 has no optional fields (`docs/SESSION_SCHEMA_V1.md:210`), so each console
  entry gains a **required** key `link_mode` whose closed token set is `slot` (run the slot's mode)
  plus the three link tokens. The migration writes `"slot"` into every entry, so no document's
  meaning or render moves. A value the effect does not support refuses with
  `effect.link_mode.unsupported` (`prepare.rs:374`) at the entry's path, as an insert's does. The
  SDK writes `"slot"` by default and accepts a per-strip `linkMode`.
- **D3. Wire (L2).** The console entry message appends one field (an in-place V1 amendment, on the
  decision 12 and 13 precedent). Nothing is renumbered and there is no `ABI_VERSION` bump.
- **D4. Live switch (L3).** Each linked lane gains a one-word latest-target link cell (mode and
  ramp length) on the effect lane's cells (*Hold effect parameter, bypass and EQ-target values in
  latest-target cells*, #1345), drained after bypass and before parameters; a link edit is never
  refused for room (D15-2). Each lane holds two link weights per channel, `w_link` (own magnitude
  to linked combination) and `w_avg` (maximum to average), as `LinearRamp`s over the edit's own
  ramp (the `Link` row of *Carry an optional per-edit ramp length on live session edits*, #1394
  D3, read through `LiveRamps::resolve`, #1394 D5), or, when the edit has none, the session
  default `LiveRamps::link_samples` (the `fader_ms` key, #1054 D3 and D4; D15-1; root ruling R9).
  While a ramp runs, the detector is `combined = max + w_avg * (avg - max)` and
  `d = own + w_link * (combined - own)`, with each effect's operation order frozen in its slice. At rest the weights are exactly 0 or 1, and the
  kernel takes today's `select` form from the derived masks, so a settled lane renders today's bits.
  The classifier writes the link cell for a changed console-entry override, a changed slot default
  (one write per strip whose entry says `slot`), and a changed insert link mode, for these four
  effects.
- **D5. Limiter agreement.** After a switch into `maximum`, the linked-pair record `gain_linked`
  is false until both channels' gain words agree again. The limiter re-checks agreement
  (`gain_state_agrees`) once its ring has been fully rewritten under `maximum`, so the fast path
  returns within one ring length. It is never taken on disagreeing words.
- **D6. Carry (D15-7).** The per-lane link mode and weight ramps are lane state, written by each
  effect's payload codec (#1370), so *Carry console effect lanes across a plan swap* (#1279) and *Carry
  live-controlled effect lanes across a plan swap* (#1280) carry them. #1279's D1 lists link mode
  among the values that must be bit-equal for a carry; for these four effects that clause becomes
  "carry, then retarget" with a link-cell write once L3 lands. The payload codecs are stream A's code
  in the same effect crates: coordinate each crate's `write_lane`/`restore_lane` (or equivalent)
  edit with stream A.

## Slices

1. **L1, #1368: *Lower the link mode to per-lane state in the linked effects' banks*** (D1).
   No grammar change. Gates: a mixed-mode cohort binds one bank per console slot and every
   lane is bit-identical to a fresh plan of that lane's strip at its own mode; every checked-in
   render digest and the console benchmark workload counts are unchanged; per-effect bank
   conformance at both lane widths; the settled all-`dual_mono` console record's
   `isolated_cycles_per_lane_sample` stays within 2% for each touched effect, else the slice stops
   and reports the gap for owner ruling. If one effect cannot fit in half a day, split it into its
   own slice; the limiter is the likely one.
2. **L2, #1369: *Declare a strip's console link mode in the session, the wire and the SDK*** (D2, D3).
   Depends on L1. Gates: the grammar, canonical writer, wire round trip and SDK builder accept the
   four tokens and refuse others; an unsupported mode refuses at the entry's path; migration
   changes every checked-in document by the added key alone; a session with a per-strip override
   renders bit-identical to one where that strip carries an insert of the same effect at that mode
   (pre-insert position), and the slot still binds one bank; the `author-session` skill and the app
   handoff drop the bus-compression hazard guidance (DESIGN 2.2b).
   **Deliverable (K3 follow-up amendment, from the K3 verdict's MINOR-2): amend the binding texts
   in L2's own PR.**
   - `AGENTS.md`, "Approved audio architecture" (`:31`), the console sentence "the session declares
     each console slot once (a stable slot ID, the native effect, quality and link mode), and every
     strip carries every slot, in that order, with only its own parameters and bypass": it gains
     the strip's link-mode override (the slot's `link_mode` becomes the default an entry may
     override).
   - Decision 12's ruling, `docs/rulings/engine-footprint-2026-09-29.md`, the **Shape** bullet
     (`:48`): an amendment note that the entry now also carries `link_mode` (`slot` or a link
     token), citing decision 13 Q1 and this issue.
   - Decision 13's ruling, Q1: recorded as delivered.
3. **L3. Switch a link mode live** (D4, D5), as two half-day slices:
   - **L3a, #1370: *Ramp a lane's detector link between modes*** in the four kernels, through a
     render-safe `retarget_link` entry point; it also adds the payload words of D6. Gates: per effect and at both widths, a switch in each direction
     renders a detector that matches an `f64` reference of D4's blend within the effect's
     SIMD/scalar tolerance, lands on today's bits once the ramp ends, and never moves a bank-mate's
     bits; the limiter's output never exceeds its ceiling across any switch, on a full-scale
     two-channel fixture whose channels differ by 20 dB; D5's record returns within one ring length.
   - **L3b, #1371: *Carry the link record from the edit to the lane*** on both hosts: the link
     cell, its application at the block boundary, and the classifier rows of D4. Gates: the
     classifier turns each of D4's three edits into link writes and no rebuild; a paused host's many
     link edits are all admitted and the last wins; on the C ABI a committed override
     change commits with no plan replacement, and the render equals a reference fed the same link write;
     a swap during a link ramp carries it (with #1279 and #1280).

## Invariants

- A console slot always banks, on every target and for every strip count (decision 12). A link
  override never splits or declines a bank.
- Banking may couple lanes' cost, never their bits: a lane renders the same bits in a mixed bank
  as alone.
- No third-party effect, no sidechain change.

## Non-goals

- Overriding a slot's effect, quality or sidechain per strip.
- Link modes for the EQ, the soft-clip or the delay (`dual_mono` only).
- The multiband compressor, which is not console-eligible and keeps a compile-time link
  (`render`, `crates/multiband-compressor/src/lib.rs:1022-1052`). *Make the multiband compressor's
  link mode live* (#1367) applies this design to it after #1371.

## Objective gates (umbrella)

- #1368, #1369, #1370, #1371 and #1367 closed with Sol PASS, their evidence upstream and their GitHub issues
  synchronized.
- `AGENTS.md`'s console sentence and decision 12's Shape bullet carry the per-strip link-mode
  override, landed in L2's PR, with no decision-13 qualifier left on it.
- Decision 13's ruling records Q1 as delivered.
- Each slice's own gates include: the touched effects' `KERNEL_ROSTER` rows;
  `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`, then
  `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`,
  `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` and
  `bash scripts/test-web-audioworklet.sh`; `bash scripts/check-cross-targets.sh`;
  `scripts/run-aarch64-tests.sh` (CI `aarch64-debug`/`aarch64-release` when no arm64 host);
  `cargo test --locked --all-targets -p compressor -p gate-expander -p transient-shaper -p true-peak-limiter --features math/lane,lane/test-support`;
  `cargo test --locked -p graph-compiler -p host-core --features graph/test-support,host-core/test-support`;
  `cargo test --locked -p console-workload`;
  `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
  `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`.

## Test value

Each slice states its own. The umbrella's claims are what these defend:
- L1's mixed-cohort gate: a lowering that leaves link in the key splits the bank, and a per-bank
  link arm chosen from one lane moves another lane's bits. Either turns it red.
- L3a's ceiling gate: a blend that can fall below a channel's own peak lets the limiter overshoot
  on the quieter channel during the ramp; it turns red. No existing test switches a limiter's link.
- L3b's no-rebuild gate: a classifier that still compares link structurally rebuilds; it turns red.

## Dependencies

This umbrella closes last, after its slices:

- *Lower the link mode to per-lane state in the linked effects' banks* (#1368).
- *Declare a strip's console link mode in the session, the wire and the SDK* (#1369).
- *Ramp a lane's detector link between modes* (#1370).
- *Carry the link record from the edit to the lane* (#1371).
- *Make the multiband compressor's link mode live* (#1367).

The slices in turn depend on:

- *Carry console effect lanes across a plan swap* (#1279).
- *Carry live-controlled effect lanes across a plan swap* (#1280).
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345), for L3b's
  link cell.
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), for `LiveRamps::link_samples` (`fader_ms`).
- *Carry an optional per-edit ramp length on live session edits* (#1394), for L3's per-edit ramp
  field and `LiveRamps::resolve`.
- Batch K3 of *Submix strips and live aux sends* (#1196) has merged (`6f1788f3a`); it no longer
  blocks.
