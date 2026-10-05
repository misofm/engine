# Duck-swap a strip whose state cannot continue across a plan swap

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

When a structural edit changes how a playing strip processes audio (an insert added, removed or
reordered, an effect's quality, `delay_samples`, a value the plan keeps prepared, a bypass the
plan keeps prepared), the strip ducks out over the session mute ramp, its sends from taps before
the fader with it, the new plan is swapped in after the ramp, and the strip and those sends fade
back in. Today the edited strip's new chain starts at rest at the swap block, a step from the old
chain's output to the new one's. Strips the edit did not touch keep playing, bit for bit. A
transaction that reverts the edit during the duck brings the strip back with the fade-in; it
never leaves it muted.

## Context

- D15-7: a changed prepared (rule-3) value restarts its owner behind a D15-9 transition; an owner
  carries only when the join of stream A's carry slices (#1277, #1279-#1284) finds its values and
  layout equal. Successor preparation: `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`), `prepare_host_runtime_with_live_lanes_successor`
  (`:818`).
- The classifier sends these edits to a rebuild: `LiveRebuild::Structure` for insert order,
  identity, quality, link mode and every other non-value field, `Prepared` for a non-`Block`
  parameter, `PreparedBypass` for the delay and the multiband (`crates/host-core/src/live_delta.rs:118-147`,
  steps at `:150-180`). `delay_samples` is a channel-builtin field
  (`crates/session/src/visit.rs:99`). *Carry the link record from the edit to the lane* (#1371)
  and *Make the multiband compressor's link mode live* (#1367) make the link mode live for the
  effects that have one. A link-mode change is therefore not a trigger of this issue: D1 ducks a
  strip for it only while some effect's link is still prepared, by construction, and gate 3 does
  not list it.
- Round 2 (Q-C2): warm state removes at-rest artefacts, but a switch between two different chains is
  still a step of `new - old` at the swap, so the duck stays even with a warm successor (#1287); the
  duck runs on the predecessor before `S` and the fade-in on the successor after it.
- The shared step: *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325) writes
  ramped mutes into the displaced plan's strip lanes and pre-fader route lanes, reads
  `p = render_sample` (`crates/capi/src/runtime/plan.rs:10`, `:236`) and schedules the successor
  at `S = ceil_q(p + q + N + C)`. Its host-core function `plan_strip_transition(base, overlay,
  next, successor)` runs after preparation and reads the prepared successor (#1325 D7); it returns the
  duck set, never an arm set. This issue extends the duck set; arming happens in preparation.
- A route from a tap before the fader (`input`, `post_input`, `insert_send`, `insert_return`,
  `pre_fader`; `SendTap`, `crates/session/src/model.rs:856-871`) bypasses the fader, so a fader
  duck does not reach it.
- The armed fade-in: *Fade in a strip that a swap adds during playback* (#1288): an armed fader
  channel is prepared muted and fires an unmute ramp of `N` at the first block boundary at or after
  `played block + D`, never before the plan's adoption block.

## Decisions frozen for this slice

- **D1. Which strips.** Exactly `PreparedHost::restarted_strips()` (#1277 D6): the strips in both
  plans with at least one owner the carry join restarts for a prepared difference, which each later
  carry slice extends with its family. This is the D15-9 list on the carry rules, and it stays
  correct for any future reason an owner cannot carry. A strip whose owners all carry is never
  ducked; its value changes stay live or carried-then-retargeted (D15-7). A changed route source,
  tap or destination does not put the source strip in this set (D15-9: a re-pointed route is
  ramped at route level and never ducks its source strip; #1283 and #1284 leave it out; #1363
  ramps the route itself).
  - **Sidechain consumers join the set.** A sidechain edge (`GraphEdgeId::EffectSidechain`) has no
    gain lane: the compiler wires the routed sidechain's source tap straight into the effect's
    `SidechainInput` port (`crates/graph-compiler/src/compile.rs:390-416`; the tap's stage,
    `crates/graph-compiler/src/ids.rs:187-193` and `:210-220`). So a duck cannot reach it. When a
    restarted strip's tap feeds a carried effect's sidechain and that tap follows a restarted owner
    with state (taps `post_input`, `insert_send`, `insert_return`, `pre_fader`), the detector sees
    the old chain's output until `S` and the restarted chain's at-rest output after it: a step at the
    swap, even in a plain duck-swap. So the strip that holds the consuming effect joins the set. The
    rule is applied after the carry join and repeated until no strip is added, since a joined strip
    may itself feed a sidechain; `restarted_strips()` reports the closed set. A track's `input` tap
    is its raw source, which no owner of the restarted strip touches, so it adds nothing. A
    submix's `input` tap is the sum of its incoming routes. It adds nothing only while the
    submix's `Input` stage keeps its place against the consumer: the consumer's sidechain line
    from it keeps its length (moved by #1283 D2). Under a warm lead `P` that means the stage
    arrives at exactly its predecessor arrival plus `P` (*Prepare a warm successor whose carried
    nodes lead the predecessor by P*, #1354 D2 step 6); without one, at its predecessor arrival.
    This holds for every rebuild, warm or not; #1354 D2 step 4 applies the same check with `P = 0`.
    If the line changes length, #1283 D4's head-aligned copy breaks the detector's input at `S`,
    so the consuming strip joins the set. A `post_fader` or `post_pan` tap is exact `+0.0`
    from the end of the duck until the fire (the fader is ducked to a settled `+0.0`, then armed
    muted), so it adds nothing either.
- **D2. Whole pre-fader restart.** For a duck-swapped strip, preparation marks not carried, after
  the join: every owner before the fader, the fader stage, and every route out of the strip whose
  tap precedes the fader. A partial carry upstream of a restarted latent owner would emit its
  carried tail, then the restarted owner's at-rest zeros: a hole with a step at each edge. Owners
  after the fader (matrix/pan, post-fader and post-pan sends, their compensation lines) carry as the
  join says; they carry the ducked tail.
  - A source-claim line (*Grow latency during playback by adopting a primed warm successor*, #1287
    D1) is not an owner of the strip: it holds raw source frames and no processing state, so it
    carries for a duck-swapped strip too (*Carry source-claim lines across a plan swap and fill a
    grown line for a prime*, #1402 D1).
  - **A strip restarted whole.** A strip that warm preparation restarts whole (*Prepare a warm
    successor whose carried nodes lead the predecessor by P*, #1354 D2 step 6, `restart_whole`)
    is in `restarted_strips()` (#1354 D4) and is marked not carried in every owner, after the
    fader too (matrix/pan, post-fader and post-pan sends, their compensation lines), except its
    claim lines. It is ducked (D4) and armed (D3) like any duck-swapped strip. Nothing is lost by
    restarting its post-fader owners at rest: by `S` its fader output and every route line out of
    it hold exact `+0.0` (D4's duck and #1325 D3's `C`), and its armed fader keeps them at `+0.0`
    until the fire.
- **D3. Arm.** Each duck-swapped strip is armed through #1288's arm entry point (its D1 strip-set
  argument; channels the model leaves unmuted), with `D` per #1288 D4 (its pre-fader latency; for a
  submix plus the longest compensation delay of an input route whose line starts at rest). Its
  routes from taps before the fader are armed with it by #1363 D1 (c). The fire comes no earlier
  than adoption (#1288 D3), so for a playing source the fade starts at the first block at or after
  `S + D`.
- **D4. Duck.** This issue extends #1325 D7's `plan_strip_transition`: the duck set is the removed
  strips plus `successor.restarted_strips()`, and the duck routes are every route from those strips
  whose tap precedes the fader, minus every strip and route already in the running plan's duck
  overlay (#1325 D6: a second ramped mute would reset the running ramp to `N` from its part-ducked
  level and end it after the scheduled `S`). The control plane writes their ramped mutes and
  schedules `S` exactly as #1325 D2-D3, in the same transaction, with one `S`; #1325 D3's `C`
  covers every route out of a duck-swapped strip as well as out of a removed one, since a restarted
  pre-fader route's line must empty before `S`.
- **D5. Edits during the window.**
  - A live edit to a ducked strip goes to the newest candidate's cells and applies at adoption
    (D15-17). A mute disarms the strip (#1288 D2), so the user's mute wins; a fader edit sets the
    gain the fade reaches.
  - A structural edit withdraws the candidate (#1310) and uses #1325 D6's duck overlay: its base is
    the running plan's kept model with the ducked strips muted, it inherits `S`, and every overlay
    strip it keeps is restarted (the withdrawn candidate carried none of its owners before the
    fader, D2) and armed. So a transaction that reverts the edit restores the strip with the fade-in
    at `S`; it never leaves the strip muted, and never steps from a part-ducked level.
- **D6. Reporting.** As #1325 D5: a planned duck-swap is the designed result of its edit, so its
  revision reports path `rebuild` and completes at `S` as `EXACT` (or `SUPERSEDED`), with no
  fallback flag and no counter. `TRANSITION_FALLBACK` and its counter are set only when the fallback
  path of *Duck-swap the strips a latency growth restarts, and fall back to the transition when a
  warm successor cannot adopt* (#1397) publishes this duck-swap because a warm successor cannot
  adopt; that issue owns the flag. This issue never sets it.
- **D7. Realtime and the acked-batch question.** As #1325 D8. The arm table is sized at
  preparation.

## Deliverables

1. D1-D3 in host-core successor preparation, after the carry join; D4 in
   `crates/host-core/src/transition.rs`. D5's overlay is #1325's; this issue adds no control-plane
   state.
2. The control plane passes the duck set through #1325's path (no new control-plane code beyond
   the call).
3. Header comment and `docs/C_ABI_V1_QUALIFICATION.md`: which edits duck-swap (and that a strip
   whose effect is sidechained from a ducked strip's tap ducks with it, D1), the dip length
   (`N`, the wait to `S`, `D`, `N`).
4. Tests below.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/transition.rs`,
  `crates/host-core/tests/successor_swap.rs`
- `crates/capi/tests/strip_transitions.rs`
- `crates/capi/include/miso_engine_v1.h` (comments only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Added strips (#1288) and removed strips (#1325).
- Composing with a warm successor: *Duck-swap the strips a latency growth restarts, and fall back
  to the transition when a warm successor cannot adopt* (#1397) publishes the warm candidate with
  `not_before` at this duck's `S`, and render adopts it no earlier (*Grow latency during playback by
  adopting a primed warm successor*, #1287). The fire needs no change for it: it measures the played
  block in the plan's own render time (#1288 D3).
- An arm still waiting when the adopted plan is itself succeeded: *Keep an added strip's pending
  fade-in across a later plan swap* (#1392).
- No crossfade. A true crossfade with ghost strips is deferred by D15-9; it reopens on a measured,
  audible dip in a listening test.

## Hazards

- The duck set is as large as the carry join makes it. While a state family has no carry slice
  (for example per-node effect instances before #1282), every strip holding such an owner is
  duck-swapped on any structural edit. That is correct (no click), and each of stream A's carry
  slices shrinks it.

## Objective gates

Through the exported C entries, quantum 128, 48 kHz, `N = 2000`, single-threaded, as #1325's gates.

1. **Duck, swap, fade.** Track A muted in the model, track B audible. Between blocks `k` and
   `k + 1` a transaction adds a compressor insert to B. (a) Every block up to `S` equals a reference
   that live-mutes B at the same point. (b) From `S` on, every block equals a fresh plan of the
   successor session, fed the same source frames, with B muted and live-unmuted with `N` at its
   first block. (c) The same with B holding a true-peak limiter insert in A as well, so `D` is
   the limiter's latency: the fresh plan's unmute lands at block `ceil_q(D)/q`. The compressor
   adds no latency (`crates/compressor/src/lib.rs:295`), so no node's arrival grows and the edit
   stays a plain duck-swap after the warm successor lands (*Prepare a warm successor whose
   carried nodes lead the predecessor by P*, #1354 D2 step 4 returns `Ordinary`); a latency
   growth on an audible strip is #1397's gate 1. All bit-identical. (d) The watermark reports the
   revision with first sample `S` and `MISO_ENGINE_V1_OUTCOME_EXACT` only;
   `transition_fallback_count` is unchanged.
2. **Only edited strips duck.** A and B audible, B's source fed exact zeros; the gate 1 transaction
   leaves every block equal to a run with no transaction.
3. **Every trigger ducks.** For each edit (insert added; removed; reordered; an insert's quality;
   `delay_samples`; the true-peak limiter's `lookahead`, which is `AutomationRate::None`,
   `crates/true-peak-limiter/src/lib.rs:198-210`), B's output holds at least `q` consecutive
   exact-zero frames per channel ending at `S`. A value-only fader edit produces no such run, and
   neither does a re-pointed send from B (D15-9).
4. **Whole restart.** In `successor_swap.rs`, at `Backend::Simd8`, `Backend::Simd4` and prepared
   between render calls: a strip whose second insert is replaced has all its pre-fader owners and
   its fader not carried and is armed; an untouched strip carries every owner.
5. **Pre-fader send ducks with the strip.** Gate 1(a)-(b) with B also holding a `pre_fader` send
   into submix R, and R's output observed: up to `S` it equals a reference that live-mutes B and the
   send at the same point; from `S` it equals the fresh successor plan with B and the send muted
   and live-unmuted with `N` at the fire block. Bit-identical. R also takes a track through a
   true-peak limiter insert, so the send's line into R has a compensation delay of 300 samples or
   more, `C`; the watermark's first sample is `S = ceil_q(p + q + N + C)`.
6. **A revert during the duck fades back in.** In gate 1, at block `k + 4` a second transaction
   removes the compressor again. It returns OK. Up to `S` every block equals gate 1(a)'s
   reference; from `S` every block equals a fresh plan of the reverted session with B muted and
   live-unmuted with `N` at `S`; from `S + N` on, B's output equals the unducked reference. B is
   in the duck overlay, so the revert writes no second mute: the adoption is at exactly gate 1's
   `S`.
   (b) Gate 5's session (B's `pre_fader` send into R with its compensation delay `C`): the revert
   at `k + 4` likewise leaves R's output up to `S` equal to gate 5's reference, and the adoption
   is at gate 5's `S`, which counts `C`.
7. **A sidechain consumer ducks with its source.** Gate 1's transaction, with B muted in the model
   and track C audible. C holds a compressor insert whose `sidechain-in` port is routed from a tap
   of B. (a) From B's `insert_return` tap: every block up to `S` equals a reference that live-mutes
   C at the same point; from `S` every block equals a fresh plan of the successor session with C
   muted and live-unmuted with `N` at its fire block; C is in `restarted_strips()`. (b) From B's
   `input` tap: C carries every owner, is not in `restarted_strips()`, and every block equals a run
   with no transaction. Bit-identical, at `Backend::Simd8` and `Backend::Simd4` in
   `successor_swap.rs` and through the C entries.
8. **Realtime.** As #1325's realtime gate, over the fade blocks too.
9. Commands:
   - `cargo test --locked -p capi --test strip_transitions`
   - `cargo test --locked -p host-core -p capi --features host-core/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1(d): a planned duck-swap reported as a fallback (D6) turns it red.
- Gate 1(b): a successor that carries the ducked fader (the strip stays muted) or does not arm it
  (it pops in at full gain) turns it red. Gate 1(c): a fade that ignores `D` turns it red.
- Gate 2: a duck applied to every strip, or an untouched strip restarted, turns it red.
- Gate 3: an edit kind that restarts an owner without the duck (a step at `S`) turns it red.
- Gate 3: a re-pointed send that still ducks its source strip (against D15-9) turns its last
  clause red.
- Gate 4: a partial carry upstream of a restarted owner (the hole in D2) turns it red.
- Gate 5: a pre-fader send carried across the duck-swap (it keeps sending through the dip, then
  steps when the restarted chain behind it emits) or left unarmed turns it red, and so does an `S`
  without the send's `C` (the line's last ducked frames cut at `S`).
- Gate 7(a): a duck set that ignores sidechain edges (C's compressor reads B's old chain until
  `S`, then its restarted chain's at-rest output, so C's gain reduction steps at `S`) turns it red.
  Gate 7(b): a rule that also adds consumers of an `input` tap ducks C needlessly; it turns red.
- Gate 6: a supersession whose base omits the overlay (the strip carries its ducked fader and stays
  muted) or does not arm the restored strip (it enters at full gain) turns it red. A revert that
  writes a second mute to the ducked strip or its send (the ramp restarts at `k + 4`, the blocks
  before `S` differ and `S` moves) turns it red. Gate 6(b): a `C` that counts only removed strips'
  routes (the send's line cut at `S`) turns it red.

## Dependencies

- *Fade in a strip that a swap adds during playback* (#1288).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363) (D1 (c)).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Give every route whose tap precedes its strip's fader a live lane on every plan* (#1391) (the
  pre-fader route ducks, through #1325 D2).
