# Skip the wet path of a bank whose every lane is bypassed

Filed by root for the console sprint (2026-10-26). Draft; GitHub number to be assigned.

## Owner decision (2026-10-26, verbatim)

"If we can't apply the transparent passthrough default for all effects, let's not apply that to
any effect and go with actual bypass requirement for transparency."

Context: the owner was told that a bypassed effect costs slightly more CPU than an enabled one
(the wet path plus the shunt select). The owner wants bypass to be real passthrough. Effect
parameter defaults do **not** change. This decision amends decision 12's clause "a bypassed lane
runs the wet path; that cost is accepted" (`docs/rulings/engine-footprint-2026-09-29.md:76-80`)
and lifts amendment 11's "an all-lanes-bypassed skip stays out of scope" (`:90-92`). Recorded as
an **owner decision**, dated 2026-10-26, under the ruling's amendment 15.

## Current behaviour (verified on main 0f6a1734e)

- A session bypass is lowered to a prepared `bypass = false` plus a per-lane `BypassShunt`
  (`crates/effect-contract/src/lib.rs:1535-1563`; the clause "**The wet path still runs.**" is
  `:1547-1548`). The shunt type is `crates/effect-contract/src/live.rs:2230`; its lane-side state
  (`bypassed`, `bypass_ramping`, `needs_dry`, `apply_bypass`, `advance_bypass_mix`) is
  `live.rs:1170-1250`.
- The rack's `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:1457-1486`) captures the dry
  block into the shunt (`:2760-2766`), runs `process_bank` for every lane (`:2767-2782`), then
  bit-selects or crossfades the latency-matched dry into exactly the bypassed or ramping lanes
  (`:2850-2869`). Nothing skips a bank whose lanes are all bypassed.
- The delay never banks (`NEVER_BANKED_EFFECTS`, `crates/effect-compiler/src/prepare.rs:293`) and
  renders a per-node shunt in `crates/graph/src/runtime.rs:5890-5945` (`LiveControlEffect`,
  `:3054`): capture, `process`, `apply_bypass`.
- The multiband banks a mixed-bypass cohort (#1340) through the same bank stage; nothing special.
- The silence skip (#1107) is chain-wide: `BankChain::process` skips a block only when every input
  is `+0.0` and `lanes_at_rest` covers every lane (`rack/src/lib.rs:4118-4126`, `skip_block`
  `:4232-4270`, counter `silent_skips` `:3453`, `:5161`). The stage's `lanes_at_rest` **clears
  every lane that reads dry** (`:2422-2425`), so today a bypassed lane keeps its bank rendering even
  on silence.
- Reset kinds: `ResetKind { FullToDefaults, DiscontinuityKeepParameters }`
  (`effect-contract/src/lib.rs:449`). `DiscontinuityKeepParameters` is designed for the render
  thread ("a reset can reach this from a seek or a transport stop, and the render thread has no
  allocator", `crates/soft-clip/src/lib.rs:577-579`); it clears histories and snaps ramps to their
  targets. The bank trait has whole-bank `reset` only (`lib.rs:2429`); per-lane resets exist
  inside effects for D7 recovery (`soft-clip/src/lib.rs:610`, `true-peak-limiter/src/lib.rs:818`)
  but are not on the trait.
- Launch latencies: the true-peak limiter `N + 6` with `N = rate/100` (`true-peak-limiter/src/lib.rs:11`,
  `:287`), the soft clip 31 (`soft-clip/src/lib.rs:240`); every other launch effect, the delay
  included, declares 0.

## Product contract

1. **The skip.** When every lane of a bank (or the one lane of a per-node effect) is bypassed and
   settled (`bypassed() && !bypass_ramping()` for every lane; padded lanes count as settled), and
   the block stages no automation span, no prepared target and arms no observation tap on any lane,
   the wet path does not run for that block: no `process_bank`, no `process`. The output is exactly
   the latency-matched dry the shunt gives today, so the bypassed output stays bit-identical to the
   current one. The shunt's latency line stays fed.
2. **No coupling.** A bank with any enabled, ramping or holding lane (contract 3) runs as today.
   Banking couples lanes' cost, never their bits (decision 12 M2). A session that bypasses nothing
   renders the byte-identical path (`EffectBankStage` is untouched).
3. **Un-bypass after a skip.** A skipped bank's effect state is stale. The resolution (decision
   record D1-D4): the bank is reset to rest at resume, and a lane whose un-bypass resumes the bank
   holds dry for the effect's declared warm-up before its crossfade starts.
4. **Decision 12 amended.** "The wet path still runs" is false for an all-bypassed, settled bank.
   Owner decision of 2026-10-26, above.
5. **Classification unchanged.** Bypass stays live, smoothed (the session mute ramp) and
   automatable, kind 6 on the browser, exactly as `docs/rulings/live-update-versus-rebuild-2026-10-04.md:132`
   and D15 E5 record.

## Decision record (root, 2026-10-26)

- **D1. Skip predicate and output.** In `LiveControlEffectBankStage::process` (and its
  `process_mono` and `process_from_left` forms), after the drain and the target application: skip
  iff every lane is settled-bypassed, `walk.targets` is false, no lane has a staged span, and no
  lane has an armed tap whose bypass value is `None` (D5). The skipped block's output is the shunt passthrough: at zero latency
  nothing is written (the planes already hold the dry words; no capture, no apply, two block
  copies saved); at nonzero latency the block is exchanged in place through the latency line
  (`BypassShunt::passthrough`, new: the line exchange of `capture` without the `dry_*` staging,
  which no reader needs on a skipped block). Both produce the words `capture` + `apply` produce
  today, by copies, so `-0.0` survives. A collapsed block passes the left plane only, as the body
  does; the seam slot writes right = left. A new per-stage `bypass_skips` counter (`u64`,
  saturating, like `silent_skips`) counts skipped blocks and is summed on `BankChain`.
- **D2. Staleness and the resume reset.** A stage holds `stale: bool`, set by the first skipped
  block. On the first block after a skip that runs the bank for any reason (a lane's switch leaves
  bypass, a staged span, a prepared target, an armed tap whose bypass value is `None`), the stage calls
  `reset(DiscontinuityKeepParameters)` on the whole bank, then applies that block's targets, then
  runs. The reset is whole-bank because the skip is whole-bank: every lane of a stale bank was
  bypassed throughout, so no lane has state worth keeping. Parameters are kept: a span drained
  while stale ran the bank (D1), so every parameter value a lane holds is current, and the reset
  snaps ramps to those targets. `reset` runs at a block boundary on the render thread, is
  allocation-free by contract (`soft-clip/src/lib.rs:577-579`), and is bounded by state size, like
  the D7 recovery and the swap carry. **A per-lane reset in a bank that was not skipped is neither
  possible on the trait nor needed:** a lane in a running bank kept running and is not stale.
- **D3. Warm-up (the hold).** A lane whose switch leaves bypass on the block that resumes a stale
  bank holds settled-dry for `W` frames before its crossfade runs: its crossfade lane reports
  `settled_dry` while a per-lane `hold` counter is above zero, and the ramp starts when it reaches
  zero. `W` is the effect's declared warm-up: a new `PreparedNativeEffectBank::warmup_samples()`
  (and the per-node twin) with the default `metadata.latency`, overridden where a FIFO structure
  holds more history than the latency: the true-peak limiter `T + N` (the lookahead ring plus the
  box smoother: at `T` the ring is full, at `T + N` the box holds only post-reset gains, so the
  ceiling guarantee holds from the first crossfaded sample), the soft clip `latency + tail` (31 +
  29 = 60; the implementer confirms against the exact-rest ring layout, `soft-clip/src/lib.rs:258-284`,
  and raises it to 62 if the gate asks). Every zero-latency launch effect keeps the default 0:
  its rest state is a fixed point and its zero history is the same history it has at a session's
  first sample (the EQ's filter transient, the dynamics' detector attack, the delay's first echo),
  and no finite bound exists for an IIR state. A lane un-bypassed in a bank that is already
  running (not stale) takes no hold: today's ramp, bit-identical to E5 (#1341).
  - **Why not crossfade at once.** The dry side is delayed by exactly `latency` and carries
    signal; the wet side's first `latency` samples are the line's zeros, and for the soft clip the
    half-band FIR's past taps hold zeros until `latency + tail`: a crossfade there dips by up to
    the FIR's missing weight (about 2.5 dB on DC for 31 samples) or, for the limiter, by the
    whole signal for `T` samples, with no ceiling guarantee for the next `N`.
  - **Why not prime the state from the shunt line.** The line holds `latency` dry samples, not
    the `T + N` or `latency + tail` an effect needs, and running extra samples inside one block is
    unbounded for a long line (the delay). A hold is bounded, allocation-free and exact.
  - **Against D15-2 and the watermark.** The switch's cell drains at block `B` (first sample
    `s_B`), the first block that begins after the submit returns (or the adopting block while a
    replacement plan is pending). The revision is *applied* at `B` and the watermark covers it
    there, exactly as today: the wet path starts at `s_B`, the committed model holds the final
    value, and a superseding write still counts in `live_values_superseded`. The hold is part of
    the switch's smoothing law for a lane resumed from rest, as the mute ramp is part of the
    mute's: the output is dry through `[s_B, s_B + W)`, the crossfade `out = dry + (wet - dry) m(k)`
    runs from `s_B + W` on the `IndexedRamp` law (`k = position + f + 1`), so **un-bypass is first
    audible at sample `s_B + W + 1`** (the first frame with `m(k) > 0`) and complete at
    `s_B + W + ramp_samples`. The watermark is never early and never claims audibility. Root
    records this as its reading of D15-2 and flags it for owner review in the verdict, with the
    alternative (a per-lane "audible-at" field on the watermark) named and not taken.
- **D4. Plan swaps and lane moves.** The carry (`rack/src/lib.rs:1800-1835`, `:1925-2005`) resets a
  stale source bank (`DiscontinuityKeepParameters`) before it snapshots any lane, so a carried
  payload is at rest with current parameters whether the successor skips or runs, and the shunt
  line carries as today. A render-time lane swap (`swap_bank_lanes`, #1782a) and #1981's packing
  move only lanes `lanes_at_rest` reports, and the stage never reports a lane that reads dry
  (`:2422-2425`), so a stale lane never moves; a stale bank is also not a packing destination
  (#1981 adds the term when it lands). The D7 block-boundary check never fires on a skipped block
  (no wet words); a resume reset clears any state it would have read.
- **D5. Armed taps publish the bypass value; they do not block the skip** (root, 2026-10-26,
  amending the draft). The browser console arms the gain-reduction tap (tap 1) of the gate and the
  compressor on every strip for the whole session (`misofm/studio app/src/studio/gain-reduction.ts`,
  `engine-client.ts:332-350`), so a tap that blocks the skip would keep every gate and compressor
  bank running and the issue would save almost nothing on the shape it exists for. An observed value
  must describe the block emitted (`graph/src/runtime.rs:5940-5944`); a skipped bypassed lane
  emitted dry, so the effect applied no gain reduction, and the true observation is the tap's
  bypass value. Each effect tap declares it: a new `bypass_observation()` on the tap descriptor,
  either `Constant(v)` (the gate's and the compressor's gain-reduction taps: `0.0` dB, and every
  other launch gain-reduction tap the implementer finds) or `None`, which keeps today's rule (an
  armed `None` tap keeps its bank running). On a skipped block the stage folds the constant into
  each armed lane's window exactly as a computed value is folded, so window peaks, cadence and
  `observedSample` are unchanged in shape. This also changes what a bypassed lane observes in a
  bank that is *not* skipped: today it reports the gain reduction the wet path would apply; it now
  reports the bypass value too, so a lane's observation never depends on whether its bank mates are
  bypassed (banking couples cost, never bits, and never observations). The console already hides
  the reading of a bypassed slot, so no app change is needed.
- **D6. Silence and bypass** (root, 2026-10-26, from the banking audit,
  `scratchpad/bank-audit/report.md` finding 2). Today `lanes_at_rest` clears every lane that needs
  dry (`rack/src/lib.rs:2413-2432`, `live.rs:1184-1186`), so one bypassed lane stops #1107's
  silence skip for its whole bank, and the console, which loads with every slot bypassed, never
  takes that skip. Change: a settled-bypassed lane counts as at rest once its latency line holds
  only `+0.0` (always true at zero latency once its input block is `+0.0`), because its output is
  then exactly `+0.0` either way. A partly bypassed bank on silence then skips through #1107 when
  its enabled lanes are at rest, and an all-bypassed bank skips through D1. The line's all-zero
  state is tracked by a bounded per-lane counter of trailing `+0.0` samples fed by capture and
  passthrough, not by a scan. Gate: one `crates/rack` row, a bank with one bypassed lane on
  silence reaches `silent_skips > 0`; red on today's predicate.
- **D7. The two never-banked effects, `miso.delay` and `miso.reverb` (S2)** (`NEVER_BANKED_EFFECTS`,
  `crates/effect-compiler/src/prepare.rs:295`). `LiveControlEffect` (`graph/src/runtime.rs:3076`,
  rendered in the `NodeKind::LiveControlEffect` arm of `execute`, about `:6271-6345` on the #2142
  branch) takes the same rule with one lane: skip when settled-bypassed with no span, target or armed `None` tap; at zero
  latency no capture and no apply; `stale` and the resume `reset(DiscontinuityKeepParameters)` on
  the `PreparedNativeEffect`; the hold through the same `EffectControlLane` logic S1 builds (the
  delay's `W` is 0, so the hold is a no-op and the first echo starts at un-bypass). A per-node
  `bypass_skips` counter beside `silent_skips` (`:3648`). Decision 15 E4's shunt-over-tripped-ring
  rule stays: a skipped block runs no D7 check, and the resume reset clears a tripped ring.

## Slices

- **S1 (this issue): the banked skip, the resume reset, the warm-up hold.** Rack stage, lane
  hold, shunt passthrough, trait warm-up with the two overrides, the identity row, the counter
  tests, the warm-up gate, the CPU row, the rulings. Closable in half a day of implementation
  plus one review.
- **S2 (this issue's last slice): the per-node skip for the delay and the reverb** (D7), reusing
  S1b's lane hold. S2 closes #2129.
- Not in scope: a per-lane silence skip (a bank skips as a whole), any change to
  effect parameter defaults, any change to the bypass classification or ramp length.

## Gates (one test per claim, red first on a named mutant, in the owning crate)

1. **Bit-identity.** The all-lanes-bypassed cohort over the fixture, as a row of
   `crates/graph-compiler/tests/bypass_shunt_identity.rs` against its prepared-bypass oracle (today's
   shunt words by construction), with the skip counter asserted above zero on the row so the row
   exercises the skip. Red on a passthrough that writes the staging words, drops `-0.0`, or feeds
   the line from the wrong plane. PR evidence, not a committed digest: one before/after `to_bits`
   comparison of the console fixture's rendered output with every console slot bypassed.
2. **Realtime.** The existing cross-host render zero-allocation test and the realtime-policy tool
   cover the new code, which sits inside the stage's existing `REALTIME_POLICY` regions (the tool's
   region count changes only if a region is added; record it).
3. **The kernel is not called.** `crates/rack` unit test, the `silent_skips` model (`:8027-8036`):
   an all-bypassed bank skips `BLOCKS` blocks (`bypass_skips == BLOCKS`), a bank with one enabled
   lane skips none, a bank with a staged span, a prepared target or an armed `None` tap on a bypassed
   lane skips none that block, and a bank whose only armed tap is a `Constant(0.0)` gain-reduction
   tap still skips and folds `0.0` into that lane's window. Red on a skip that ignores a span or a
   `None` tap, on a skipped block that leaves the window unfolded, or on a predicate that reads
   `bypassed()` without `bypass_ramping()`.
4. **Resume.** `crates/rack` unit test with the test double: after a skip, the block that
   un-bypasses lane 0 runs the bank from a reset state (the double records `reset(kind)` once with
   `DiscontinuityKeepParameters`, then `process_bank`), a parameter staged during the skip is still
   the lane's value after the reset, and a lane un-bypassed in a never-skipped bank sees no reset.
   Red on a missing reset, a `FullToDefaults`, or a reset on every block.
5. **Warm-up: no dip, no NaN, no denormal.** `crates/graph-compiler/tests` (the crate that holds
   the launch registry beside gate 1; rack has no effect dev-dependency), for the true-peak limiter
   and the soft clip at least, at 48 kHz: lane 0 bypassed for 16 blocks (skipped), then un-bypassed
   with the session ramp; the reference is the same lane never bypassed. Assertions: no NaN and no
   subnormal in the output; the output is bit-identical to the dry through `s_B + W`; from
   `s_B + W + ramp_samples` the output is bit-identical to the reference on a sub-threshold
   fixture (the limiter's gain is exactly 1 on both sides, the soft clip's FIR holds only post-reset
   input); and over the crossfade the output is never below `min(dry, reference)` per sample by
   more than 1e-6. A second limiter fixture with overs asserts no sample above the ceiling after
   `s_B + W` (the box is primed). Red on `W = latency` for either effect (the dip), on a hold that
   runs on a never-skipped lane (the reference leg's E5 identity breaks), or on a limiter `W` of
   `T` alone (the ceiling row).
6. **CPU row (descriptive).** `tools/bench console` on a new `sixteen_track_strip_bypassed`
   workload: the standing sixteen-track strip (`SixteenTrackStrip`, two full eight-lane banks,
   `tools/console-workload/src/lib.rs:416-419`) with every console slot bypassed on every track;
   one run, one warmup, two rounds, before (main) and after, x86-64 AVX2. Recorded, not gated.

## Paths (S1)

- `crates/effect-contract/src/live.rs`: `EffectControlLane` hold counter and its crossfade-lane
  and `needs_dry` terms; `BypassShunt::passthrough`.
- `crates/effect-contract/src/lib.rs`: `warmup_samples` on `PreparedNativeEffectBank` and
  `PreparedNativeEffect` with the latency default; rewrite of the "wet path still runs" clause
  (`:1535-1563`).
- `crates/rack/src/lib.rs`: the stage's skip in its three process forms, `stale`, the resume
  reset, the carry reset (D4), `bypass_skips` and its chain sum; unit tests (gates 3, 4).
- `crates/true-peak-limiter/src/lib.rs`, `crates/soft-clip/src/lib.rs`: the `warmup_samples`
  overrides with their derivations in the module docs.
- `crates/graph-compiler/tests/bypass_shunt_identity.rs` (gate 1 row) and a new
  `crates/graph-compiler/tests/bypass_skip_warmup.rs` (gate 5).
- `tools/console-workload/src/lib.rs`, `tools/bench/src/console.rs`: the bypassed sixteen-track
  row (gate 6).
- `docs/rulings/engine-footprint-2026-09-29.md` (amendment 15: the owner decision, the clause at
  `:76-80` and amendment 11 at `:90-92`), `docs/rulings/live-update-versus-rebuild-2026-10-04.md:132`
  (the reason column: "latency is always paid; an all-bypassed, settled bank skips its wet path"),
  `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md` (a note under E5:
  the hold on a lane resumed from rest, with the audible sample), `docs/REALTIME_DEPENDENCY_POLICY.md`
  only if the reset-at-resume needs a sentence, this spec.
- `Cargo.lock` only if graph-compiler gains a dev-dependency (it should not).

## Overlap with in-flight work

- **#2114 (reverb, new crate, registered in `effect-compiler/src/prepare.rs`).** S1 does not touch
  `prepare.rs`. The trait default (`W = latency`) covers the reverb with no edit; if its pre-delay
  is a FIFO longer than its declared latency the reverb overrides `warmup_samples` in its own
  crate. Merge order is free; the later PR rebases over the trait addition, a one-line conflict at
  most if #2114 adds a trait method of its own.
- **#2128 (loudness SDK, `sdk/` only, plus `crates/protocol/tests/sdk_message_fixture.rs` and
  `hosts/host-web/qualification/protocol-calls.mjs`).** Disjoint file sets. #2128 does not move
  the worklet module (TypeScript only); S1 does.
- **#1981 (packing).** D4 names the term #1981 adds (a stale bank is not a destination). No shared
  file edit in S1; #1981 edits `rack/src/lib.rs` when it starts, so it rebases over S1 or declares
  the edit order.
- #2115 S1/S2 touched `true-peak-limiter/src/lib.rs` (the `annex2` visibility); merged, no
  conflict.

## Release window

The rack, effect-contract, limiter and soft-clip changes move the AudioWorklet module bytes (as
#2123's doc lines did, `.github/ISSUE_SPECS/2121-release-misofm-engine-0-6-0.md:24-25`). Per the
release-window rule, this PR does not merge between a release PR's merge and the verified npm
publish; 0.6.0 is closed, so the next release (#2121's successor) re-pins `EXPECTED_WORKLET_SHA256`
and re-records the two-browser qualification over the new module, and S1 states the new digest in
its evidence.

## Open for the owner

- D3's reading of D15-2 (applied at `B`, audible at `s_B + W + 1`), recommended as written.
- D5 (an armed tap publishes its declared bypass value, so it does not keep a bypassed bank
  running), amended by root.

## Evidence record

### S1 attempt 1 (2026-10-26), paused at a checkpoint

Paused by the coordinator: a design review found that D2/D3 make a lane's reset and hold depend on
whether its bank mates were bypassed too, which couples bits across a bank. Root is amending the
ruling to per-lane staleness (`reset_lane` on the bank trait, every un-bypass resets and holds);
the work splits into S1a (`reset_lane` for every banking effect) and S1b (the skip on per-lane
staleness). This checkpoint implements D1-D6 as first written and is not the final shape.

Done on this branch (per the original D1-D6):

- D1 skip in `LiveControlEffectBankStage` (`skips_wet_path`, `pass_bypassed`,
  `BypassShunt::passthrough`), `bypass_skips` per stage and summed on `BankChain`; padded lanes via
  a new `BankStage::bind_active_lanes`.
- D2 whole-bank resume reset; D3 per-lane hold (`EffectControlLane::begin_hold`, the bank
  crossfade's `k < 1` dry select), `warmup_samples` on both effect traits (limiter `T + N`, soft
  clip `latency + tail = 60`); D4 `settle_for_carry` from `disengage_for_carry`; D5
  `ObservationDescriptor::bypass_observation` (`Some(0.0)` on every launch tap) folded for every
  settled-bypassed lane; D6 per-lane zero run of the shunt line in `lanes_at_rest`.
- Gates: 1 (`bypass_shunt_identity` skip count), 3 and 4 (`crates/rack/tests/bypass_skip.rs`), D6
  rows in `crates/rack/tests/lane_rest.rs`, 5 (`crates/graph-compiler/tests/bypass_skip_warmup.rs`).
  Each red on named mutants, except as listed below.
- One-time bit identity against the branch base 0f6a1734e (not committed): the master output of
  `sixteen_track_strip_bypassed`, the same with the product's live lanes, `sixty_four_track_app_shape`
  (with and without live lanes) and `sixteen_track_strip`, 400 blocks each: byte-identical
  (sha256 28ba0504...e1e9f both sides).

Findings for the rebrief:

- The gate 5 mutant "limiter `W = T`" stays green, and so does `T + 1`: after a reset the box ring
  holds only post-reset `m_q` once `n >= s_B + Wb` and `Wb <= N + 1 <= T`, so `g[n] <= r[n-N]`
  holds from `s_B + T`. `T + N` is safe but appears conservative by `N`; the gate's claim that `T`
  fails the ceiling row could not be made red.
- Gate 5's soft-clip `W = 59` is not distinguishable from 60 at the 1e-6 dip tolerance; `W = 32` is
  red.
- Gate 5 stays green with the resume reset removed (the skipped bank's frozen state is not seen
  through the hold at this fixture); gate 4 is the reset's red row.
- Not started: gate 6 CPU row (the workload variant exists, not yet emitted by `tools/bench`),
  the rulings amendments, the realtime-policy tool run.

## Decision record amendment (root, 2026-10-26): per-lane staleness; split into S1a and S1b

- **Finding (design review, BLOCKER B1):** under D2/D3 as first ruled, a lane is stale only when
  its whole bank skipped. So whether an un-bypassed lane gets the reset and the hold depends on
  whether its bank mates were bypassed and on its cohort, and its bits depend on its bank mates.
  That breaks AGENTS.md's "banking couples cost, never bits".
- **D2 and D3 amended.** Staleness is a per-lane (per-strip) fact. Every un-bypass from settled
  bypass resets that lane alone (`reset_lane(lane, DiscontinuityKeepParameters)`, #2131) and holds
  dry for `W`, whether or not its bank ran while it was bypassed. A bypassed lane in a running bank
  still computes, but its state is discarded at un-bypass. The whole-bank skip of D1 is unchanged;
  it is now purely a cost decision.
- **Per-lane state travels.** The stale flag and the hold counter move with the lane in every lane
  swap (#1782a, #1981's packing, cross-stage swaps included; `swap_stale_bits` today drops the bit
  across stages) and in the plan-swap carry.
- **Limiter warm-up.** The S1 implementer showed that the ceiling bound holds from `s_B + T`: after
  a reset the box ring holds only post-reset values once `n >= s_B + Wb`, and `Wb <= N + 1 <= T`.
  So `W = T + N` is about 10 ms longer than needed. Ruling: `W` is the least value that the stated
  derivation proves, recorded in the module docs, with the gate's red row at `W - 1` where that row
  can be made red. If no row can, the derivation is the record.
- **Slices.** S1a is #2131 (`reset_lane` on every banking effect; it lands first). S1b is this
  branch: the skip, per-lane staleness, travelling state, and the remaining S1 gates (the CPU row,
  the rulings amendments, the realtime-policy tool, the zero-allocation render test). S1b merges
  main after #2131 and #1981 (PR #2130) land. S2, the per-node delay, is unchanged.

## Owner decision (2026-10-26): un-bypass from rest

Root asked: "When you turn a bypassed effect back on, it starts fresh ("from rest"). It does not continue from where it was. [...] Reply yes or no." The owner answered, verbatim: "Yes". This approves the per-lane un-bypass law in the amendment above (every un-bypass from settled bypass resets that lane and holds dry for `W`, then crossfades). It is class B against E5 (#1341), where a bypassed lane kept computing and un-bypass continued warm.

### S1b attempt 2 (2026-10-26), on the amended D2/D3

Branch `codex/batch-b13-bypassskip`, with origin/main merged twice (8f32d1673 for #2131's
`reset_lane`, then 7e824290f for #2145 and #2133/#2146). The final head is the one pushed with this
record.

Implementation:

- **Per-lane staleness (amended D2/D3).** `EffectControlLane::stage` flags a switch out of
  **settled** bypass (`rest_pending`; a reversal before the crossfade toward bypass settles does
  not). The rack's drain (`read_cells`) then calls `reset_lane(lane, DiscontinuityKeepParameters)`
  on that lane alone and `begin_hold(W)`, before the block's targets and spans, whether or not its
  bank skipped. The whole-bank `resume`, the stage's `stale` mask, `swap_stale_bits` and
  `settle_for_carry` are removed. The flag and the hold live in the control lane, so they move with
  the lane in `swap_lanes` (same-stage and cross-stage, #1981's packing included) and in
  `carry_from`.
- **Per-node parity.** `runtime::LiveControlEffect` takes the same law (the instance's `reset`,
  then the hold through `BypassShunt::crossfade_held`), and folds a settled-bypassed instance's
  declared tap value (D5), so a placement change moves no bit. `bank_equals_node` holds with the
  hold active (the limiter's 486-frame hold ends inside a block).
- **Limiter `W = T`** (the trait default, the declared latency); derivation in the crate's module
  docs. Soft clip `W = 60`, derivation in `soft_clip::warmup_samples`.
- **M1.** Every `reset_lane` trait body and the per-lane helpers it calls, in the nine banking
  effects, and every per-node `reset` (ten effects) sit in `REALTIME_POLICY` regions: the tool
  reports 234 regions on main 7e824290f and 283 on this branch.
- **M3** (optional limiter oracle) not done.
- `Runtime::bank_slot_runs` subtracts `BankChain::bypass_skips`.

Red evidence (each red on the named mutant, then green on revert):

| Gate | Test | Mutants |
|---|---|---|
| B1 | `bypass_skip_warmup::a_lanes_un_bypass_does_not_depend_on_its_bank_mates` | red on the tree before the amendment (compressor, sample 2048: `-0.0066666724` after a skipped bank against `-0.0066666515` after a running one); red with the drain's reset and hold disabled |
| 4 | `rack/tests/bypass_skip.rs::a_lane_leaving_settled_bypass_is_reset_alone_keeping_parameters` | no lane reset; `FullToDefaults`; a whole-bank reset in its place; the rest flagged on every switch to wet (a reversal resets) |
| 5 | `bypass_skip_warmup::a_resumed_lane_holds_dry_for_its_warmup_then_crossfades_onto_the_reference` (exact bits, ramp and step legs) | limiter `W = T - 1`; soft clip `W = 58`; no hold; the bank crossfade ignores the hold |
| 5 | `...a_resumed_limiter_keeps_its_ceiling_from_the_first_wet_frame` | no hold; the bank crossfade ignores the hold |
| runs | `silent_bank_skip::bank_slot_runs_follow_the_skips` (bypassed-slot leg) | red before the subtraction (9 runs counted where 6 banks ran) |

Gates 1, 3 and the D6 rows are unchanged from attempt 1 and pass. `W - 1` for the soft clip (59)
cannot be made red by any output word: the reset word the frame still reads is weighted by the
product of the two outermost half-band taps (about `1.7e-9`), below half an `f32` ulp; `W - 2` is
red, so the derivation is the record. The limiter's ceiling row is not red at `T - 1` (its extra
frame is the ring's reset `+0.0`); the crossfade row is.

The C ABI's `a_live_bypass_crossfades_over_the_session_mute_ramp...` lift rows are rewritten: a
lift resets the effect, so no never-bypassed control is its target; they now hold `D` through the
warm-up and require the per-node and banked lifts to agree bit for bit. `graph`'s
`live_bypass_is_latency_preserving_and_reversible` expects the two held dry frames.

Gate 6, CPU row (descriptive; `SessionMeasurement::run_for` at the frozen 1000 observations, one
warmup and two measured rounds, pinned to one core, x86-64 AVX2, through a scratch uncommitted
test; "before" is origin/main 7e824290f with only the workload variant added):

| Row | Before p50 / p95 ns per block | After p50 / p95 ns per block |
|---|---|---|
| `sixteen_track_strip` (round 1, round 2) | 23475 / 29286, 23696 / 29486 | 23765 / 24086, 23765 / 24076 |
| `sixteen_track_strip_bypassed` (round 1, round 2) | 30498 / 36559, 30539 / 36630 | 6492 / 6573, 6453 / 6532 |

Every row: 0 render errors and 0 forbidden render operations (the bench's audited allocator). Output
digests before and after are identical (`sixteen_track_strip` 00b84736...4988,
`sixteen_track_strip_bypassed` 6841e080...a2fa5): the one-time bit identity of the bypassed render.
`tools/bench console` now emits the row (80 records; the validators and the runner know the kind).

Gates run: the lint job's every step (fmt, clippy, doc with `-D warnings`, workspace policy and its
mutation tests, lane policy, unfused seal, rack, effect-runtime and the other hermetic scripts,
the three lane-guard builds); the realtime-policy tool; CI's workspace debug test step with its
features (host-web's render-lock probe and host-core's zero-allocation render tests included);
the DSP crates, graph-compiler, bench and console-workload in release;
`scripts/test-console-benchmark.sh`.

### S1b attempt 3 (2026-10-26): the review's two test gaps, the counter walk, the W=4 re-pin

- **MAJOR 1, C ABI lift rows** (`capi` `a_live_bypass_crossfades_over_the_session_mute_ramp...`):
  each lift row now requires words past `W` plus the ramp that are not the dry control's. Red on
  an acked lift that stays dry (the lane's `stage` ignoring a switch to wet), run per effect: the
  compressor, the soft clip, the delay and the multiband each fail "the lift was acked but its
  output stayed dry"; green on revert.
- **MAJOR 2, D6 across lane moves** (`lane_rest::the_live_stage_rests_only_lanes_with_no_work_and_skips_exactly`,
  extended): a bypassed lane whose line holds signal (right plane only) is moved into lane 1 by a
  swap in the stage, a carry from a donor stage and a cross-stage swap, and is not at rest until its
  line holds only `+0.0`. Removing each of the three zero-run copies (carry, same-stage swap,
  cross-stage swap) is red at the row's "claimed at rest, but a silent block wrote" check: the tail
  drop itself.
- **MINOR**: `note_line_input` walks frame-major over the contiguous interleaved block from its end
  and stops when no lane counts or after `latency` frames (the run saturates there). The lane
  vocabulary has no bitwise lane test (its `eq` takes `-0.0` as `+0.0`), so the fold is over `u32`
  words. A counter that reads one plane only is red (the move rows feed the right plane alone).
- **#2143's successor action** (after #2149 merged): `Runtime::bank_slot_runs` carries each row's
  bypass skips, `TestOnlyBankSlotRuns::bypass_skips` exposes them, and
  `console_workload::bank_counters` reports them in place of the constant 0. New W=4 pins, from the
  wasm leg (the native `Simd4` leg runs on arm64 in CI and has not read them yet):
  `c1_console_16_load` `[15, 0, 0, 0, 260, 7164, 0]`, `c1_console_16_typical`
  `[15, 0, 3584, 2304, 260, 3580, 0]`; `all_active` and `sparse` unchanged. The plan gate holds
  every row's runs, bypass skips and skipped chains to the plan's slot-blocks, the two unbypassed
  rows to the full track runs, and `Load` to no run.
- Gates: `scripts/run-wasm-gates.sh` (both legs, then the V8 spill gate) ok; every lint-job step;
  `cargo test` for rack, graph-compiler, bench, console-workload, wasm-gates and capi.


## Amendment (finish map, 2026-10-26; tree main `26d8f97c8` + #2142 PR #2154 `4f5d448eb`)

### S2 scope after S1b (PR #2150) and #2142 (PR #2154)

Already done by S1b on the per-node path, and **not to be rebuilt**: the un-bypass law
(`LiveControlEffect::stage`: `take_rest()` -> `processor.reset(DiscontinuityKeepParameters)` ->
`begin_hold(warmup)`), the D5 fold of a settled-bypassed instance's declared tap value, and the
`settled_bypassed` read before the mix. D7's "`stale` and the resume reset" clause is therefore
superseded by the per-lane law: there is no per-node `stale`.

What S2 adds, in the arm only:

- Skip iff `settled_bypassed && staged.staged == 0 && prepared_targets() is empty && no armed tap
  whose bypass value is None`. On a skipped block: at zero latency no `capture` and no `process`
  (the planes already hold the dry words; `apply_bypass` is not needed either, since `needs_dry`
  selects dry = the plane); at nonzero latency exchange the block through the line
  (`BypassShunt::passthrough`, built by S1b) and no `process`. The words equal today's
  `capture` + `process` + `apply_bypass` for a settled-bypassed lane by construction (the wet
  words are discarded by the select).
- `bypass_skips: u64` on `LiveControlEffect`, exposed through `NodeKind::bypass_skips()` beside
  `silent_skips()` (about `:3696`). Not added to `console_workload::bank_counters` (the W=4 table's
  `bypass_skips` is the bank counter; its pins stay).
- The reverb's `W` is the trait default (declared latency 0, `crates/reverb/src/lib.rs:514`); the
  delay's likewise; the hold is a no-op for both. Decision 15 E4's rule stands: no D7 check on a
  skipped block.

Gates (minimal): one row in `crates/graph-compiler/tests/bypass_shunt_identity.rs` (the delay and
the reverb per node, settled-bypassed for `N` blocks: words equal the oracle by `to_bits`,
`bypass_skips == N`; a block with a span, a target or an armed `None` tap skips none; red on a
skip that reads `bypassed()` without `bypass_ramping()` or drops the line exchange); one-time
`to_bits` of C1-16 `Load` and `Toggle` (600 blocks) against main as PR evidence; no CPU row (#2147
measures the reverb). Realtime: the arm is inside its `REALTIME_POLICY` region; record the count.
Paths: `crates/graph/src/runtime.rs`, the test file, this spec. The module bytes move: the
release-window rule applies.
