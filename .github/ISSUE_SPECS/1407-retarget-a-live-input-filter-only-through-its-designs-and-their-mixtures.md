# Retarget a live input filter only through its designs and their mixtures

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-05 from the blocker that stopped *State a bounded tail and an exact-rest bound for
every node* (#1329) in attempt 1. Code anchors verified on `main` at `6fb211594`; the cited files
are unchanged on `codex/d15-stream-g` at `3b0fa85ef`.

## Product outcome

A live HPF/LPF edit on the builtin input section can only ever put a recursion word into the filter
that is a designed filter, the disabled identity at rest, or a linear mixture of designs. A disable
becomes a 64-sample crossfade from the filtered to the dry signal, with the filter running at its
own design; an enable from rest becomes the reverse crossfade. Re-sending the same target is a
no-op on a ramp in flight. With this, the section is provably stable in `f32` under every control
history the hosts admit, a disable settles and elides again even while hosts keep re-sending it, and
#1329 can bound the tail of a strip with a live input lane.

## Context

- **The retarget law today.** `InputStage::apply_prepared_filter` (`crates/builtins/src/lib.rs:1351-1426`)
  compares each of the six target words `[c1, a2, a3, m0, m1, m2]` with the lane's *current*
  (mid-ramp) word and, if any differs, restarts a ramp: `step = (target - current) * (1/64)`,
  countdown `INPUT_FILTER_RAMP_SAMPLES = 64` (`crates/builtins/src/filter_control.rs:9`). Both
  public entry points delegate to it: `InputBuiltins::apply_prepared_filter` (`lib.rs:3371`) and
  `BuiltinInputBank::apply_prepared_filter` (`lib.rs:3659`).
- **The kernel.** `input_chain_ramp_block_filter` (`crates/lane/src/kernels/builtins.rs:794-893`)
  and its mono twin (`:915-990`) use the current words for the frame, then advance them:
  `current = select(done, target, current + step)` (`:872-877`, `:973-978`). When a lane's ramp
  completes onto the identity target (`current_target_identity`, `:898`), it clears that lane's
  integrators (`:878-883`, `:979-981`). `refresh_filter_plan` (`lib.rs:1263-1279`) keeps an
  in-flight section non-elidable; `settle_filter` (`:1313-1349`) snaps settled lanes.
- **The disabled target** is exactly `[0, 0, 0, 1, 0, 0]` (`validate_prepared_input_filter_target`,
  `filter_control.rs:76-102`). The recursion step with zero input is the linear map
  `A(w) = [[1 - 2 c1, -2 a2], [2 a2, 1 - 2 a3]]` on `(ic1, ic2)`; at the identity words `A = I`.
- **Who sends targets.** Today only the browser host and the SDK. A session quantum can be any
  nonzero value (`crates/session/src/validate.rs:48`); the SDK asks the browser for a render size
  equal to it (`sdk/src/browser/engine.ts:531`); the C ABI renders one quantum per call. Host-core
  marks a section dirty on any edit, even an unchanged one (`apply_input_filter_edit`,
  `crates/host-core/src/control_preparation.rs:157-195`, dirty bits at `:189-190`), and the atomic
  pair edit (parameter id 0) re-sends both sections (`InputFilterPreparer::prepare`, `:203-290`).
  The drain applies every record at the next block boundary.
- **The blocker (verified 2026-10-05, evidence `/tmp/claude-1002/v1329-blocker/`).** A 15 Hz LPF
  at 48 kHz, disabled and re-sent every 1-frame block: each restart shrinks the recursion words by
  63/64 per frame toward the identity. Worst one-step `f32` V-norm ratio over 2M states against the
  exact operator norm: block 800 `+9.40e-9` (exact `-4.76e-9`), block 1000 `+4.08e-10`
  (`-2.04e-10`), block 1400 `+7.49e-13` (`-3.75e-13`); over frames 700-1500 the `f32` trajectory
  grows by `+1.45e-6` where the exact one decays by `-1.45e-6`. The V-norm is
  `||x||_V^2 = x1^2 + sqrt(2) x1 x2 + x2^2` (the shared eigenbasis of every `k = sqrt(2)` design,
  condition number `kappa = 2.414`). Further defects: while re-sends continue the ramp never
  completes (`c1`, `a2` stuck at subnormal `0x00000020`, `m0` at `0x3f7fffe0`), `changed` stays
  true, the section never elides and its integrators are never cleared, which breaks the #808
  contract "sample A+64 uses the exact target"; it settles 64 frames after re-sends stop.
- **No re-send is needed to leave the stable set.** At quantum 1, a design for one block and then a
  disable puts frame 63 of the identity ramp at 1/4096 of the design: exact `q - 1` is `-1.13e-7`
  (96 kHz), `-1.23e-7` (88.2 kHz), `-2.26e-7` (48 kHz), `-2.46e-7` (44.1 kHz), all inside #1329
  D3's `f32` inflation `6 * 2^-24 * kappa = 8.6e-7`. So removing duplicate re-sends alone does not
  fix the law.
- **The contract being restored.** `docs/rulings/builtins-input-liveness-d2.md:6-26` (#808
  amendment): "Sample A uses current words; sample A+64 uses the exact target. Disabled completion
  clears only the addressed integrators before the first identity sample."
- **Readback for tests** (feature `test-support`): `input_section_words` (`lib.rs:5413`, the
  *current* words `[c1, a2, a3, k, m0, m1, m2]` per section, order `SvfSection::words`, `:763-773`),
  `input_state_words` (`:5435`), `input_elision_plan` (`:5465`), and the bank forms
  `bank_lane_state_words` (`:5477`) and `bank_elision_plan` (`:5471`).
- **Related open slices.** *Elide a builtin input filter section again after a live disable settles
  it to identity* (#1268, stream F) measures elision after a single disable on `main`; this slice
  changes the disable path, so #1268 measures after it. *Apply value-only input HPF and LPF edits
  to the running C ABI plan through prepared targets* (#1262) adds the C ABI producer.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-05).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), approved option (m), the live filter
  retarget law, as this issue, rules 1-4 below. Rationale: every reachable recursion word becomes a
  design or a mixture of designs; it restores the #808 contract; it adds no state, no latency and
  no sealed-size change; live enable/disable becomes the standard 64-sample bypass declick.
  Stream G files and owns it.
- **D1. Rule 1 (in-flight re-send).** A target whose six words equal, bit for bit, the lane's
  in-flight target on that channel, on a lane whose countdown is non-zero, leaves that lane and
  channel untouched: current, target, step and countdown words keep their bits. A lane whose
  countdown is zero (settled) keeps today's rule. Rule 1 must not reach settled lanes: the probe
  that applied it to settled lanes broke
  `crates/builtins/src/tests.rs:288` `trim_refresh_preserves_asymmetric_settled_filter_steps`.
- **D2. Rule 2 (disable).** A target equal to the identity `[0, 0, 0, 1, 0, 0]`, when the lane's
  current words are not already the identity, sets the target words as today, gives `c1`, `a2`,
  `a3` a step of `+0.0` (they stay frozen at their current words), ramps only `m0`, `m1`, `m2` with
  today's step `(target - current) * (1/64)`, and starts the 64-frame countdown. At completion the
  kernel's existing `select(done, target, ...)` snaps the recursion words to `0` and its existing
  identity mask clears both integrators, as today. No kernel change.
- **D3. Rule 3 (enable from rest).** A non-identity target on a lane and channel that is *settled
  disabled* — countdown zero, all six current words bitwise the identity, and both integrators
  `+0.0` (bits `0x00000000`) — first writes the target's `c1`, `a2`, `a3` into the current words
  (so they take step `+0.0`) and then ramps only `m0`, `m1`, `m2` over 64 frames. The predicate
  reads all three conditions; checking only `c1 = a2 = a3 = 0` (the probe's shortcut) is not
  enough, because a restored identity section can hold non-zero integrators, and jumping its
  recursion would release that state through the new design. Such a section takes rule 4.
- **D4. Rule 4.** Every other retarget follows today's rule unchanged: all six words ramp from the
  current words to the target over 64 frames.
- **D5. Scope of the change.** Only `InputStage::apply_prepared_filter` and its doc comment change
  (about 30 lines; probe diff `/tmp/claude-1002/v1329-blocker/option-m.diff`). Each channel the
  selector covers decides its rule from its own words, so a lane whose channels hold equal words
  keeps equal words and stays symmetric. `refresh_filter_plan`, `refresh_channel_symmetry`,
  `settle_filter`, the kernels, the countdown, the prepared target validation, the host-core
  preparer and every sealed size are unchanged.
- **D6. Bits move only on live enable/disable.** A disable renders a per-sample linear blend of the
  filtered output (filter at its own design) and the dry input; an enable from rest the reverse;
  a design-to-design retarget and every settled path render today's bits. No pinned artifact is
  expected to move (the probe left `audit fixture-builtins --check` and `test-debug-a` green). If
  one moves, stop and list it in the PR.

## DSP evidence (AGENTS.md)

- **Equations:** TPT SVF in stored A1 form [SIMPER-SVF] [ZAVALISHIN-TPT]: `v3 = v0 - ic2`,
  `d1 = -c1 ic1 + a2 v3`, `d2 = a3 v3 + a2 ic1`, `n1 = ic1 + 2 d1`, `n2 = ic2 + 2 d2`, joint flush
  (#1328), output `y = m0 v0 + m1 v1 + m2 v2`. With zero input the state step is the affine-in-words
  map `A(w)` above. All builtin designs share `k = sqrt(2)`, hence one eigenbasis `V` and one norm
  `||.||_V`; each design has `||A(d)||_V < 1`. Because `A(w)` is affine in `(c1, a2, a3)`, a word
  `w = (1 - t) d1 + t d2` has `||A(w)||_V <= max(||A(d1)||_V, ||A(d2)||_V)` (triangle
  inequality), and by induction every word reached by ramps, freezes and restarts among designs lies
  in their convex hull, whose norm maximum is at a design. Under today's law, ramps toward the
  identity (`A = I`, norm 1) and restart chains reach words whose margin `1 - ||A||_V` is below the
  `f32` inflation; under D1-D4 no history reaches the identity's recursion words except through the
  completion snap, which clears the integrators in the same step.
- **Coefficient and update rules:** D1-D4. The disable crossfade is `y = (1 - t) y_filtered + t v0`
  with `t = n / 64` up to the rounding of the interpolated mix words (the bypass declick of
  [SMITH-SASP]'s crossfade form, applied to the mix row only).
- **Numerical limits (measured with the probe, `/tmp/claude-1002/v1329-blocker/m.txt`):** random
  restart chains at quanta 1-63, worst exact `q - 1` over reachable recursion words: `-5.21e-5`
  (44.1 kHz), `-5.20e-5` (48 kHz), `-5.21e-5` (88.2 kHz), `-5.24e-5` (96 kHz), which is the
  slowest design; sampled `f32` within `2.5e-7` of exact, about 60x margin against `8.6e-7`.
  No subnormal recursion word is reachable (today's chains reach `0x00000020`).
- **Latency and tail:** latency 0, unchanged. A disable's filtered contribution reaches weight 0 at
  the completion frame, where the integrators clear; the section then elides. The tail bound is
  #1329's.
- **Units and smoothing:** 64 updates, fixed (`INPUT_FILTER_RAMP_SAMPLES`): 1.45 ms at 44.1 kHz,
  0.67 ms at 96 kHz.
- **Denormal/NaN:** targets are validated normal-or-zero and finite (`filter_control.rs:76-102`);
  rule 3 copies validated words, rule 2 freezes existing ones, so no new non-finite or subnormal
  word arises. The per-block non-finite check is unchanged.
- **Citations:** [SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP] (finite-wordlength effects and
  stability of recursive filters), [SMITH-SASP] (crossfades), all in `dsp-research/BIBLIOGRAPHY.md`.
- **Fixtures and objective tests:** gates 1-4 below, in `crates/builtins/tests/filter_liveness.rs`.
- **Benchmarks:** none; the change is on the control drain (per admitted target), not per frame, and
  the render kernel is unchanged.
- **Listening:** none run. The audible change is a live enable/disable: a 64-sample crossfade
  instead of a 64-sample coefficient sweep through near-identity words, the standard bypass
  declick.

## Deliverables

1. Rules 1-4 in `InputStage::apply_prepared_filter`, with its doc comment stating them.
2. Gates 1-4 as tests in `crates/builtins/tests/filter_liveness.rs`.
3. The #808 paragraph of `docs/rulings/builtins-input-liveness-d2.md` states rules 1-4 and that
   sample A+64 uses the exact target under any re-send history.
4. This spec's evidence: the gate-4 per-rate maxima, recomputed by the implementer.

## Authorized paths

- `crates/builtins/src/lib.rs` (`InputStage::apply_prepared_filter` and its doc comment only)
- `crates/builtins/tests/filter_liveness.rs`
- `docs/rulings/builtins-input-liveness-d2.md` (the #808 amendment paragraph only)
- this spec

`crates/builtins` is stream A's column: this is a stream G named exception, recorded in
`docs/handoffs/decision-15-2026-10-05/STREAMS.md`.

## Non-goals

- The tail and exact-rest values (#1329), the elision measurement of #1268, the C ABI producer
  (#1262), any change to host-core admission, the kernels or the 64-update window.
- Rejected alternatives (root decision, 2026-10-05):
  - (a) Hold a new target until the ramp in flight completes: adds up to 63 samples of control
    latency; the pending slot breaks the C ABI builtin bank ceiling (15,833 bytes against 15,680),
    moves the direct-route plan bytes from 6,503 to 7,303, fails `audit capi`'s ABI checks, and
    supersedes targets that were already acknowledged.
  - (b) A budget argument that the defect stays below the tail floor: certifies a defective law.
  - (c) Live input-filter lanes only at quanta of 64 or more: drops low-latency hosts.
  - Removing duplicate re-sends in host-core alone: the 1/4096 case (one block of a design, then a
    disable) remains.

## Hazards

- Rule 1 applies only to in-flight lanes (D1); `trim_refresh_preserves_asymmetric_settled_filter_steps`
  is the canary and must stay green unchanged.
- Rule 3's predicate reads the integrators (D3), not only the recursion words.
- The `+0.0` steps of D2 and D3 must be written for every covered channel, so channel symmetry
  (`refresh_channel_symmetry`, `lib.rs:1079`, and its per-lane predicate
  `compute_lane_channel_symmetry`, `:2330`, compare the step words) sees equal words on a
  symmetric lane.
- The browser artifact compiles this function: run the worklet chain (gate 5). Stream A owns
  `crates/builtins`; #1329 edits the same file and the same ruling paragraph after this slice.

## Objective gates

1. **Re-sent disable elides on time** (`filter_liveness.rs`, every launch rate): `InputBuiltins`
   with LPF 15 Hz (and, separately, HPF 1 kHz) settled, then at quantum 1 apply the disable target
   before every 1-frame block. Assert: after exactly 64 frames from the first disable the section's
   six current words are the identity, both integrators are `+0.0`, and `input_elision_plan` elides
   it; it stays so for 64 more re-sent frames. Repeat on a `BuiltinInputBank` (`Backend::Simd4`, 4
   members) with the command on one member. Red on revert (today it never completes).
2. **Disable freezes the recursion** (every launch rate, both sections): settled design, then one
   disable at quantum 1; for each of the 64 frames the `c1`, `a2`, `a3` current words equal the
   pre-disable words bit for bit, while `m0`, `m1`, `m2` move. Also with a disable applied while a
   design-to-design ramp is in flight (frame 20 of 64): the recursion words then stay at that
   frame's interior words. Red on revert.
3. **Enable from rest** (every launch rate, both sections): from the prepared disabled state, apply
   a design target and render one frame of non-zero input. Assert the current `c1`, `a2`, `a3`
   equal the target's after the first frame, the mix words are the identity mix advanced by one
   step, and `output[0]` equals the trimmed input bit for bit. A second case restores non-zero
   integrators onto the identity section (`set_input_state_words`) and asserts rule 4 (all six
   words ramp; `c1` after frame 0 is `target_c1 / 64` rounded as the kernel does). Red on revert.
4. **Reachable-word scan** (release, every launch rate; debug runs 48 kHz with quanta {1, 2, 7, 63}
   only): for every quantum 1..=63, 16 seeded histories of 512 blocks each drawn from {enable to a
   design with log-uniform cutoff in `[10 Hz, max]`, disable, re-send the in-flight target,
   design-to-design retarget}, for both sections. After every frame read the `c1`, `a2`, `a3` words
   and evaluate `||A(w)||_V` in `f64`. Assert every value `<= q_design + 6 * 2^-24 * kappa`, where
   `q_design` is the largest `||A(d)||_V` over the designs the history used and `kappa = 2.414`.
   Record the per-rate maxima in this spec. Red on revert (restart chains exceed it).
5. Commands:
   - `cargo test --locked --all-targets -p lane -p builtins -p dsp-reference --features builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - the worklet chain, as in `qualification.yml`:
     `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
     `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-builtins-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a law that restarts a ramp on an identical re-send never completes the disable, never
  clears the integrators and never elides; no test re-sends a target today.
- Gate 2: a disable that ramps the recursion words toward the identity (today's law, the source of
  the near-identity words) is red, including from an interior word.
- Gate 3: an enable that sweeps the recursion up from the identity, or a rule-3 predicate that
  ignores the integrators and releases restored state through the new design, is red.
- Gate 4: judged by what its generator reaches: re-sends, mid-ramp disables and retargets at every
  quantum below 64, the histories that reach the near-identity words; any rule that lets a word
  leave the convex hull of designs exceeds the bound.

## Dependencies

none

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

**Change.** Rules 1-4 in `InputStage::apply_prepared_filter` (`crates/builtins/src/lib.rs`) with
its doc comment; gates 1-4 in `crates/builtins/tests/filter_liveness.rs`; the #808 paragraph of
`docs/rulings/builtins-input-liveness-d2.md`. Kernels, `refresh_filter_plan`,
`refresh_channel_symmetry`, `settle_filter`, validation, host-core and every sealed size are
unchanged. Rule 2 restarts the countdown unconditionally (D2 "starts the 64-frame countdown"),
so a disable always reaches the kernel's completion snap and integrator clear.

**Gates 1-4** (`filter_liveness.rs`):
- Gate 1 `a_disable_re_sent_every_frame_completes_clears_and_elides_on_time`: every launch rate,
  LPF 15 Hz and HPF 1 kHz; on `InputBuiltins` the six current words, both integrators and the
  elision plan read settled exactly from frame 64 through frame 128 and not before. On the
  `Simd4` 4-member bank there is no per-lane coefficient reader, so the gate reads the member's
  integrators and `bank_elision_plan` (an elided section is identity on every member with `+0.0`
  integrators), and checks the member renders the dry input from frame A+64.
- Gate 2 `a_disable_freezes_the_recursion_words_and_moves_only_the_mix`: frames 1-63 keep
  `c1`, `a2`, `a3` bit-equal to the pre-disable words (settled design and frame-20 interior word),
  the mix moves every frame, and frame 64 reads the identity with `+0.0` integrators.
- Gate 3 `an_enable_from_rest_jumps_the_recursion_only_when_the_integrators_are_zero`: both cases
  as specified.
- Gate 4 `every_reachable_recursion_word_stays_inside_the_hull_of_the_designs`: as specified; a
  q-frame block is rendered as q one-frame calls so the words are read after every frame (the
  ramp is partition-invariant, `filter_ramp_endpoint_is_partition_invariant_and_reset_honors_kind`),
  and commands land every q frames. Each block draws one command per section. One design draw in
  eight takes an interval endpoint exactly (10 Hz or the maximum), because the maximum cutoff is
  the design with the largest norm and a log-uniform draw rarely reaches it. A word with zero
  recursion is excluded from the norm and instead must carry `+0.0` integrators. `||A(w)||_V` is
  the spectral norm of `R A R^-1`, `R = [[1, 1/sqrt(2)], [0, 1/sqrt(2)]]`; for designs it equals
  the spectral radius (checked off-line at 10 Hz to the maximum cutoff).

**Gate-4 per-rate maxima (release, quanta 1-63, 16 histories x 512 blocks, both sections):**

| Rate | max reached `||A(w)||_V - 1` | max design `q - 1` | max excess over the history's design | bound |
|---|---|---|---|---|
| 44.1 kHz | `-5.213e-5` | `-5.213e-5` | `0` | `8.633e-7` |
| 48 kHz | `-5.241e-5` | `-5.241e-5` | `0` | `8.633e-7` |
| 88.2 kHz | `-5.213e-5` | `-5.213e-5` | `0` | `8.633e-7` |
| 96 kHz | `-5.241e-5` | `-5.241e-5` | `0` | `8.633e-7` |

The largest reached norm is a design's own (the maximum-cutoff design); no interpolated `f32`
word exceeded its history's largest design. Debug runs 48 kHz at quanta {1, 2, 7, 63}.

**Mutation evidence** (each applied to `lib.rs`, run, reverted):
- Full revert to today's law: gates 1, 2, 3, 4 red (gate 4: `rate 44100 quantum 1 history 0
  section 1: reached -1.574e-5 over design -5.213e-5 by 3.639e-5`; gate 1: not settled at frame
  64).
- Rule 1 removed: gates 1 and 4 red.
- Rule 2 removed (disable ramps all six words): gates 2 and 4 red.
- Rule 3 removed: gates 3 (first case) and 4 red.
- Rule 3 predicate ignoring the integrators: gate 3 (restored case) red.
- Rule 1 extended to settled lanes: `tests::trim_refresh_preserves_asymmetric_settled_filter_steps`
  red (the D1 hazard canary); green and unchanged with the implementation.

**Rendered bits that move (before/after, one-off comparison, not committed).** From a prepared
settled design (HPF 1 kHz, LPF 1 kHz, LPF 15 Hz; every launch rate; two-sine input):
- Disable: frame A and every frame from A+64 are bit-identical before and after; frames A+1 to
  A+63 move. After, the output equals `(1 - n/64) y_filtered + (n/64) x` within `3.8e-7`
  (HPF) and `5.3e-8` (LPF) of an `f64` blend of a parallel filter left at its design; before, the
  deviation from that blend was up to `1.24e-1` (1 kHz) and `1.2e-4` (15 Hz) -- the coefficient
  sweep through near-identity words.
- Enable from rest: frame A is bit-identical (dry); every later frame moves, because the filter now
  starts at its design from zero state. After, the output equals `(1 - n/64) x + (n/64) y_design`
  within `3.2e-7` of an `f64` blend with a prepared design filter; before, up to `1.94e-1`.
- Design-to-design retarget from a prepared design (1 kHz to 2 kHz, 15 Hz to 30 Hz): 200 frames
  bit-identical at every rate. Settled paths are untouched code.

**Gate 5 commands and results (x86-64-v3 host):**
- `cargo test --locked --all-targets -p lane -p builtins -p dsp-reference --features builtins/test-support,lane/test-support`: pass (223 passed, 0 failed).
- `cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness`: pass (13 passed).
- `test-debug-a` workspace command from `qualification.yml`: pass (1,426 passed, 0 failed).
- `cargo build --locked --release -p audit && bash scripts/check-builtins-fixtures.sh . target/release/audit`: `builtins fixtures: ok (50 files)`; no pinned artifact moved.
- Worklet chain (build `--named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`): pass.
- `scripts/check-cross-targets.sh`: PASS (AArch64 rows check/lint only; AArch64 tests run in CI).
- `check-builtins-policy.sh`, `check-workspace-policy.sh`: ok. `cargo clippy --locked --workspace --all-targets -- -D warnings`: clean. `cargo fmt --all -- --check`: clean.

**Open:** no listening run (spec: none required). AArch64 test execution is CI-only.
