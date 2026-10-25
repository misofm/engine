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
  layout equal. Successor preparation: `SuccessorBase { inventory, committed, revision }`
  (`crates/host-core/src/prepare.rs:641-647`), `prepare_host_runtime_with_live_lanes_successor`
  (`:818`).
- Until #1281 and #1282, a strip whose surviving insert lane cannot carry is in
  `restarted_strips()`: its insert restarts at rest on the swap and D1 ducks it (#1277 D6, #1279;
  root amendment, 2026-10-12).
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
  `p = render_sample` (`crates/control-plane/src/plan.rs:13`, `:263`) and schedules the successor
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
  - **A forced restart.** *Duck-swap the strips a latency growth restarts, and fall back to the
    transition when a warm successor cannot adopt* (#1397 D2 step 1) adds the field
    `SuccessorBase::forced_restart`, the transition's duck set. It holds only strips in both plans.
    A strip in it is in `restarted_strips()` like a strip the carry join restarts, and D1-D3 apply
    to it the same way: it is restarted (D2), armed (D3), and the sidechain rule below runs over
    it. The field is empty for every other successor.
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
    is its raw source as its delay line (`delay_samples`, ring length, source) leaves it, which no
    other owner of the restarted strip touches, so it adds nothing unless that delay line restarts:
    the tap is the line's output, which then starts at rest at `S`, so the consumer of a track's or
    a submix's `input` tap whose delay line restarts joins the set, before the submix rules below
    (root amendment, 1324b attempt 2). A submix's `input` tap is the sum of its incoming routes. It adds nothing only while the
    submix's `Input` stage keeps its place against the consumer: the consumer's sidechain line
    from it keeps its length (moved by #1283 D2). Under a warm lead `P` that means the stage
    arrives at exactly its predecessor arrival plus `P` (*Prepare a warm successor whose carried
    nodes lead the predecessor by P*, #1354 D2 step 6); without one, at its predecessor arrival.
    This holds for every rebuild, warm or not; #1354 D2 step 4 applies the same check with `P = 0`.
    If the line changes length, #1283 D4's head-aligned copy breaks the detector's input at `S`,
    so the consuming strip joins the set. Under a lead the stage misses `a + P` only when a line
    that is not carried held it in A, such as a route from a restarted strip's tap before the
    fader (D2): a carried `post_fader` or `post_pan` route out of a restarted strip has its floor
    `a + P` (#1285 D2) and holds the stage there. The submix's `input` tap also adds nothing,
    whatever the stage's arrival, when every line into the stage comes from a ducked or an added
    strip: those lines hold exact `+0.0` from the end of the duck until the fire, and the
    scheduled `S` counts the sidechain line plus the longest line into the stage (D4's `C`, on
    every duck-swap, warm or not). A `post_fader` or `post_pan` tap is exact `+0.0`
    from the end of the duck until the fire (the fader is ducked to a settled `+0.0`, then armed
    muted), so it adds nothing either.
- **D2. Whole pre-fader restart.** For a duck-swapped strip, preparation marks not carried, after
  the join: every owner before the fader (the compensation lines on its chain included), the fader
  stage, and every route out of the strip whose tap precedes the fader. A partial carry upstream of
  a restarted latent owner would emit its carried tail, then the restarted owner's at-rest zeros: a
  hole with a step at each edge. Owners after the fader (matrix/pan, post-fader and post-pan sends,
  their compensation lines) carry as the join says; they carry the ducked tail.
  - A source-claim line (*Grow latency during playback by adopting a primed warm successor*, #1287
    D1) is not an owner of the strip: it holds raw source frames and no processing state, so it
    carries for a duck-swapped strip too (*Carry source-claim lines across a plan swap and fill a
    grown line for a prime*, #1402 D1).
  - **A strip restarted whole.** A strip that warm preparation restarts whole (*Prepare a warm
    successor whose carried nodes lead the predecessor by P*, #1354 D2 step 6, `restart_whole`)
    is in `restarted_strips()` (#1354 D4) and is marked not carried in every owner, after the
    fader too (matrix/pan, post-fader and post-pan sends, their compensation lines), except its
    claim lines. It is ducked (D4) and armed (D3) like any duck-swapped strip. Nothing is lost by
    restarting its post-fader owners at rest: by `S` its fader output and every line out of it
    hold exact `+0.0` (D4's duck, with `S` counted with D4's `C`, which includes each
    `EffectSidechain` line from its `post_fader` or `post_pan` tap into a carried node, which its
    grown arrival can shorten), and its armed fader keeps them at `+0.0` until the fire.
- **D3. Arm.** Each duck-swapped strip, a forced-restart strip (D1) included, is armed through
  #1288's arm entry point (its D1 strip-set argument; channels the model leaves unmuted), with `D`
  per #1288 D4 (its pre-fader latency, the compensation lines on its chain included; for a submix
  plus the longest compensation delay of an input route whose line starts at rest). Its
  routes from taps before the fader are armed with it by #1363 D1 (c). The fire comes no earlier
  than adoption (#1288 D3), so for a playing source the fade starts at the first block at or after
  `S + D`.
- **D4. Duck.** This issue extends #1325 D7's `plan_strip_transition`: the duck set is the removed
  strips plus `successor.restarted_strips()` (for the transition, *Duck-swap the strips a latency
  growth restarts, and fall back to the transition when a warm successor cannot adopt*, that set
  holds #1397 D2's grown strips through `SuccessorBase::forced_restart`, D1), and the duck routes
  are every route from those strips whose tap precedes the fader, minus every strip and route
  already in the running plan's duck overlay (#1325 D6: a second ramped mute would reset the running
  ramp to `N` from its part-ducked level and end it after the scheduled `S`). The control plane
  writes their ramped mutes and schedules `S` exactly as #1325 D2-D3, in the same transaction, with
  one `S`; #1325 D3's `C` covers every route out of a duck-swapped strip as well as out of a removed
  one, since a restarted pre-fader route's line must empty before `S`.
  - **`C` for every duck-swap.** On every duck-swap, warm or not (a rebuild that `warm_lead`
    calls `Ordinary`, a lead-0 or `Primed` warm edit, every rebuild of a session without a
    `WarmConfig`, and the transition, #1397 D2), `plan_strip_transition` computes `C` as the
    largest of these lengths, all in the running plan: each route line out of a ducked strip
    (#1325 D3); each `EffectSidechain` line from a ducked strip's `post_fader` or `post_pan` tap
    into a carried node (a grown arrival can shorten it, for a restarted strip, D2, and for a
    transition's grown strip); and, for each restarted submix whose `input` tap feeds a carried
    node's sidechain under D1's exemption (every line into its `Input` stage comes from a ducked
    or an added strip), that sidechain line plus the longest line into the stage. A ducked
    strip's last nonzero frame then leaves every such path by `p + q + N + C <= S`, so a line
    that the successor shortens drops only exact `+0.0` at `S`. *Duck-swap the strips a latency
    growth restarts, and fall back to the transition when a warm successor cannot adopt* (#1397)
    D1 and D2 use this `C` unchanged.
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
9. **A shortened sidechain line from a restarted submix's `input` tap drops no fade sample.** In A,
   track T2 routes from its `post_fader` tap to submix Bu; Bu holds a true-peak limiter insert and
   routes to the output; track T3 holds a true-peak limiter insert, then a compressor insert whose
   routed sidechain reads Bu's `input` tap, and routes to the output. All are audible. One
   transaction removes T2, adds track T1 with a true-peak limiter insert routed from its
   `post_fader` tap to Bu, and removes Bu's limiter. No carried node's arrival grows (restarted Bu's
   `Input` stage grows from 0 to `L`), so it is a plain duck-swap (with a `WarmConfig`, #1354 D2
   step 4 returns `Ordinary`). Bu is restarted, T3 stays carried under D1's exemption (the only line
   into Bu's `Input` stage comes from added T1), and T3's sidechain line shrinks from the limiter's
   latency `L` to 0.
   (a) `plan_strip_transition` returns `C = L` (T3's sidechain line in A, plus T2's line into
   Bu's `Input` stage, which is 0), and the watermark's first sample is `S = ceil_q(p + q + N +
   L)`.
   (b) Every block before T1's fire block (the first block at or after `S + L`) equals a
   reference that makes no structural edit and live-mutes T2 and Bu with `N` at the same point.
   Bit-identical.
10. Commands:
   - `cargo test --locked -p capi --test strip_transitions`
   - `cargo test --locked -p host-core -p capi --features host-core/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-host-core-policy.sh`, `cargo run --locked --release -q -p realtime-policy`,
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
- Gate 9: a `C` that counts route lines only (round-7 M3) schedules `S` before A's sidechain line
  has emptied, so W drops the last `L` samples of T2's fade from T3's detector and T3's gain
  differs from the reference after `S`; it turns red.

## Dependencies

- *Fade in a strip that a swap adds during playback* (#1288).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363) (D1 (c)).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Give every route whose tap precedes its strip's fader a live lane on every plan* (#1391) (the
  pre-fader route ducks, through #1325 D2).
- *Carry live-controlled effect lanes across a plan swap* (#1280), *Carry an insert lane that moves
  between a bank and a per-node instance* (#1281) and *Carry per-node effect instances across a plan
  swap* (#1282): gates 5, 7(b) and 9 assume that an untouched insert carries. Until these land, a strip
  that holds an insert is in `restarted_strips()` and is ducked (root, 2026-10-12).

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `e2f6cfb24`.

**Batch order (root, 2026-10-15).** Batch 15a is the controls lane: #1344, #1398, #1310, in
that order. The rest of batch 15 follows. This slice is not in batch 15a; see the root ruling below.

**Moved anchors.**
- `SuccessorBase` is `crates/host-core/src/prepare.rs:1070-1097`, with six fields: `inventory`,
  `committed`, `revision`, `edit_ramps` (#1394), `monitor` (#1559) and `observation` (#1584).
  `prepare_host_runtime_with_live_lanes_successor` is `:1311`. `restarted_strips()` is
  `:686-690`; its doc already names this issue as its consumer. The restart set is built at
  `:3101-3105` from the strip, effect and strip-delay joins.
- The classifier no longer has `Prepared` and `PreparedBypass`. Decision 17's control rows
  replaced them: `LiveRebuild` is `prepare.rs:4230` (`Structure`, `Domain`,
  `RebuildOnly(reason)`, `AutomationTarget`), and the steps are documented at
  `crates/host-core/src/live_delta.rs:259-280`. A non-`Block` parameter and a rebuild-only builtin
  are `RebuildOnly` with the row's reason. Each trigger of gate 3 still rebuilds: insert add,
  remove, order and quality are `Structure`; `delay_samples` is a rebuild-only row (the row at
  `crates/session/src/visit.rs:122`, field 5; `crates/host-core/src/control_provider.rs:3108-3113`
  finds it as "a rebuild-only value"); the limiter's `lookahead_ms` is `AutomationRate::None` with a
  prepared reason (`crates/true-peak-limiter/src/lib.rs:212-228`).
- `SendTap` is `crates/session/src/model.rs:957-972`.
- The sidechain edge is built at `crates/graph-compiler/src/compile.rs:377-384`; the tap's stage
  is `crates/graph-compiler/src/ids.rs:198` and `:210-220`.
- `render_sample` is `crates/control-plane/src/plan.rs:13`, read at `:39`.
- The compressor's zero latency is `crates/compressor/src/lib.rs:306`.

**Code that does not exist yet.** `crates/host-core/src/transition.rs`,
`plan_strip_transition`, the duck overlay and `crates/capi/tests/strip_transitions.rs` are
#1325's. The arm entry point is #1288's. `SuccessorBase::forced_restart` is #1397's; until #1397
lands, D1's forced-restart clause has nothing to read.

**Dependencies on GitHub.**
- Closed: #1277, #1280, #1281 and #1282. So per-node and moving inserts carry now. The hazard
  and the dependency note on gates 5, 7(b) and 9 no longer apply: an untouched insert carries.
- Open, in batch 15a before this slice: #1310.
- Open and not in batch 15: #1288 (arm), #1325 (duck, `S`, overlay, `transition.rs`), #1363 (route
  arm, D3) and #1391 (pre-fader route lanes). D1-D5 and every gate need all four.
- Forward references, not dependencies: #1397, #1354, #1287, #1402 and #1392 (open).
- **Root decision under owner delegation (2026-10-15; not one of B15-R1 to B15-R9): #1324 leaves batch 15a. It lands in a later batch after #1288,
  #1391, #1363 and #1325, in that order. Batch 15a is #1344, #1398, #1310.**
  The finding follows. It needs #1288, #1325, #1363 and #1391,
  which are open and outside batch 15. None of their code is on `main`. Proposed: move #1324 out
  of 15a to the batch that lands #1288, #1391, #1363 and #1325, in that order, with #1324 after
  them. Batch 15a is then #1344, #1398 and #1310. #1285's batch-15 note (D3's duck-swap) then
  keeps its fallback: until #1324 lands, a restarted strip starts at rest with no duck. The
  alternative is to add the four to batch 15a before #1324.

**Decisions.**
- D1-D7 hold against the code. Decision 17 does not change which strips restart: the restart set
  comes from the carry joins, not from the classifier's rows.
- D3's "channels the model leaves unmuted": since #1559 a successor bakes the effective values
  (`controls::effective`: the model, VCA mutes and the monitor overlay, C8). An armed strip must
  stay muted when the effective value is muted, so a solo elsewhere does not fade it in. Read
  D3 with "effective" for "model". #1288's arm entry point must take the same input.
- D5's structural edit during the window is #1310's supersession with #1325 D6's overlay. #1310's
  base (its batch-15 amendment) keeps P0's model; the overlay mutes go on top of it.

**Authorized paths.** Deliverable 2 calls #1325's path from the control plane. Add
`crates/control-plane/src/control.rs` (the call only). The control plane is
`crates/control-plane/src/` since #1309.

**Gates and commands.** The commands, packages and scripts exist, except
`crates/capi/tests/strip_transitions.rs`, which #1325 creates. `--test strip_transitions` fails
until then.

**Realtime-path edits.** None planned under `crates/engine/src/realtime`. The fire and the ramped
mutes are #1288's and #1325's render-side work.

## Slice 1324a record (implementer, 2026-10-25)

Scope (root, scope note R1-R5): D1 on the carry joins only, D2, D3, D4 with `C` over route lines,
D5 through #1325b's overlay, D6, D7; gates 1 (a-d), 2, 3, 4, 6, 8. 1324a never ducks for a
sidechain: D1's sidechain closure, D4's sidechain-line `C` and gates 7 and 9 are 1324b's. Gate 5's
send is in gate 1's session (`b-send`, `pre_fader` into `r`, with `C` from `a`'s limiter).

- **D1-D2, one filter** (`prepare.rs`, `StripRestart`). After every join, the restart set is the
  strip, effect and strip-delay joins' `restarted` plus a warm lead's `restart_whole`. One filter
  drops, for each strip in it, the fader's move and retarget, every effect move and retarget (a
  dropped retarget that opened an owner transaction discards it), its input-section move, every
  compensation line on its chain before the matrix, into its effects' sidechains and on its routes
  from a tap before the fader, and the live-lane carry of those routes. The matrix, post-fader and
  post-pan routes and their lines carry. The join functions are untouched; #1354a's scattered
  `restarts_whole` filters stay (they are a subset). The input section is filtered too: D2's "every
  owner before the fader" (R2 lists the families; the input section is one of them by its rule).
  The strip delay (source alignment and claim lines) is not: it holds raw source frames (D2's
  claim-line bullet). The input-carry install moved after the filter; its admission keeps the
  ceiling it charged.
- **D3** Every restarted strip is armed afresh with this plan's `D` (#1288's entry point,
  `fade_arms_for`), on #1288's terms (live lanes, no declared discontinuity); a pending #1392 arm of
  a restarted strip is not carried (its fader does not move), so the fresh arm replaces it. Each of
  its routes from a tap before the fader that binds a six-word op is armed too and waits for the
  strip (#1363 D1 (c)). Effective mute decides by construction: a lane prepared muted arms nothing.
- **D4** `plan_strip_transition(base, overlay, next, restarted)`: the duck set is the removed
  strips plus `restarted`, minus the overlay; `C` already spans every route out of a ducked strip.
  `control.rs` passes `prepared_runtime.inventory.restarted_strips()` (the call only).
- **Test inventory** `test_only_floors_only` now keeps the join rows (strip stages, input
  sections, effect lanes, strip delays): a successor of it restarts and arms nothing, and, rendered
  without adopting, still carries no state (#1285's floored reference).
- `graph::test_only_carry_counts` gains the compensation-line move count (root, from 1354a); 1354a
  gate 2's R6 assertion checks it (X's lines are all new; `s-main`'s carries without the bus
  limiter).

**Bits that move (class A outside the intended change).** Only on strips in `restarted_strips()`:
their owners before the fader and their fader no longer carry, and on a live plan they and their
pre-fader routes are armed. On the bank widths no other path changes. On the scalar oracle every
strip's fader is no bank lane, so every kept strip restarts at every swap (as before) and now
restarts and arms whole. Existing tests whose claim was a carry inside a restarted strip were
folded: `a_prepared_change_to_one_slot_restarts_only_its_strip` (all of `eq3`'s slots now stay),
`a_surviving_insert_restarts_its_strip` (its console-carry claim dropped),
`a_removed_console_slot_restarts_every_strip` (the 45-lane count dropped),
`an_added_insert_restarts_its_strip_and_its_console_slots_carry` (deleted: superseded by gate 4),
`a_changed_lane_delay_restarts_only_its_lane` (plain plans, so no arm hides the lanes), and
route_mute's `a_rebuild_that_adds_a_mute_entry_keeps_the_sends_ramp` and
`a_rebuild_that_hand_mutes_the_source_ramps_the_send_over_the_edits_length` (bank widths only: on
the scalar oracle the strip restarts, so its `pre_fader` send restarts with it).

**Known limits.** (1) A carried effect whose sidechain reads a ducked strip's pre-fader tap steps
at `S` (1324b; closed by the 1324b record below). (2) A pre-fader route that follows its
source's automated mute has no six-word op, so it cannot be armed: it restarts at its prepared
value at `S` (a step in). Arming a following route needs graph work outside this slice: #2011.
(3) A restarted submix's
`D` adds the longest line of any route into it, not only of a line that starts at rest (D3 says
the latter): its fade may start later than needed. (4) An insert's quality is no trigger any
session can reach: every launch effect declares only `normal` (gate 3 lists five triggers). (5)
The route live-lane filter is unobservable for an armed route (the arm settles the state the
carried, ducked lane holds); it decides only limit (2)'s case, and no test pins it.

**Tests (red first on a mutant, each).** Gate 4
`a_restarted_strip_carries_nothing_before_its_fader_on_every_width` (host-core); the Waiting-arm
assertion in `a_chain_restarted_strip_still_fades`; gates 1 (with 8),
2, 3 and 6 in `strip_transitions.rs`: `a_restarted_strip_ducks_then_fades_in_after_s`,
`an_untouched_strip_plays_through_a_duck_swap`,
`every_restarting_edit_ducks_and_a_send_or_value_edit_does_not`,
`a_revert_during_the_duck_fades_back_in_at_s`.

**A declared discontinuity ducks nothing** (found by the fallout below): a successor prepared at
a declared discontinuity restarts every owner, so its `restarted_strips()` lists every strip;
`plan_strip_transition` ignores `restarted` when `base.discontinuity` (#1323 D2), and such an edit
is adopted at the next block as before. Pinned by capi's
`an_edit_that_supersedes_a_discontinuity_successor_is_prepared_at_it`, red without the guard.

**Test fallout (root authorization, 2026-10-25).** A restarting rebuild now waits for `S`. Each
edit moves only the adoption point (render through `S`, or wait on the applied-revision watermark);
no equality, count or bit comparison was loosened. Shared helpers: control-plane
`tests/support/adoption.rs`, capi `feed_and_render_until_direct` (existing) and
`render_owner_until_swap`, host-web `Rig::step_through_adoption` and control_parity's
`step_both_until_applied`. Claims changed to the duck-swap expectation (the old claim was
next-block adoption of a strip-restarting rebuild):
- control-plane `capture_follows_newest_plan`: a capture armed before an insert-adding rebuild is
  now answered by the running plan without the insert (as for a removal, #1325); the stale branch
  keeps its claim with a rebuild that restarts no strip (a muted track added).
- capi `assert_rebuilds` (`a_set_control_on_a_rebuild_value_is_a_rebuild`): a `delay_samples`
  rebuild leaves exactly the duck's two mute writes on the restarted strip, no other cell write.
- capi `in_place_commands_equal_full_commands_through_the_c_abi`: compares every watermark word of
  the two sessions (supersessions now occur, alike on both paths) instead of requiring none.
- capi `edits_after_render_took_two_candidates_unretired_each_commit` and
  `two_taken_candidates_and_a_published_adoption_never_wedge_the_session`: their edits are now
  structural edits that restart no strip (tracks added), so render still takes each candidate at
  the next block, which those claims are about.

**Root cause of the in-place parity failure (fixed here).** The input join's `debug_assert`
("a compiled strip's input values have live records", `prepare.rs` input join) fired on unmodified
main too, inside a caught panic, whenever a transaction moved a high-pass above its strip's
low-pass: such a model compiles, the live path refuses its records, and the preparation's own
`builtin.filter.order` refusal comes later. On main no candidate was pending at that moment. With
1324a's longer phase 1 one was: the rebuild had withdrawn candidate A when it panicked, the capi
host caught the panic, and A (acked revisions) was dropped with the publisher's mark consumed. The
next edit's withdrawal found `Nothing`, promoted A as if render ran it, parked the running epoch
for good, and routed the source producer to a plan render never adopted: its ring filled
(`source.backpressure`). Acked-batch answer: yes, an acked candidate was lost, through the panic.
The fix removes the `debug_assert` (a refusal is the designed path: the section restarts, or the
preparation refuses), pinned by host-core's
`an_input_section_the_live_path_refuses_is_a_typed_refusal` (red with the assert back, and the
in-place test red with it). Left for root: any other panic inside a rebuild after the withdrawal
still drops the withdrawn candidate in a host that catches panics (a panic-safety question for
#1310's withdraw-then-rebuild order): #2010.

**SDK evals.** `sdk/test/structure-evals.mjs` waits a duck-swap out with one bounded helper,
`runUntil`: it renders until the state the test then asserts holds, within `duckBlocks` blocks
(`ceil((q + N + L)/q) + ceil(L/q) + 1`, `N` the session's mute ramp, `L` the plan latency measured
by an impulse, which bounds `C` and `D`), plus the tap's window where a fresh arm must publish.

**An input section the live path refuses is no click.** Every value the live input path refuses
(trim outside -144..24 dB, a cutoff outside its range, a high-pass at or above its low-pass) is
refused by the successor's own preparation with the same rules (`builtins::prepare_input_track`,
the same SVF design), so the rebuild is refused and the running plan plays on. The other branches
that restart an input section (a rate or control-kind change) are refused by both hosts or occur
only on a plain plan, which has no duck and no arm. Listing such strips in `restarted_strips()`
was tried: it changes only plain plans (three host-core tests restart their strips whole, with no
duck to cover it) and nothing a live host renders, so it was not kept.

## Slice 1324b record (implementer, 2026-10-25)

Scope (scope note): D1's sidechain closure, D4's sidechain terms of `C` (with the submix
`input`-tap exemption), gates 7 and 9. Closes 1324a's limit (1).

- **D1** (`prepare.rs`, `close_over_sidechains`): after the joins and the warm lead's whole
  restarts, a strip of both plans with an insert keyed (successor model) from a restarted strip's
  `post_input`, `insert_send`, `insert_return` or `pre_fader` tap joins the restart set, repeated
  until no strip joins. A `post_fader`/`post_pan` tap adds nothing. An `input` tap (track or
  submix) adds its consumer when its strip's delay line restarts (`strip_delay_carry.restarted`,
  kept before the merge; root amendment of D1, attempt 2); otherwise a track's adds nothing, and a
  submix's `input` tap adds its consumer only when the sidechain's compensation line changes length
  (predecessor's line against the successor's; the line-length form of "the stage arrives at its
  predecessor arrival plus `P`") and some line into the stage, in either plan, comes from a strip
  neither ducked (removed or restarted) nor added (`submix_input_is_quiet`). A successor route
  into the stage from a tap before the fader that follows an automated source mute (unarmable,
  limit 2, #2011) makes the stage not quiet, so its consumer ducks rather than read the route's
  step at `S`. The whole-strip filter,
  the arm and `restarted_strips()` then treat a joined strip as any restarted strip. The closure
  is not order-free when the exemption flips on a later round (a strip that joined earlier stays);
  that only ducks more, never less.
- **D4** (`transition.rs`): `C` also takes, into each carried strip (in `next`, not restarted, not
  in the overlay), every sidechain line from a now-ducked strip's `post_fader`/`post_pan` tap, and
  for a now-ducked submix whose `input` tap keys it under the exemption, that line plus the longest
  route line into the submix (running plan's lines).
- **Warm lead.** `warm_lead`'s `R` is the ordinary successor's `restarted_strips()`, so it is now
  closed over sidechains for its join part. A consumer of a strip restarted whole is closed only
  in the warm successor itself, after the floors (`warm_lead` doc limit; #1354b).

**Bits that move (intended change only).** A carried strip keyed from a restarted strip's tap
behind a restarted owner (or, outside the exemption, from a restarted submix's `input` tap) now
ducks, restarts before its fader and fades in; a duck-swap with a sidechain line of the D4 kinds
waits longer for `S`; a warm lead's `R` holds such closure strips, which take floor `a` instead of
`a + P`. No existing test changed: host-core, capi, control-plane and host-web suites pass
unmodified.

**Limit 3 (a restarted submix's `D` counts every input route's line)** is D3's arm delay, not the
`input`-tap part (D1's exemption and D4's `C` term); it stays open, unchanged by this slice.

**Widths (attempt 2, root's MINOR 4).** Gate 7's set membership is checked at `Simd8` and
`Simd4` in host-core; its rendered bits are checked through the C entries at the build's width
only. The bits at the other width rest on the banking claim (banking regroups lanes and never
changes per-lane arithmetic, so a placement or width change moves no bit); no per-width bit check
was added.

**Tests (each red first on a mutant).**
- capi `a_sidechain_consumer_ducks_with_its_restarted_source` (gate 7 (a)(b) through the C
  entries): red with the closure off (block 7 differs from the reference that mutes `a`) and with
  a track's `input` tap joining (block 7 differs from the run with no transaction).
- capi `a_shortened_sidechain_line_from_a_restarted_submix_drops_no_fade_sample` (gate 9; attempt
  2 gives `t2` a limiter and adds a removed plain `t4` into `bu`, so the line into `bu` is `L` and
  `C = 2L`): red with `C`'s sidechain terms off, with the exemption off, and with the
  line-into-the-submix term zeroed (`into * 0`), each with `S` 3584, not 4096.
- host-core `a_strip_keyed_from_a_restarted_strip_restarts_with_it_on_every_width` (gate 7's
  membership at `Simd8` and `Simd4`; rows `insert_return`, `post_input`, `insert_send`, `input`,
  `input` behind a restarted delay line, `post_fader`, a two-round chain, a submix `input` key
  whose line keeps its length, and one whose line shrinks while carried `aux` feeds the bus): red
  with the closure off, after one round, with `post_fader` joining, without the line-length
  check, with a track's `input` tap joining, with the delay-line arm off (`["eq0"]`), with
  `post_input` and `insert_send` mapped to false, and with the submix arm `false &&`
  (`["bus", "late"]`). Folds after attempt 2's PASS: a row whose bus input is only `late`'s
  `pre_fader` route following its stored mute automation, red with the following-route clause
  `false && ...` (`["bus", "late"]`); a row with the bus's `delay_samples` changed, red with the
  delay-line arm limited to tracks (`["bus"]`); and the exact arm count (each audible restarted
  strip armed, the muted `late` not).
- host-core `transition::tests::c_counts_a_post_fader_sidechain_line_into_a_carried_node` (D4's
  `post_fader` term; feature `control-provider`): red with that term off (`C` 0, not 486).

