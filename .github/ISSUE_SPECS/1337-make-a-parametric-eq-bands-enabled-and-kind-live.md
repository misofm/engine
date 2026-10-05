# Make a parametric EQ band's enabled and kind live

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-13 E2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host switches an EQ band on or off, or changes its kind (bell, shelves, low pass, high pass,
notch), as a live update on both hosts. The band's six SVF words ramp over the EQ's 64-sample
prepared-target ramp on the same integrator state, with no plan rebuild, and a banked lane renders
the same bits as its per-node instance. Decision 14 F2's last EQ rows (ids 1, 2, 17, 18, 33, 34, 49,
50) leave the prepared class.

## Context

- **Descriptor.** `parameter()` sets `automatable = field >= 2`
  (`crates/parametric-eq/src/lib.rs:491-492`), so `enabled` and `kind` declare
  `AutomationRate::None`, no smoothing and 0 samples (`:505-528`). The HPF and LPF `enabled` rows
  are already `Block`, `Linear`, `RAMP_SAMPLES` (`cut_parameter`, `:530-562`).
- **The one render refusal.** `apply_target_lane` refuses a general-band target whose `enabled` or
  `kind` differs from the prepared value, with `EffectTargetError::Domain` (`:2869-2883`). Nothing
  else refuses: the decoder accepts any band `enabled`/`kind` (`control.rs:33-93`), the validator
  requires a disabled band's words to be exactly `IDENTITY` (`validate_target_coefficients`,
  `control.rs:328`), and the preparer designs every kind at every launch rate (test
  `enabled_targets_validate_each_kind_at_each_launch_rate`, `control.rs:643`).
- **Hosts are descriptor-driven.** The C ABI classifier returns `LiveRebuild::Prepared` for a
  non-`Block` row (`crates/host-core/src/live_delta.rs:441`) and `EqTargetPreparer` refuses it
  (`crates/host-core/src/control_preparation.rs:406`); the browser refuses it with
  `UNSUPPORTED_KIND` (`hosts/host-web/src/lib.rs:4519`). Each path admits the rows once the
  descriptor says `Block` and automatable, with no host code change.
- **Words.** A disabled band designs to `EqSvfWords::IDENTITY` (`c1 = a2 = a3 = 0`, `m = (1, 0, 0)`,
  `:700-707`; `BandTarget::words`, `:939-944`). Kinds differ in `g`, `k` and the mix
  `(m0, m1, m2)` (`design_svf_words_f64`, `:747-794`). `start_ramp` ramps all six words linearly from
  the words in force (`:1441-1462`).
- **Stability.** `word_spectral_norm` (`:796`) is the norm of the state matrix the kernel iterates.
  It is convex in the words, so a ramp between two contractive endpoints stays contractive
  (`:789-794`, `:812-835`). It depends on `c1, a2, a3` only, so a change of mix cannot destabilise.
- **A disabled section freezes its state.** With identity words `d1 = d2 = 0` in `svf_step`
  (`crates/lane/src/kernels.rs:643-653`), so the integrators hold. The test
  `a_band_switched_off_keeps_the_bank_eliding` (`lib.rs:5150-5161`, #979) relies on that frozen,
  non-zero state. Today only a live HPF/LPF disable reaches it. Re-enabling such a section resumes
  from a state of a signal long gone.
- **Elision.** A section that is the identity on every lane is skipped on stationary blocks
  (`dead`, `:1883`, `:2096`, `:2376`). This keeps a default (disabled) band free.
- **The payload omits them.** The lane payload holds four numeric fields per band plus the two cut
  enables (`:94-100`); restore takes band `enabled` and `kind` from `self.initial`
  (`:2663-2664`, `:3405-3408`). Once they are live, a lane can differ from `initial`.

## Decisions frozen for this slice

- **D1. Descriptor.** Band `enabled` and `kind` become `AutomationRate::Block`, `automatable: true`,
  `SmoothingRule::Linear`, `smoothing_samples: RAMP_SAMPLES` (64), exactly as `cut_parameter`. Rule
  4 holds: they are live, hence may be automated.
- **D2. Remove the refusal.** `apply_target_lane` drops its `permitted` check. A target's `enabled`
  and `kind` are applied like its numeric fields: `targets[track][section]` takes the new
  `BandTarget` and `start_ramp` ramps the six words. The HPF and LPF keep their fixed kinds (the
  decoder already refuses another).
- **D3. One SVF state through every change.** No change of kind or enable resets or copies state,
  except D4. A kind change on an enabled band ramps `c1, a2, a3, m0, m1, m2` from the words in force
  to the new design over 64 samples. Disable ramps to `IDENTITY`; enable ramps from it.
- **D4. Re-enable starts from rest.** When a target arrives for a lane whose section words are
  `IDENTITY` to the bit with no ramp in flight, and the new words are not `IDENTITY`, the lane's
  `ic1` and `ic2` in that section are set to `+0.0` (`lane_put` through the lane's mask) before the
  ramp starts. This applies to every section, the HPF and LPF included. Reason: the frozen state
  belongs to an old signal; `+0.0` is the state a band prepared disabled has, so a re-enabled lane
  renders the same bits as one prepared disabled and enabled at the same block, whatever its
  history. A disabled band keeps `IDENTITY` words, so elision stays (running disabled bands to keep
  a fresh state would cost four SVF sections per lane on every default EQ; rule 3 optimisation).
- **D5. Payload.** Each lane section appends eight words after the LPF enable word: for band
  `b` in `0..4`, word `116 + 2b` is `enabled` (0 or 1) and `117 + 2b` is `kind` (1-6). Restore
  builds each band's configuration from these words, not from `initial`, and refuses another value
  with `effect.state.payload`. `STATE_LAYOUT_VERSION` stays 1: nothing persists the payload (R6b),
  and the header's data word count refuses an old-length payload. The `configuration` rustdoc
  (`:2663`) is corrected.
- **D6. Banking never moves a lane's bits.** D2-D4 write only the addressed lane, through its mask.
  A lane ramping its kind still makes its section take the ramped kernel for 64 samples for the
  whole bank; that couples cost, not bits.
- **D7. Carry (D15-7).** The successor restores the predecessor lane's payload, which now carries
  `enabled` and `kind` (D5); pending targets move with the inherited queue (#1280).

**Equations.** Simper's trapezoidal SVF (Cytomic, 2013), RBJ cookbook transfers: `g = tan(pi f0 /
fs)`, `k` the damping, `y = m0 x + m1 v1 + m2 v2`; stored as `c1 = t / (1 + t)`, `t = g (g + k)`,
`a2 = g (1 - c1)`, `a3 = g a2` (`:747-794`). **Update rule:** per sample `w += (w_target -
w_start) / 64` (a multiply by `2^-6`), target assigned exactly at sample 64. **Stability:** every
point of the ramp has `||M||_2 <= RAMP_PATH_NORM_TOLERANCE` (`:835`) by convexity; this is the
sufficient condition for a time-varying recursion that Laroche gives ("On the Stability of
Time-Varying Recursive Filters", JAES 55(6), 2007), stronger than frozen-pole stability. Mix words
stay within `OUTPUT_MIX_BOUND` (`:844`) at both ends, hence along the ramp. Background on TPT SVFs
under modulation: Zavalishin, *The Art of VA Filter Design* (2.1.2), and Wishnick, "Time-Varying
Filters for Musical Applications", DAFx-14. **Latency:** 0. **Tail:** unchanged (owned by #1329).
**Denormal/NaN:** unchanged; integrators flushed in `svf_step`, D7 lane recovery
(`recover_failed_lanes`). D4 writes `+0.0`, never `-0.0`.

## Deliverables

1. D1-D6 in `crates/parametric-eq/src/lib.rs` and the tests below.
2. Regenerated `sdk/assets/miso-engine-v1-parameter-metadata.json` and
   `sdk/src/generated/catalog.ts` (`node sdk/codegen/assets.mjs`, then
   `node sdk/codegen/generate.mjs`); no hand edits.
3. A short listening note in the PR: a 1 kHz +9 dB bell toggled and switched bell to notch on a
   music stem, and an 80 Hz HPF re-enabled after 2 s off, before (rebuild) and after (live).

## Authorized paths

- `crates/parametric-eq/src/lib.rs`, `crates/parametric-eq/tests/`. Stream A owns the payload code
  in `lib.rs` (`STATE_*` constants, `snapshot_track`, `snapshot_cut_enables`, `decode_track`,
  `commit_track`, `restore_track`) and `tests/carry.rs`: coordinate D5 with the owner of #1279/#1280.
- `crates/conformance/src/randomized.rs`: the prepared-target draw only (gate 6).
- The two regenerated SDK files above. One test line each, coordinated with their owners:
  `sdk/test/live-controls-types.ts:128-129` (stream H), the EQ half of
  `prepared_parameter_changes_need_a_rebuild` in `crates/host-core/tests/live_delta.rs:1068-1107`
  and the doc line `crates/host-core/src/live_delta.rs:128-130` (stream B),
  `crates/capi/src/runtime/live_tests.rs` (one new test, stream B).

## Non-goals

- No grammar, wire or host code change. No change to HPF/LPF kinds, to the numeric fields, or to
  the ramp length (D15-1's session default does not apply to the EQ's fixed 64-update ramp).
- The stale browser comments (`hosts/host-web/src/lib.rs:1041-1043`) are stream H's.
- No change to `svf_step` (*Flush the SVF jointly so builtin and EQ filters reach exact rest*
  (#1328) owns `crates/lane/src/kernels.rs`; no file overlap, either order).

## Hazards

- `a_band_switched_off_keeps_the_bank_eliding` (`lib.rs:5161`) switches on, then off: D4 does not
  touch it. A test that re-enables a frozen HPF would move bits by design; none is known.
- No render digest may move: D4 changes bits only for a lane re-enabled live after a live disable.

## Objective gates

1. **Live end to end.** C ABI: on `eq_session` (`live_tests.rs:2057`), a transaction enabling band 2
   of the console EQ and switching eq-insert band 1 from bell to notch commits as live (no plan
   replacement) and renders bit-identical to a fresh plan of the committed model once the 64-sample
   ramp has passed and the integrators are compared from a common rest (drive both from silence).
   Host-core: the classifier returns records with targets for each of the eight rows.
2. **Settles to the design.** For every kind pair (including disabled) at each launch rate, a
   lane's words 64 samples after the target equal `design_svf` of the new band bit for bit, and its
   `targets` entry equals the new `BandTarget`.
3. **Ramp path stays contractive.** Over a grid of every ordered kind pair (7 states including
   disabled), frequencies {20, 1000, 18000} Hz, gains {-24, 0, 24} dB and Q {0.1, 0.707, 18}, at
   each launch rate, every word set of the `f32` walk satisfies `validate_svf_within(..,
   RAMP_PATH_NORM_TOLERANCE)` or is `IDENTITY`; the output of a full-scale noise input stays finite
   and below `OUTPUT_MIX_BOUND` times the input peak.
4. **Re-enable from rest (D4).** A lane enabled, fed noise, disabled live, fed noise 200 blocks,
   then re-enabled renders bit-identical from the re-enable block on to a lane prepared disabled and
   enabled at the same block. For the HPF section too. At `W = 1` and each native bank width.
5. **Bank bit-identity.** A full bank at `Simd4` and `Simd8` where lane 1 toggles band 2 and lane 3
   changes band 3's kind at different blocks renders each lane bit-identical to its scalar instance;
   the other lanes are bit-identical to an unedited run.
6. **Randomized differential.** `conformance`'s target draw also draws `Boolean` and `Enumeration`
   rows whose rate is `Block`; `tests/randomized.rs` (24 seeds) passes, continued and restored.
7. **Payload (D5).** A lane snapshot taken mid-ramp of a kind change, restored into an instance
   prepared at the old kind, continues bit for bit; payloads with an enable word 2 or a kind word 0
   or 7 are refused and move nothing.
8. **Realtime.** A block that applies an enable, a kind change and a D4 re-enable makes 0
   allocations and frees (`bench_support::alloc` thread counters); `audit parametric-eq` reports
   no violation.
9. **Unchanged.** Every checked-in render digest and the console benchmark workload counts.
10. Commands:
    - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p parametric-eq -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,lane/test-support`
    - `cargo test --locked -p host-core --test live_delta`; `cargo test --locked -p capi`
    - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
    - `cargo build --locked --release -p audit && ./target/release/audit parametric-eq --blocks 100000`
    - `cargo test --locked --release -p audit -p bench -p console-workload`
    - `bash scripts/check-parametric-eq-render-contract.sh`; `bash scripts/check-realtime-policy.sh`;
      `bash scripts/check-effect-runtime-policy.sh`; `bash scripts/check-workspace-policy.sh`
    - `bash scripts/check-sdk-generated.sh`; `bash scripts/check-sdk-types.sh`
    - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`, then
      `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`,
      `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` and
      `bash scripts/test-web-audioworklet.sh`
    - `bash scripts/check-cross-targets.sh`; `scripts/run-aarch64-tests.sh` (CI `aarch64-debug` and
      `aarch64-release` when no arm64 host)
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
      `cargo fmt --all -- --check`

**Superseded and deleted in the same PR:** `prepared_target_rejects_general_enable_change_without_mutation`
(`tests/contract.rs:589`); the `field >= 2` rate assertions of `descriptor_is_frozen`
(`tests/contract.rs:83-100`, rewritten to all-`Block`); the EQ loop of
`prepared_parameter_changes_need_a_rebuild` (rewritten to assert live); the `@ts-expect-error` on
`band-1-kind` (`sdk/test/live-controls-types.ts:128-129`, now a legal call).

## Test value

- Gate 1: a host path that still classifies the rows prepared, or a refusal left in
  `apply_target_lane`, turns it red.
- Gate 2: a target applied to `targets` but ramped to the old kind's words turns it red.
- Gate 3: a kind pair whose word walk leaves the contractive bound (a future designer change)
  turns it red; nothing tests cross-kind ramps today.
- Gate 4: a re-enable that resumes the frozen state turns it red.
- Gate 5: a D4 clear or a target write that reaches a neighbour lane turns it red.
- Gate 7: a restore that still reads `enabled`/`kind` from `initial` refuses or renders the old kind;
  it turns red.
- Gate 8: an allocation on the target path turns its count red.
- Gate 6 is the existing differential; its generator now reaches the rows.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279)
- *Carry live-controlled effect lanes across a plan swap* (#1280)
