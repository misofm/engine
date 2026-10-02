# VERIFY-1: adversarial review of the submix, send and VCA plan

Role: Sol adversarial review. Reviewed `DESIGN.md` and `issues/00`-`23` against `main` at
`fe8ac679` (confirmed with `git log -1`), `AGENTS.md`, and the owner's preference files. Nothing in
the repository was modified.

**Overall verdict: FAIL as written. Revise and re-verify.**

- The architecture is sound and well researched:
  - submixes become ordinary strips;
  - sends are routes;
  - VCAs are control-only;
  - routes become live only into submixes;
  - the performance tiers are class A.
- Both delegated decisions are upheld.
- The affine ramp law is upheld, with three amendments.
- One BLOCKER makes live send mute or unmute wrong on any send that carries a PDC compensation delay.
- The MAJOR findings fall into four groups:
  - gaps in control-plane ownership and in the interaction with open #1053;
  - an unsound silence-skip condition;
  - oversized slices and pre-committed optimisation;
  - a benchmark slice and several gate commands that do not work as written;
  - schema spellings frozen wrong in K1: `matrix_or_pan` is not a JSON key, and validation paths
    are by index, not `[id=..]`;
  - anchor and authorized-path errors that would strand a "work only from this body" implementer.

None of this needs a new design round. Every finding has a concrete fix.

How the evidence was gathered:

- **Own reading.** I read every issue body and the design end to end. I checked the load-bearing code
  myself:
  - `execute_op`'s delayed-edge staging and `program.rs`'s `DelayRef`;
  - host-web's mute and solo admission;
  - the compressor's link masks;
  - the D11 ramp documentation and code;
  - the browser and SDK live-control defaults;
  - the CI aarch64 jobs.
- **Anchor checks.** Three read-only sub-verifiers checked every file:line anchor in DESIGN sections
  2-7 and issues 00-06, 07-13 and 14-23. Their results are merged in section 4.
- **Probe re-run.** I re-ran the planner's probe in its scratch copy, not in the repository:
  - `cargo test -p host-core --test probe_submix`;
  - `cargo test -p capi --test probe_submix --test probe_route_edit`.

  All pass and reproduce the DESIGN section 3.2 claims (see 4.1).
- **Ramp simulation.** I simulated the proposed affine ramp law against today's D11 law in `f32`:
  700 random ramps with lengths from 2 to 48000 (`scratchpad/verify_affine.py`).

---

## 1. Delegated decisions

### (a) VCA groups in scope now, sequenced last: **UPHOLD, with fixes**

Why it survives:

1. **The intent must live in the session.**
   - The engine owns the edited session on every platform, and apps save the engine's canonical
     JSON (owner rulings in `product-goal-browser-daw.md`).
   - V1 has no free-form metadata.
   - An agent that emulates a VCA by rewriting member faders therefore has nowhere to keep the
     grouping, or each member's own value, where a fan's phone or the next agent can see it.
   - Once a member clamps at the `[-144, 24]` dB edge, the saved session has lost the balance for
     good.

   This is the planner's strongest argument, and it holds.
2. **The audio semantics are the industry's.**
   - dB offsets are summed over a reach set, and each VCA counts once on a diamond.
   - Mute is an implicit OR that keeps the member's own mute.
   - There is no audio path.
   - Post-fader sends follow, because the offset lands on the member's fader.

   This matches the Pro Tools, S6L, DiGiCo and SSL material the plan cites. Clamping to the fader
   domain is the engine's equivalent of S6L's cap.
3. **It needs no render code, and nothing depends on it.** Sequencing it last keeps it off the
   critical path.

Weak arguments that should not be repeated to the owner:

- **"32 commands per gesture instead of one."** The browser admits a batch atomically, and #1053's
  C ABI transaction carries many edits. A VCA emits the same 32 fader records internally anyway.
- **"It overwrites the producer's values in a fan's personal mix."** A personal mix is a copy, by
  ruling.

Fixes this verdict depends on:

- the effective-mute owner for submix members (MAJOR-2);
- the interaction with #1053 (MAJOR-3);
- the C ABI caps (MINOR-6).

I also recommend filing 21-23 as their own umbrella once the bus and send umbrella closes. Their
required root key then adds no third full fixture migration inside this one.

### (b) A submix strip is the same two-lane, dual-mono strip as a track, with all five input keys: **UPHOLD**

Why it survives:

1. **It is `AGENTS.md`'s law, applied per strip.**
   - L and R carry independent state and parameters.
   - Coupling happens only through a declared `link_mode` or the explicit smoothed 2x2.
   - A "stereo" strip with one shared parameter set would be a second strip shape, a second grammar
     and a second family of bank programs. That breaks "one implementation shape" and buys nothing
     an insert with `link_mode: maximum` or `average` cannot already do.
2. **Never collapsing a bus is the only reading of the owner's ruling.**
   - The collapse applies only to a declared mono source file, and a bus has no source.
   - The real guard is not the pool class. `arm_mono_collapse` arms only where
     `gathers_track_input` holds **and** every lane's ID is in the host's `eligible` set, which is
     `session_structural_symmetry` over tracks only (`runtime.rs:2332-2336`,
     `host-core/src/prepare.rs:1284-1289`).
   - So the design and issue 02 are wrong to say that the pool class is "the only guard" and that
     `gathers_track_input` "must never hold for a bus". It is a pure node-shape test, so it will hold
     for every bus chain.
   - Seeding `Stereo` explicitly is harmless but not sufficient. The eligibility set must stay
     tracks-only, and issue 02's hazard and gate 4 should name that set as the guard.
3. **All five input keys are justified.**
   - Trim and polarity cost nothing.
   - A disabled HPF or LPF is the arithmetic identity.
   - `delay_samples` lowers no node at zero.
   - The input stage is where D7 sanitizes.
   - A reduced bus input section would need its own grammar and codec for no gain.

Corrections to the owner-facing framing:

- **Q1's premise is wrong.** It says a per-strip `link_mode` override "would split banks".
  - In the compressor kernel, link is already a lane mask splatted per bank: the `linked` and
    `averaged` masks at `crates/compressor/src/kernel.rs:330-339`. `dual_mono` only selects a
    cheaper path, at `:624`.
  - A per-lane link would therefore be a per-lane mask, not a bank split.
  - Its real costs are:
    - a schema change to decision 12's slot declaration;
    - the slower linked path for the whole bank whenever any lane links;
    - the same generalisation repeated in every effect that has a link mode.

  Restate Q1 with those costs.
- **The bus-compressor consequence is a hazard, not a footnote.** Every bus carries the session's
  console compressor (O1). If the session declares that slot `dual_mono` for tracks, it runs
  unlinked on every stereo bus and shifts the image under asymmetric material. Issue 06's
  `author-session` skill update and its app handoff must say: bypass the console compressor on
  buses, and use a linked insert for bus glue.

---

## 2. The affine ramp law (P5, R4): **sound; UPHOLD with three amendments**

**The law.**

- `step = round(round(target - start) / length)`, computed once.
- For `1 <= k < length`: `c(k) = round(round(k * step) + start)`.
- From `k >= length`: `c = target`.

**Why it is sound.**

- `k` is exact in `f32` below 2^24.
- Rounding is monotone, so `c` is monotone in `k`.
- `c` cannot pass the target. At `k = length - 1` the real-number gap is about one `step`, which
  exceeds the accumulated relative error up to roughly 5.6 million samples.

**What the simulation found.** 700 random ramps, lengths 2 to 48000, coefficients up to ±15.85:

| | Monotonicity or overshoot violations | Worst error, in ulp of the ramp's scale |
|---|---|---|
| Affine law | 0 | 2.26 |
| Today's D11 `current += step` | Passed the target before the snap on 1,791 samples | 17,180 |

So the affine law is strictly more accurate, and it is width-independent by construction.

**Is a second law justified?** Yes, but not for the reason the plan gives.

- D11 could be vectorised across the four coefficients, as one four-lane serial chain per frame. So
  "otherwise it is a scalar loop" is not quite true.
- The decisive reason is that `c(k)` is a pure function of the frame index. The fused reducer
  (issue 15) and the epilogue fold (issue 18) can then compute the same coefficients in any
  traversal order, without threading serial state through tiles. That is what makes their class-A
  gates achievable.
- No bit-identity contract ties route ramps to fader or matrix ramps, so nothing pinned moves.
- The settled state equals a fresh plan's, because the target is assigned, never computed.

**Required amendments to issue 11 D4:**

1. **Bound `length`, and saturate `position`.**
   - Clamp or refuse `length` above 2^22, with a typed domain error at the producer, so that `k`
     stays exact and the no-overshoot bound holds. The fader already clamps its countdown at 2^24
     (`FADER_RAMP_COUNTDOWN_MAXIMUM`, `crates/builtins/src/lib.rs:2529`).
   - Saturate `position` at `length`, so a route settled for a long time cannot wrap a `u32`.
2. **Define `current(position)` exactly.** It is the same expression at `k = position`, or the stored
   target once settled. The oracle can then reproduce a retarget mid-ramp.
3. **Define when `active` flips.** This is MAJOR-1 below.

Also stop calling it "affine D11", in the design and in issues 11, 15 and 18. In the codebase "D11"
names the recursive law (`current = select(done, target, current + step)`,
`crates/lane/src/kernels/builtins.rs:185-202`). Give the new law its own name, for example "R1", in
`docs/BUILTINS_AND_METERING_V1.md`.

---

## 3. Findings

### BLOCKER

#### BLOCKER-1. Live mute or unmute of a send with a PDC delay cuts the fade or plays stale audio

**Issues:** 10 (D3), 11 (D5, D6), 15 (D3), 18 (D3); DESIGN P4 and 5.7.

**Evidence.** A compensation delay is not a node after the route. It is state that belongs to the
consumer.

- `execute_op` (`crates/graph/src/runtime.rs:2995-3007`) copies the producer's buffer into a staging
  scratch and runs `delays[line]` inside the consuming reduction.
- `DelayRef { line, staging }` is per consumer input (`crates/graph/src/program.rs:49-58`).
- Route buffers live in the coloured `DisjointArena`. A buffer that was not written this block
  therefore holds another op's samples.

The plan has three properties that do not fit together:

- an inactive route op is skipped (10 D3, 11 D5);
- `active` is written at the route op, at input time;
- the reduction skips the input on `active` at output time, `d` samples later.

What goes wrong:

1. **On mute.** The reduction stops reading the delayed input as soon as the ramp ends. The delay
   line still holds the last `d` samples of the fade. Whenever `d` is longer than the ramp, it also
   holds full-level audio, which is simply cut off. That is an audible click. For example, the
   limiter's 486 samples is longer than a 5-10 ms mute ramp.
2. **On unmute**, one of two things happens, and both are wrong audio for the first `d` samples:
   - the staging copy kept running while the route op was skipped, so the line filled with
     another op's arena samples;
   - or the staging was skipped too, so the line still holds pre-mute audio.
3. **No gate catches it.** No gate in 10, 11, 15 or 18 drives a delayed route through a live mute or
   unmute. Delayed sends are common: any send from a tap before a latent `post_insert` slot, or from
   a strip whose inserts differ in latency from the bus's other inputs.

**Required fix:** choose one rule and state it in 10, 11, 15 and 18.

- **(a) Delayed routes never go inactive.** A route with a compensation delay keeps running with
  target `[0; 4]` while muted, so it keeps feeding its line. Its contribution is defined as the
  zero-coefficient mix. Only undelayed routes use the skip.
- **(b) Deactivation is deferred.** A delayed route becomes inactive at the reduction only after `d`
  further samples of zero-coefficient output have entered its line, counted down per block. While
  inactive, neither the route op nor its staging runs, so the line is known to hold zeros when the
  route is reactivated.

(a) is simpler, and it costs only the delayed sends.

Either rule needs two gates:

- a mute, idle, unmute round trip on a route with a 486-sample compensation delay, bit-identical to
  a fresh plan once the ramp plus `d` samples have passed;
- no stale sample reaching the bus during the unmute window.

Coordinate with #899, which owns the staging copy.

### MAJOR

#### MAJOR-1. The block in which a mute ramp ends must still be mixed

**Issues:** 11 D5; 15 D3; 18 D3.

"When its ramp completes, `active = false`, which writes the executor's `active` table": the
destination reduction reads that table in the same block, after the route op has run. A ramp ending
at frame 100 of 128 would then lose frames 0-99 of its fade.

**Fix:** `active = false` takes effect at the **next** block boundary. The block in which `k` reaches
`length` is mixed whole, with exact zero coefficients after the snap.

**Gate:** a 480-sample mute ramp at a 128-frame quantum snaps at frame 96 of the fourth block. That
block must be bit-identical to the oracle.

#### MAJOR-2. No control-plane owner for a submix strip's mute

**Issues:** 09, 20, 21, 22.

- host-web lowers kind 4 (mute) through the solo mirror, which is indexed by track:
  `ready.solo.set_user_mute(track, ..)` refuses with `UNKNOWN_TRACK` past the track count
  (`hosts/host-web/src/lib.rs:4354-4373`).
- Issue 09 D2 says kinds 1-8 "apply to any strip", but its own hazard forbids resizing
  `LiveControlSoloState` to strips. Gate 1, which sends kind 4 at a bus index, therefore fails, or the
  implementer invents an unreviewed bypass.
- Issue 20 D1 takes a bus source's "effective mute" from `LiveControlSoloState`, which has no bus
  lanes.
- Issues 21 D6 and 22 D2 feed `vca_mute` into that same track-only state, although a VCA may contain
  submixes.

The solo state also emits only mute records, never `FaderDb`, and an explicit mute always stages a
record (sub-verifier, `lib.rs:4353-4378`). So "the solo pattern" is not a ready-made VCA engine.

**Fix:** add to issue 09 a host-core strip-mute mirror.

- It holds `{user_mute, vca_mute}` per strip lane, with shadow, commit and an emitted record.
- Solo stays track-only and composes on top of it for tracks.
- A submix's effective mute is `user_mute || vca_mute`.
- Issues 20-22 read this mirror for submix lanes.

Add two gates: kind 4 mutes a bus, and a follow-mute send works from a bus source.

#### MAJOR-3. With 10 or 21 on `main`, #1053's value-only path stops matching a fresh plan

**Issues:** 10, 13, 21, 23; #1053 (open).

#1053 classifies a delta that touches only track `fader` or `matrix_or_pan` as live, and pushes the
member's **own** values. That diverges from a fresh plan in two cases:

- **After issue 10.** A fresh plan zeroes a `follows_mute` send's source column while that lane is
  muted (D1b). A #1053 live mute leaves the send alone, so the leak returns until the next structural
  edit.
- **After issue 21.** A fresh plan bakes `member + sum(reach)` into the fader. A #1053 live fader
  pushes the member's own dB, so the VCA offset disappears.

The plan orders 13 and 23 after #1053, but it does not order 10 and 21 against #1053. Whichever lands
second leaves `main` rendering something other than its own committed model.

**Fix**, either one:

- issues 10 and 21 each add: if #1053 has landed, its classifier treats a strip fader or mute delta as
  structural when that strip is a `follows_mute` source or a VCA member, until 13 or 23 lands. Back
  it with a capi gate comparing the live result with a fresh plan;
- or amend #1053's spec now to carry that rule.

#### MAJOR-4. `route_coefficients` cannot reproduce a prepared `follows_mute` route

**Issues:** 10 D1b; 11 D2 and gate 1; 12 D3; 20.

- Issue 11 D2 claims that a live target and a prepared constant are "the same bits by construction".
  But its signature, `route_coefficients(gain_db, matrix, mute)`, has no source-mute input.
- Issue 10 D1b makes the prepared constant depend on the source strip's mutes.

Between slices 12 and 20 this has two consequences:

- any live send edit on a `follows_mute` route whose source lane is muted restores the leaking
  column;
- issue 11 gate 1 ("random gain, matrix and mute change ... equals a fresh plan") fails whenever its
  generator draws `follows_mute: true` with a muted source.

**Fix**, either one:

- add a `source_lane_muted: [bool; 2]` argument in issue 11 that zeroes columns exactly as D1b does.
  Issue 12's mirror then carries each live route's prepared source mutes, and issue 20 only updates
  that input live;
- or move the prepared `follows_mute` semantics from 10 into 20, keeping only the field in 10.

#### MAJOR-5. Issue 16 skips at `post_pan` without requiring a settled matrix, and its #940 citations are wrong

**Issue:** 16 (D1, D2, gate 2, gate 7).

**The signed-zero condition.**

- Mute is a bit-clear, so a settled-muted lane is exactly `+0.0` at `post_fader`. A sub-verifier
  confirmed this: `andnot` in `gain_mute_block` and `fader_matrix_block`.
- At `post_pan`, however, the output is `ll * 0 + lr * 0`. Its sign follows the coefficients.
- `matrix2x2_ramp_block` ramps the coefficients with no identity select. A pan or matrix ramp through
  zero on a muted strip therefore changes the zero's sign within the block.
- D1 requires only the **fader** to be settled. So the "known signed zero per block" that feeds
  #940's fix-up can be wrong. That breaks class A.
- Open #1072 (the banked matrix normalizes `-0.0`) interacts. The rule must use the sign the banked
  production path gives, and it moves when #1072 lands.

**The citations.**

- **Gate 2's mutation list is wrong.** #940's G2 list is:
  1. drop the fix-up;
  2. fill `+0.0` regardless;
  3. fix up whenever anything was skipped;
  4. **swap the two inputs within a pair**;
  5. the first pair with `initial_store = false`.

  Issue 16 replaces the swap with "skipping a ramping route". That is not one of #940's mutations,
  and gate 1's corpus has no ramps, so it cannot turn gate 1 red.
- **Gate 7 names the wrong check.** The "non-generic `#[inline(never)]` tails" rule is rule 3 of
  `scripts/check-web-audioworklet-callgraph.py`, which `check-web-audioworklet.sh` runs.
  `check-realtime-policy.sh` has no such rule.
- #940 is closed.

**Fix:**

- define silent-muted at `post_pan` as "fader settled-muted **and** matrix settled for the whole
  block";
- add a gate for a matrix ramp through zero on a muted strip;
- correct the mutation list;
- add `check-web-audioworklet.sh` with the callgraph rule to gate 7, and to issue 15, whose new
  `L`-generic kernels with an `f32` tail are the shape that failed that rule in #920;
- name #1072 as interacting.

#### MAJOR-6. Slices too big, or bundling independent outcomes (`AGENTS.md` half-day and one-outcome rules)

**Issues:** 02, 10, 17.

- **Issue 02** covers:
  - grammar, the BTLV codec and the conformance hash;
  - lowering and builtins preparation;
  - a runtime delay arm;
  - a migration and two docs;
  - five new render gates.

  It even says "if it runs past one working day". Split it up front:

  - **02a:** grammar and codec;
  - **02b:** lowering, builtins, the bus delay arm and gates 1-4.

  Both stay in K1.
- **Issue 10** bundles three independently useful outcomes:
  - route `mute`;
  - prepared `follows_mute`;
  - D7's tightening of route domains, which still waits on owner question Q5.

  Split D7 into its own slice gated on Q5. Either split `follows_mute` out too, or fix it per
  MAJOR-4.
- **Issue 17** admits it may run past a day. Split it at its own boundary:
  - **17a:** multi-destination folding, one route per lane;
  - **17b:** fan-out.

#### MAJOR-7. Tier 3 is committed before it is measured, and it names a seam that cannot carry it

**Issues:** 17, 18, 19; DESIGN 6.5.

**Commitment before measurement.**

- `AGENTS.md` says to ship a correct, measurable implementation and record diminishing-return work as
  an issue. The owner has also rejected fragile machinery bought for small gains.
- Issue 15 alone already deletes every undelayed route buffer and route pass.
- Tiers 17 and 18 add the plan's most delicate machinery: per-destination prefix proofs, K entries
  per lane, and live lanes inside the epilogue. What they buy is removing a lane's `post_pan` write
  and the reducer's re-read of it.
- No number yet says that residual matters, and issue 19 frames the order question only after 17
  and 18 are built.

**The seam.** DESIGN 6.5 and issue 17 say to generalise "the unarmed per-lane accumulate seam"
(`aux_plane_mut` and `arm_aux`, `crates/rack/src/lib.rs:1704` and `:2107`). That seam cannot do the
job:

- it holds one optional destination per lane;
- `accumulate_aux` only adds the raw resident result with `+=`, with no 2x2 and no store-first mode;
- it is a scalar, strided per-frame loop;
- it was reserved for #210's PFL.

The real template is `FoldLane`, `fold_plane`, `fold_cohort` and `fold_resident_tiles`
(`crates/graph/src/runtime.rs:1613-1946`).

**Fix:**

- issue 15's evidence reports the share of block time left in the `post_pan` write and fused re-read
  on the bus and send rows;
- 17 and 18 start only if that share passes a threshold stated in the umbrella; otherwise they are
  recorded as successors;
- re-point 17 at `FoldLane` and leave the aux seam to #210;
- fold issue 19's padding and order analysis into whichever slice closes the tier.

#### MAJOR-8. Issue 14 (benchmark rows) is mis-specified, and the "2 % slower" gates misuse a single descriptive run

**Issues:** 14, 15-19.

The sub-verifier found the following against the tree:

- **B0 did add a fixture.** The design's B0 precedent is wrong: commit `e1b0fed3` committed
  `fixtures/session/v1/console-sixty-four-track-app.json`, a derive script and
  `check-console-fixtures.sh`.
- **The V8 rows need committed fixture documents.** `web-mixing-automation-benchmark.mjs:137-145,
  :191, :232-233` boots fixture files, and `DOCUMENT_KINDS` and `lib.jq` are hard-coded to two
  documents. That contradicts the non-goal "no new committed fixture document".
- **Required files are outside the authorized paths:**
  - `tools/bench/src/floor.rs`, whose `floor_row` exhaustive match at `:154-163` does not compile
    without new arms;
  - `scripts/web-mixing-automation-lib.jq`;
  - `scripts/web-mixing-automation-validator.jq`.
- **The runner has no row list to edit.** It hard-codes 60 records (`:381`), and the validator checks
  `length == 60` (`:10`, `:30`).
- **Preflight takes an argument:** `preflight-console-benchmark.sh --step NAME`.
- **An existing test would fail.** `every_standing_workload_folds_one_route_per_track`
  (`tools/console-workload/tests/chain_shape.rs:346`) iterates `WORKLOADS` and fails on bus rows
  unless they are excluded.
- **"Prepared the way hosts prepare" is false.** The rows go through
  `builtins_compiler::prepare_session_builtins` plus `GraphCompiler::compile_with_builtins`
  (`tools/console-workload/src/lib.rs:1310-1333`), not through host-core. They attach no live
  controls (`PlanConfig::BASELINE`, `:910-955`).
- **After 11 the native send rows time a path production does not run.** They time static `Route`
  ops, while a live-controlled browser runs `LiveRoute`. That is the owner's "benchmark real paths
  only" rule.
- **Issue 19's evidence folder no longer exists.** "Beside S4's" points at nothing at HEAD (R10
  removed those evidence sections), and its declined-fold row needs a new frozen row that its paths
  do not authorize.

**The "2 % slower" gates.** Issues 15-18 each pass or fail on "no row more than 2 % slower" from one
uncontrolled run, and `AGENTS.md` forbids a retry. S4 saw p50 move by 4.46 µs on a single row, so a
noisy run could fail a class-A slice with no recourse.

**Fix:**

- rewrite issue 14 against the real runner: fixtures or derivation, paths, record counts, the
  `--step` preflight, and the `chain_shape` exclusion;
- add live-control variants of the send rows, labelled as such;
- make 15-18's timing **descriptive**: reported against the baseline, with Sol judging any
  regression beyond 2 %, not a hard gate. Use the 2 % only as the owner's tolerance in that
  judgement.

#### MAJOR-9. The Simd4 and bank-coverage gates cannot be run as written

**Issues:** 11, 15, 17, 18; DESIGN 7.

- **`run-wasm-gates.sh` cannot give these slices Simd4 coverage.** It runs only the frozen
  `tools/wasm-gate-corpus` lane digests. It never runs graph's random-DAG corpus or host-core's
  `randomized.rs`, so "at `Simd4` through `scripts/run-wasm-gates.sh`" cannot happen.
  - Crate tests get Simd4 coverage from `bash scripts/run-aarch64-tests.sh debug` on arm64, which is
    CI's `aarch64-debug` job (`.github/workflows/qualification.yml:917-950`).
  - The script needs its `debug|release` argument and refuses a non-arm64 host.
- **The graph random-DAG corpus has no banks** (`crates/graph/src/lib.rs:5322`, "every unit a single
  op"). It cannot reach the epilogue fold, so gate 1 of 17 and 18 rests on host-core's
  `randomized.rs` alone.

**Fix:**

- name `run-aarch64-tests.sh debug`, or the CI `aarch64-debug` job at the batch push, as the 4-lane
  gate;
- confirm that each new test sits in its package list;
- make host-core's randomized generator reach the fold explicitly, with reach counts recorded;
- do the same in DESIGN 7, where `run-aarch64-tests.sh` also lacks its argument.

#### MAJOR-10. The live-controls premise is false, which skews 11, 18 and the "why" behind D1

**Issues:** DESIGN 5.7 ("Why the split"); 11 (gate 6); 18 (product outcome).

- "The browser always prepares with live controls" is false.
  - `live_control_command_queue_records == 0` means no control channel, and
    `WebBootOptions::explicit_defaults()` sets 0 (`hosts/host-web/src/lib.rs:1102-1104` and
    `:1137`).
  - The SDK also defaults to 0 (`sdk/src/core/abi.ts:186`).
  - The standing console rows run `PlanConfig::BASELINE`, with no live controls.
- **Gate 6 of issue 11 has nothing to measure.** It passes trivially unless
  `every_standing_workload_folds_one_route_per_track` gains a live-controlled arm.
- **Issue 18's premise is wrong.** "In the browser every send is live" holds only for a host that
  opts in, such as the producer's mixer. It does not hold for a fan's playback with no controls.

**Fix:**

- restate the premise as "a live-controlled plan (the producer's mixer)";
- add the live arm to the fold-count test;
- re-weigh tier 3's value with that narrower reach (see MAJOR-7).

#### MAJOR-11. Issue 15 revives a kernel family that #957 deleted, without citing why

**Issue:** 15.

- A fused route-into-reduction kernel already existed: #920, #926 (pairs), #937 (groups of 8) and
  #940 (skip).
- #957 (`bf3bacab`) deleted it because only bankless plans reached it, under the owner's "benchmark
  real paths only" rule.

The issue must cite that history, and it must show that a production host now reaches the new path.
A banked plan's routes into submixes, and any unfolded destination, do reach it. It should also carry
over that family's lessons:

- the callgraph-outlined `f32` tail;
- group-of-8 chaining.

Otherwise it reads as a reversal of an owner decision.

#### MAJOR-12. Anchor errors and authorized-path gaps would strand a body-only implementer

**Issues:** many; section 4 has the full list.

Each issue tells its implementer to "work only from this body", so every wrong anchor and every file
the change is forced to touch but cannot must be fixed before filing.

The worst cases:

- **07:** `HostPrepareCaps` literals in `crates/host-core/src/response.rs:997`,
  `limiter_linked_session.rs:255` and `crates/capi/tests/resource_lifecycle.rs:466`. The struct has
  no `Default`, so adding a cap is a Rust API break at every literal.
- **08:**
  - neither `WebMeterHeader` reserved word is free. `reserved[0]` is the publication generation;
    `reserved[1]` holds validity bits and a loss count. So `submix_count` must be appended, and the
    header grows past 64 bytes;
  - five more `3T + 3` spellings exist outside the paths;
  - `master*TrackPlusOne` is spelled in about 20 more files.
- **09:** reason 12 collides with the `COMMAND_REASON_FUTURE_TAP: u32 = 12` mutation in
  `scripts/test-web-audioworklet.sh:245` and in `MUTATIONS.md:132`.
- **10:** the opcode count 41 is also pinned at:
  - `crates/protocol/src/controller/tests.rs:326`;
  - `crates/conformance/src/protocol_corpus.rs:664`;
  - `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`.

  The schema-hash literal is also in `scripts/check-protocol-wasm-parity.sh:172-173`.
- **12:** gate 4's `cargo run -p parameter-metadata -- --check` needs a directory argument.
- **15, 16, 17 and 18:** they need `crates/host-core/tests/randomized.rs` and
  `tools/console-workload/tests/` for their own gates.

#### MAJOR-13. The K1 schema slices freeze wrong spellings, a false fixture claim and an incomplete site list

**Issues:** 01, 02, 03, 04, 06; DESIGN 5.2 and 5.11.

**`matrix_or_pan` is not a JSON key.**

- A track carries exactly one of `pan` or `matrix` (`crates/session/src/parse.rs:977-1007`,
  `:1222-1257`). Both map to one tagged wire field (track field 10: pan 1, matrix 2;
  `visit.rs:91`, `schema.rs:984-985`), and the SDK writer agrees (`session-json.ts:113-116`).
- The following are therefore invalid:
  - DESIGN 5.2's `"matrix_or_pan": {"kind": "pan", ..}`;
  - issue 02 D2's canonical order, which should read `id, builtins, console, inserts, fader,
    pan|matrix`;
  - issue 06 D3's key order.

**Validation paths are index-based, not `[id=..]`.**

- Session validation uses `PathRef::index` (`validate.rs:303`, `:473`), and tests pin
  `$.submixes[0].id` and `$.routes[0].source.submix_id` (`crates/session/tests/invalid_matrix.rs:376`,
  `:602`).
- Fix these paths: issue 02 gate 5, issue 03 D2 and gate 4, issue 04 D1, and DESIGN 5.2.
- Only the compilers use `[id=..]`.

**"No JSON fixture declares a submix" is false.** Three fixtures say otherwise:

- `fixtures/session-canonical/v1/canonical-writer-corpus.json` (submix `mix`, plus a
  `submix_output` route), regenerated by `crates/session/src/canonical.rs:249` and `:396-405`;
- `.claude/skills/author-session/worked-session.json:275-290`, whose canonical form is asserted by
  `sdk/test/console-evals.mjs:671-673`;
- `fixtures/graph/v1/invalid-scc-diagnostics.json`, which spells `submix:` nodes in a hand-written
  literal. It will go stale silently once P1 stops emitting them.

Issue 02's constructor list is also wrong in both directions. `diagnostic_parity.rs` and
`invalid_matrix.rs` build no `Submix`, and `canonical.rs:249` is missing.

**Missed sites:**

- **01:**
  - the builtins seal's `tracks` vector and `processor_seal` (`builtins-compiler/src/lib.rs:3518-3523`,
    feeding `planned_strip_banks` at `:2296`, `:2321` and `:2445`);
  - `effect_path` (`graph-compiler/src/ids.rs:232-241`), which hard-codes `$.tracks[id=..]` into
    **sealed canonical edge paths**, along with `compile.rs:249`, `:287`, `:293` and `:294`. A bus
    effect would otherwise be pinned under a track path;
  - `host-core/src/shape.rs:79-81`.
- **02:**
  - `reduction_records` (`graph-compiler/src/ids.rs:77-84`) records only `Submix` and `Output`
    nodes. Once `Submix` is no longer emitted, bus sums **vanish** from the canonical text and from
    `GraphCompiler::reductions` (`compile.rs:65`, `canonical.rs:181`). It must count a submix strip's
    `Input` reduction;
  - `estimate.rs:65-97` (track-only `effect_count` and `parameter_count`, published in
    `ResourceEstimate`).
- **06:**
  - the enginectl CLI (`sdk/src/cli/session-request.ts:37`, `:126-130`, `:418`, `:444-445`) parses
    submixes as bare IDs and accepts `submix_output`;
  - `sdk/test/enginectl-cli.mjs`, `builder-evals.mjs:959`, `console-evals.mjs:653` and
    `sdk/README.md:326`.
- **04:** `route_source::KNOWN` is also the sidechain source spec (`schema.rs:856`). The wire change
  reaches sidechains, and the conformance rows should cover a tapless sidechain source too.

**Fix:** correct each body before filing, and add the missed sites to its authorized paths.

### MINOR

1. **Issue 10 gate 1 overclaims.** It says the result is bit-identical "to the session without the
   muted route". That is false when the next contributor yields `-0.0`, because P4 then gives
   `+0.0 + -0.0 = +0.0`, which gate 2 itself pins. Qualify gate 1, or compare it against the P4
   oracle.
2. **Issue 15 gate 3 is likely false.** "Arena buffer count drops by exactly the number of fused
   routes" does not hold, because a route that is the sole reader of a non-dedicated tap runs in
   place and owns no buffer (`program.rs:724-737`, `is_dedicated :231`). Count retired route ops
   instead.
3. **Issue 16 gate 4 hard-codes 112 skips per block.** The real count depends on folds and on which
   routes are live. Derive it from the compiled plan.
4. **Issue 00 leaves an `AGENTS.md` sentence stale.** "Tracks are dual-mono: left and right have
   independent state and parameters" must become "Strips (tracks and submixes) ...". That is
   decision (b).
5. **"Unity" console entries are not unity.**
   - `Submix::unity` (03 D4) and the SDK's `submix(id)` (06 D1) set `bypass: false` with default
     parameters, so a migrated bare bus runs every console slot.
   - Use `bypass: true` wherever the defaults are not transparent, and note that latency is paid
     anyway.
   - The app handoff must also cover migrating saved documents: `submix_output` and the newly
     required strip keys.
6. **The C ABI's new caps are derived from `maximum_tracks`** (P12, 07 D2, 21 D8).
   - That silently changes the meaning of an existing field. A host that sizes `maximum_tracks` to
     its track count then refuses a session with more buses than tracks.
   - `miso_engine_v1_compile_limits` carries `uint64_t reserved[4]`, which "must be zero in ABI V1"
     (`crates/capi/include/miso_engine_v1.h:132`).
   - Taking `reserved[0]` and `reserved[1]` as dedicated bounds, with zero meaning "use
     `maximum_tracks`", is an in-place V1 amendment with no layout change. Prefer that, or make it an
     owner question.
7. **Latency grows with bus depth.** Every strip pays a latent console slot, even when bypassed
   (decision 12, L4). Each bus level therefore adds a full lookahead: 486 samples at 48 kHz for the
   limiter. DESIGN records this as R2, but it is not an owner question. Surface it as Q6.
8. **Solo leaks through pre-fader sends by default.**
   - `follows_mute` defaults to `false` in the SDK (10 D6), so solo-in-place still leaks every
     pre-fader send.
   - My probe re-run confirmed the leak: 0.3155 and 0.3549 where 0 was expected.
   - SSL defaults Follow Mute on.
   - Either default the SDK to `true` for pre-fader taps, or ask the owner.
9. **Issue 13's batch is stated two ways.** Its header says "Batch K3", but DESIGN 9.3 schedules it
   after #1053. Make them agree.
10. **Dangling buses are computed every block.** DESIGN 3.2 notes this but files nothing. Under the
    owner's "skip work on silence", file a follow-up to prune or skip a submix that has no outgoing
    route and no meter.
11. **The design and issue 11 disagree on the mechanism.** DESIGN 5.7 has a live route as "a route
    node with a `GraphNodeBinding`". A processor binding would make it `Bound(processor)` and skip the
    route mix. Issue 11's `NodeKind::LiveRoute` plus `GraphRouteControlBinding` is the coherent one;
    fix the design.
12. **`master_strip` is still a designation by index** (DESIGN 5.8). It does not "retire" the
    "designation, not discovery" stopgap; it only widens it. Reword.
13. **Issue 22 has the zero direction reversed.** A redundant mute record moves a settled `+0.0` to
    `-0.0`, not `-0.0` to `+0.0`. Also, "slices 12-15 took 13-15" should read slice 12 only.
14. **#1054's spec defines `controlSmoothing` as optional,** while V1 says it has no optional fields.
    Note it under R7; it is not this plan's defect, but P6 depends on it.
15. **Issue 07's meter-order description is imprecise.** Selected meters keep caller order;
    `host.meter.order` fires only when bound consumers mismatch. Reword before an implementer relies
    on it.

---

## 4. Factual verification

### 4.1 Probe re-run (scratch copy, not the repository)

**host-core** (`probe_submix`): every legal shape parses, compiles, prepares and renders, with and
without live controls, and matches the `f64` oracle to about 1e-7.

| Claim | Result |
|---|---|
| Fold counts | 8, 2, 0 and 8 as stated |
| Cycles | Refused with `graph.cycle`, with a witness path |
| Output count | Refused with `graph.output.cardinality` |
| Route gain -800 / +1000 dB | Refused with `graph.gain.non_finite` |
| Route gain -1000 dB | Exact zero |
| PDC (limiter cases) | 486-sample compensation on exactly the early edges; impulses peak at 486 on both planes |
| Pre-fader send under mute | Leaks, with output `[0.3155, 0.3549]` |

**C ABI** (`probe_submix`, `probe_route_edit`): the same shapes match. The route-gain edit `0505`
gives:

- an all-zero boundary block;
- `source.frame.noncontiguous` until a seek;
- a hard step to the new gain.

So DESIGN 3.2 is reproduced for the host-core and C ABI paths. I did not re-run the wasm or V8 path.
The match is to printed decimals, not bit for bit, as the design says.

### 4.2 Anchors confirmed correct (sample)

These are correct to within ±5 lines and say what the plan claims:

- **graph runtime:** `reduce_*` (`:399`, `:426`, `:562`, `:583`, groups `:456-:618`), `FrameLane`
  (`:316`), staging (`:2996-3008`), route op (`:3050-3053`, `:4062`), `plain_route_gains`
  (`:6110-6122`), `route_fold` and its clauses (`:6227-6432`), `FoldLane` and `fold_*`
  (`:1613-1946`), `folded_route` (`:5990`), `gathers_track_input` (`:5208-5222`);
- **lane:** `mix2x2_block` (`kernels.rs:1027`). `Lane::fma` is two roundings on every target,
  including AVX2 with FMA pinned (`wide_impl.rs:187`, `scalar.rs:86`, `check-unfused-seal.sh`);
- **builtins-compiler:** the bounded input drain (`:457-475`), the unbounded fader and matrix drains
  (`:970`, `:1006`), `TrackFaderRecord` (`:129`);
- **capi:** compile `:408` (no live controls), control `:46-53` and `:718-857`; the 240-byte report
  and the 14 exported symbols;
- **host-web:** the 48-byte record, kinds 1-12 (13 is free), reasons 0-11 (12 is free), admission
  (`:4235`, `:4276`, `:4324`), the queue bands, the `3T + 3` frame;
- **protocol:** 41 opcodes (`model.rs:1315`); `0504` is matrix and `0505` is gain; `0506`, `0507` and
  the `07xx` family are free;
- **session:** root field IDs are 1-7 and 9-15, so 16 is next and #1054 claims none;
- **#1053:** D1, D2 and A1.4 are as the plan states. #1053, #1054, #1057, #1058, #349, #899, #210 and
  #1107 are open; #940 is closed.

### 4.3 Errors and imprecisions found by the sub-verifiers

The material ones are carried in MAJOR-5, -8, -9, -10, -11 and -12. Smaller ones to fix in the
bodies:

- `kernels/builtins.rs:282` is the signature of `matrix2x2_block_without_identity`; the arithmetic is
  at `:300-301`.
- `checked_fader_gain` (`builtins/lib.rs:4108`) is the live check. The prepared section's range check
  is `prepare_sections :3141-3143`, and the session crate never range-checks `fader_db`. Issue 21's
  VCA range check therefore has to be added in session validation.
- `capi_retained_bytes` is a report field (`abi.rs:286`), not a function. The function is
  `capi_resources` (`compile.rs:109-213`).
- `protocol/src/model.rs:687-708` are the apply arms; the opcode enum is at `:87-97`.
- Issue 10 says 19 route-declaring documents; there are 20. Two are in `.claude/skills/` and
  `crates/graph-compiler/tests/data/`, and none in `sdk/test/` or `tools/`. All 20 are inside D7's
  domains.
- `MANIFEST.tsv` pins `canonical.json` only transitively.
- `worklet.js:951` is the `miso.sessionmap.v1` reply, not a "ready message".
- Issue 12's ":393, :432-449" are in `check-command-kind-vocabulary.py`, not in
  `test-web-audioworklet.sh`.
- The opcode counts 41 → 43 → 46 assume #1054 adds no opcode.

### 4.4 Issues 00-06 and DESIGN 2-5.6

**Confirmed:**

- Nearly every anchor is within ±5 lines and says what is claimed. That includes:
  - the issue 01 strip-site list (with the misses in MAJOR-13);
  - the submix and route wire field IDs: submix fields 2-6 are free, route fields 6 and 7 are free,
    and no field was ever retired;
  - route source: tag 2 has no `TAP` today;
  - the shared ID namespace (`validate.rs:69-100`). P1's keying of `TrackStage` and `Effect` nodes by
    submix ID cannot collide;
  - opcodes `0506`, `0507` and `0700`-`0702` are free and not retired;
  - every gate script and package named exists;
  - issue 01's workspace test line matches `qualification.yml:611-618` exactly.
- PDC is a per-edge, exact longest path (`pdc.rs:38-161`), as stated.

**Imprecise or wrong, beyond MAJOR-13:**

- `SessionPoolClasses::from_session` is at `builtins-compiler/src/lib.rs:3924`; `:3915` is the
  struct.
- `:3350` is the parameter preflight loop; the preparation loop is `:3401-3473`.
- `:3327-3345` also validates meter requests.
- The SDK constant is `OBJECT_KEY_ORDERS` (`session-json.ts:67`), not `KEY_ORDER`.
- `graph/src/lib.rs:1591-1596` allows a `TrackStage` **or the Output**.
- `folded_bus_parts` is a `#[cfg(test)]` fixture, not production lowering. P1's argument from it is
  weaker than stated.
- Issue 05 D3 is wrong that console edits are refused by `rack_mut`. Only the structural ones are
  (`0204`-`0209`, `020b`, `020c`). `020a`, `020d` and `020e` edit through `knobs_mut`
  (`protocol/src/model.rs:997`, `:1035-1046`).
- DESIGN 3.1's "the only way to change a route is `0505`": all of `0500`-`0505` are structural route
  edits.
- P3's "-inf refused by `route_transform`": session validation refuses it first
  (`validate.rs:481`).
- Issue 02 gate 1's "both launch-rate pairs used by `randomized.rs`": there are none. That test
  draws one rate per seed at quantum 128 (`randomized.rs:57`, `:328`).
- DESIGN 5.4's "audio moves only where `-0.0` becomes `+0.0`" is incomplete. D7 sanitization (a
  magnitude of at least `1e30`, or NaN, becomes `+0.0`) now also runs on bus sums. That is an edge
  case, but the claim should say so.
- Issue 05 gate 3 cites `docs/C_ABI_V1_QUALIFICATION.md` for draining the event lane, but it does
  not say that. The nearest statement is `docs/CONTROL_PROTOCOL_SEMANTICS.md:17`.
- Issue 02's `cargo test -p conformance` omits CI's feature set: conformance runs in `test-debug-b`
  with `--features math/lane,...`.
- Issue 02's `run-aarch64-tests.sh` needs its `debug|release` argument.

---

## 5. Answers to the review checklist

1. **Factual claims.** The headline claim holds for host-core and the C ABI, which I re-ran:
   - bare submixes render correctly;
   - PDC is exact through buses and sends (486 samples);
   - the fold behaves as stated;
   - pre-fader sends leak under mute.

   I did not re-run wasm or V8. The D3, D9 and D11 references are accurate. Field 16 is free, and
   #1054 claims no field ID, so R7 is only a race and not a collision. The command kinds (13 is free)
   and the reason code (12 is free in the code, but it collides with a test-mutation literal) check
   out. The interacting issues are #1053 (MAJOR-3), #1054 (an optional field that conflicts with "V1
   has no optional fields"), #1107, #1072 (MAJOR-5), #899 (BLOCKER-1), #210 (the PFL seam, MAJOR-7)
   and #940 (closed). The errors are in 4.3 and 4.4, and in MAJOR-12 and MAJOR-13.
2. **Architecture.**
   - The render plane stays free of allocation and locks by construction.
   - The plan-replacement boundaries are unchanged.
   - Decision 12 banking is preserved.
   - PDC is exact.
   - Acyclicity uses the existing check.
   - D9 order is kept.
   - The dual-mono law holds, and there is one implementation shape.
   - **The acked-batch question:** every new queue checks room before it pushes, and its drain is
     bounded and lossless, so no ack precedes a drop.

   The defects are BLOCKER-1 (PDC meets live activity), MAJOR-1 (the activity timing), MAJOR-5 (the
   silence sign) and MAJOR-2 and MAJOR-3 (control-plane ownership).
3. **R4, the affine ramp law.** It is sound and numerically better than D11, and it breaks no pinned
   contract. It is justified because coefficients are a pure function of the frame index. It needs a
   new name and three amendments (section 2).
4. **Scope and split.**
   - Split 02, 10 and 17 (MAJOR-6).
   - Make 17 and 18 conditional on a measurement (MAJOR-7).
   - Fold 19 into the closing slice.
   - Rewrite 14 (MAJOR-8).
   - The DAG is acyclic and its order is right. The hidden cross-dependencies are #1053 against 10
     and 21 (MAJOR-3) and #1072 against 16.
   - No batch leaves `main` half-built, except through MAJOR-3.
   - Amending `AGENTS.md` in issue 00 is appropriate on the decision-12 precedent, but it is missing
     the dual-mono sentence (MINOR-4).
5. **Spec quality.**
   - The test-value answers are present and mostly sharp.
   - No test greps source.
   - There are no digest pins beyond wire hashes and re-pinned fixtures.
   - Gate wiring is broken where noted: Simd4 coverage, the benchmark runner, `parameter-metadata
     --check` and the callgraph rule.
   - Anchors and paths need correcting (MAJOR-12, MAJOR-13).
6. **Domain.** These match consoles and DAWs, with the noted caveats:
   - solo-safe buses, which are correct for a source-less bus under solo-in-place;
   - pre-fader and post-fader taps;
   - send mute;
   - follow-mute, though defaulting it to `false` keeps the solo leak (MINOR-8);
   - VCA sum, mute and clamp;
   - refusing cycles;
   - per-edge PDC.

   Two things should go to the owner explicitly: the bus-compressor link hazard and latency growing
   with bus depth (section 1(b), MINOR-7).
