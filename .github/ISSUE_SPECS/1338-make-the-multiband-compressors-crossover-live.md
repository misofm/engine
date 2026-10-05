# Make the multiband compressor's crossover live

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-13 E2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host moves a multiband compressor's crossover while it plays, on both hosts, and hears no click:
the edit is a live update (browser command kind 5; a C ABI transaction classified live by #1263's
rule for `Block`-rate parameters), the crossover glides over 64 samples, and with both bands at
unity gain the band sum stays the second-order all-pass at every sample of the glide. Today the
crossover is prepared-only, and decision 14 F2 records no reason for it.

## Context

- **The effect.** `crates/multiband-compressor/src/lib.rs` is a fixed two-band split, one crossover
  per channel. The split is two TPT state-variable stages sharing one word set:
  `(v1, lp1) = svf(x)`, `ap = x - 2k v1`, `(_, low) = svf(lp1)`, `high = ap - low`, `k = sqrt(2)`
  (module doc `:1-21`; `lr4_step`, `:503-508`). So `low + high = ap` up to one rounding, whatever the
  words are.
- **The design.** `design_lr4` (`:519-561`) designs `(c1, a2, a3)` in `f64` from
  `g = tan(pi fc / fs)`, `t = g (g + k)`, `c1 = t / (1 + t)`, `a2 = g (1 - c1)`, `a3 = g a2`, rounds
  once to `f32`, then checks the rounding back (half-power point within 0.005 dB). It returns `None`
  outside the domain. The words are kept per track (`Side::designed`, `:656-669`) and loaded per
  lane (`lane_coefficients`, `:725`), so lanes with different crossovers already share one bank.
- **The descriptor row.** Parameter id 1, `crossover`, 80 Hz to 8 kHz, default 1 kHz, logarithmic,
  `AutomationRate::None`, `SmoothingRule::None`, 0 samples (`:178-190`). The other ten parameters
  are `Block`, linear over `SMOOTHING_SAMPLES = 64` (`:105`), held in `RAMP_COUNT = 10` per-track
  `LinearRamp`s (`:76`); `apply_automation` (`:1228`) refuses parameter index 0 as an invalid span.
- **The segment kernel.** `run_segment` (`:876`) has a compile-time `RAMPING` switch; `plan_segment`
  (`:1140`) splits a block at every ramp arrival and snaps a ramp with `remaining == 1` (D11);
  `flat_path_is_identity` (`:1178`) asserts in debug that the flat path drops only `+0.0` steps.
- **The state payload.** Per channel: crossover, two smoother words, ten four-word ramps, four filter
  words: `LANE_HEADER_WORDS = 47` (`:82`, layout comment `:1289`). `stage_side` (`:1345`) validates
  and redesigns the crossover on restore; `STATE_LAYOUT_VERSION = 1` (`:86`).
- **The EQ precedent for ramping SVF words.** The parametric EQ ramps the same `svf_step` words
  linearly over 64 samples with an exact final assignment (D11; `crates/parametric-eq/src/lib.rs:20-23`).
  Its stability argument is the spectral norm of the zero-input transition matrix
  `M = [[1-2c1, -2a2], [2a2, 1-2a3]]`, which is convex in the words, so checking both ends checks
  the whole ramp (`word_spectral_norm`, `:796`; `NORM_TOLERANCE`, `:810`;
  `RAMP_PATH_NORM_TOLERANCE` with its derivation, `:815-835`). That derivation uses only
  `c1, a2, a3 < 1`, which the crossover words also satisfy.
- **Banking.** The multiband banks as an insert; it is not console-eligible until #1069 closes, and
  its session bypass stays prepared (`PREPARED_BYPASS_EFFECTS`,
  `crates/effect-compiler/src/prepare.rs:244-268`). Its randomized bank differential still runs at
  reduced strength because of #1069 (`crates/multiband-compressor/tests/randomized.rs:1-28`, ignored
  reproducer `:43`): a lane's bits depend on where a neighbour's ramp or a block boundary cuts a
  segment, because band coefficients are refreshed at each segment's first sample.
- **Not target-capable.** The multiband has no `target_preparation` capability, so its live records
  are `Parameter` records lowered to point spans. A target-capable owner refuses `Parameter`
  records (`crates/effect-contract/src/live.rs:415-432`), and the hosts' target preparers serve only
  the 60-row EQ (`EqTargetPreparer::new`, `crates/host-core/src/control_preparation.rs:309-318`).

## Decisions frozen for this slice

- **D1. Descriptor.** Row id 1 becomes `AutomationRate::Block`, `SmoothingRule::Linear`, 64
  samples. Domain, default, mapping and id are unchanged. No other row changes.
- **D2. Where the design runs.** A crossover span is applied in `apply_automation` like the other
  rows: the render thread designs the target words once per accepted span per track and channel,
  with the same `f64` designer (`math::tan`, never the platform libm). Not prepared targets: making
  the multiband target-capable would move all eleven rows off `Parameter` records, which the hosts
  do not support for any effect but the EQ. The design is bounded (a fixed count of `f64` operations,
  no loop, no allocation) and runs once per edit, not per sample.
- **D3. The design cannot fail on render.** Render calls only `design_lr4_words`, the infallible
  designer *Prove the crossover designer total and share the SVF ramp stability check in
  effect-runtime* (#1366) splits out and proves total over the whole domain at all four launch
  rates. Prepare and restore keep the checked `design_lr4`. So render has no failure branch and no
  acked crossover edit can be dropped.
- **D4. Word ramps.** Each track and channel gains three `LinearRamp`s for `c1`, `a2`, `a3`, after
  the ten parameter ramps. An accepted crossover span sets `crossover_hz` to the target and calls
  `set_target(word, 64)` on each. `(target - current) / 64` is exact (a power-of-two division; the
  steps are normal: the smallest word in the domain, `a3` at 80 Hz and 96 kHz, is about `6.8e-6`, so
  two distinct designs differ by at least one ulp, about `1e-12`, and a step is at least `1.4e-14`).
  The kernel reads the coefficient lanes **per frame** from the ramps' current words on the ramped
  path (`nc1 = -c1`, `a2`, `a3`), advanced before use, exactly as the parameter ramps are. They are
  never cached or refreshed at a segment boundary, so where a segment is cut cannot move them. The
  D11 snap, the segment split and `flat_path_is_identity` cover the three word ramps. On the flat
  path the lane coefficients equal the settled words bit for bit. A prepared bypass still advances
  the word ramps, as it advances the others.
- **D5. Stability.** The words ramp linearly between two designs. #1366 moves the EQ's norm into
  `effect_runtime::svf::transition_norm` with `NORM_TOLERANCE` and `RAMP_PATH_NORM_TOLERANCE`, and
  proves every designed crossover triple within `NORM_TOLERANCE`; by the convexity argument every
  point of a ramp is then within `RAMP_PATH_NORM_TOLERANCE`. This slice only uses them.
- **D6. Payload.** Each channel's payload gains the three word ramps, four words each, after the
  ten parameter ramps: `LANE_HEADER_WORDS` goes from 47 to 59, and the descriptor's
  `maximum_state` follows. `STATE_LAYOUT_VERSION` stays 1: the layout is prelaunch, nothing persists
  state (R6b), and the codec's word-count header refuses a stale length. Restore validates each word
  ramp with `ramp_path_within` (`crates/effect-runtime/src/state_payload.rs:284`), requires the
  target words to equal `design_lr4(crossover_hz)` bit for bit, and requires the current and target
  triples to pass `transition_norm` within `RAMP_PATH_NORM_TOLERANCE` and `NORM_TOLERANCE`. A
  refused restore changes nothing.
- **D7. Latency, tail, NaN.** Latency stays 0; the tail declaration is unchanged here. *State the
  multiband compressor's bounded tail and exact-rest bound* (#1373) owns the tail, the crossover
  glide's in-flight case included, and lands after this slice. The crossover value is
  domain-checked at admission and again by `parameter_value_valid` in `apply_automation`; the words
  are positive and normal; the four recursive words keep their `flush`; the once-per-block D7
  boundary check is unchanged.
- **D8. Carry.** The crossover becomes a live value, so a swap carries it with the lane's state and
  then retargets (D15-7); the word ramps travel in the payload (D6) or with the moved processor. No
  carry code changes here.

## Deliverables

1. D1, D2, D4 and D6 in `crates/multiband-compressor/src/lib.rs`.
2. Regenerated `sdk/assets/miso-engine-v1-parameter-metadata.json` and `sdk/src/generated/*.ts`
   (`node codegen/assets.mjs`, then `node codegen/generate.mjs`, from `sdk/`).
3. Tests (gates 2-5) in `crates/multiband-compressor/tests/crossover_live.rs`; the randomized
   differential's generator reaches crossover records.
4. Effect evidence in the PR: equations, update rule, stability, citations (below), and a blinded
   A/B listening note of a 200 Hz to 4 kHz glide on program material, ramped against a stepped
   control.

Citations: Linkwitz, "Active Crossover Networks for Noncoincident Drivers", JAES 24(1), 1976 (the
LR4 sum is an all-pass); Zavalishin, *The Art of VA Filter Design*, rev. 2.1.2, ch. 3-4 (the TPT
SVF); Wishnick, "Time-Varying Filters for Musical Applications", DAFx-14 (a contractive transition
matrix bounds the state under arbitrary coefficient variation).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs`, `crates/multiband-compressor/tests/`
- `sdk/assets/miso-engine-v1-parameter-metadata.json`, `sdk/src/generated/` (generated output only)

## Non-goals

- A third band, or more than one crossover per channel.
- Prepared targets for the multiband, or any host admission change.
- Console eligibility, the live bypass shunt (*Give the multiband compressor a live bypass shunt*,
  #1340), the tail contract.
- Fixing the browser's stale `AutomationRate::None` comments (`hosts/host-web/src/lib.rs:1043`).

## Hazards

- The whole-bank `ramping` predicate puts every lane on the ramped path for 64 samples when one lane's
  crossover moves. That couples cost, not bits: the settled lanes add `+0.0` steps to positive
  words.

## Objective gates

1. **Designer total.** Delivered by #1366 (`tests/designer_total.rs`); it stays green.
2. **The band sum stays continuous.** At 48 kHz, scalar, both thresholds at 0 dB, makeup 0 dB, and a
   1 kHz sine at amplitude 0.5 (below the knee, so both band gains are exactly 1; the test asserts
   that). Glides 200 Hz to 4 kHz, 4 kHz to 200 Hz, and 80 Hz to 8 kHz, each started mid-block.
   (a) On every sample, `|y - ap_ref| <= 2^-22 * max(|low|, |high|, |ap_ref|)`, where `ap_ref` is a
   single `svf_step` stage driven by the same word ramp; (b) the largest `|y[n] - y[n-1]|` over the
   glide and the 128 samples after it is at most twice the larger of the same measure for the
   static renders at the start and end crossovers. A mutation that assigns the target words at once
   instead of ramping them turns (b) red; the PR records the measured margins.
3. **Bank lanes keep their bits.** At `Simd4` and `Simd8`, a full bank where one lane glides its
   crossover, another is cut mid-glide by a second crossover span, and the rest ramp other rows or
   rest; each lane is bit-identical to its scalar instance fed the same spans, whole and in chunked
   blocks of 37 frames.
4. **Mid-glide payload.** A track snapshotted 20 samples into a glide and restored into a fresh
   instance renders bit-identically to the uninterrupted one; a payload whose current triple fails
   `RAMP_PATH_NORM_TOLERANCE`, or whose target words differ from the design of its `crossover_hz`,
   is refused and leaves state unchanged.
5. **Realtime.** A block that applies a crossover span on eight lanes makes zero allocations and
   frees (`bench_support::alloc` thread-scoped counters, statics warmed); the existing
   `tests/no_alloc_render.rs` covers the ramped path.
6. The randomized differential (`tests/randomized.rs`) runs at full strength (after #1069) with
   crossover records in its generator, and passes.
7. Commands:
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p multiband-compressor -p parametric-eq -p conformance --features math/lane,parametric-eq/test-support,lane/test-support`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-sdk-generated.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all -- --check`
   - `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh`
   - the browser legs: `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`,
     `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`,
     `bash scripts/test-web-audioworklet.sh`

No checked-in render digest moves: no fixture ramps the crossover, and the flat path is unchanged.
`crates/multiband-compressor/src/corpus_digests.in` stays as it is; a moved digest is a defect.

## Test value

- Gate 2: words assigned at once (a click), or a band split whose sum is no longer the first stage's
  all-pass under time variation (for example a second stage fed different words), turns it red.
- Gate 3: crossover words cached at a segment's first sample, as the band coefficients were before
  #1069, move a lane's bits when a neighbour cuts the segment.
- Gate 4: a payload that omits the word ramps, or accepts forged unstable words, turns it red.
- Gate 6 is judged by reach: its generator now reaches crossover glides and cuts.

## Dependencies

- *Prove the crossover designer total and share the SVF ramp stability check in effect-runtime*
  (#1366).
- *Multiband compressor: a ramp's cut moves a lane's bits in a bank* (#1069): gate 3 and the full
  differential need its per-lane segment rule.
- *Carry live-controlled effect lanes across a plan swap* (#1280) and *Carry per-node effect
  instances across a plan swap* (#1282): stream A's carry slices for banked and per-node multiband
  lanes; this slice changes the payload they move, so it lands after them and its gate 4 covers the
  new words.
