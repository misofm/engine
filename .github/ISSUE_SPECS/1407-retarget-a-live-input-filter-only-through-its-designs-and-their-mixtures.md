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
no-op on a ramp in flight. With this (and Amendment 1's ramp arithmetic), every recursion word the
kernel loads lies within a proven `f32` rounding allowance of the convex hull of the designs its
history used, so the section's zero-input step is a contraction, `||A(w)||_V <= 1 - 3.79e-5`, at
every launch rate and block size under every control history the hosts admit; a disable settles and
elides again even while hosts keep re-sending it; a collapsed strip renders exactly the bits of the
same strip rendered dual; and #1329 can bound the tail of a strip with a live input lane.

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

## Amendment 1 (root decisions for attempt 2, 2026-10-05)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`; math delegated), after attempt 1's FAIL verdict
(`/home/bl/misofm/submix-verdicts/1407-attempt1.md`; decisions
`/tmp/claude-1002/-home-bl-misofm-engine/258223fb-6f51-4ebb-b550-04d18c009589/scratchpad/1407-attempt2-decisions.md`).
Where this amendment and the frozen decisions above differ, this amendment holds.

- **A1. The collapsed-stage invariant (verdict BLOCKER 1, option (b)).** "While a stage is
  collapsed, channel `0` is the only live state; no decision may read channel `1`'s state, and every
  `Both` record applies channel `0`'s decision to both channels." The rule-3 predicate reads the
  integrators of the channel the stage actually advanced (channel `0` while collapsed) for both
  channels. Every other read of channel-`1` state on the drain and record-application path while
  collapsed is audited and follows the same rule (attempt 2 lists each site). D3's integrator clause
  stays: it protects restored state. A per-channel record that could make L and R differ must
  desymmetrize before it applies, if such a path exists.
- **A2. Root-authorized extension of D5.** The authorized paths extend to exactly the sites A1's
  audit needs, and to the filter coefficient-word updates of the two filter-ramp bodies in
  `crates/lane/src/kernels/builtins.rs` (A3). #1408's `ramp_toward` clamp and its eight trim, fader
  and matrix sites are not touched.
- **A3. The ramp arithmetic (verdict MAJOR 2: `f32` rule-4 ramps left the hull by up to `2.57e-6`
  against gate 4's `8.63e-7`).** Root decided: try option (iii), each filter ramp word computed as
  `target - step * remaining` from the words the ramp already holds, no new state and no sealed-size
  change, the V8 spill gate clean; fall back to option (i) (restated claims, gate 4 bound a computed
  `q_ramp`) only if (iii) fails those conditions. In both cases the claims are restated as a proven
  bound (an analytic per-frame rounding term plus the contraction argument for chained retargets, in
  closed form, confirmed by the measured worst case), gate 4's bound becomes that computed `q_ramp`
  with its margin, gate 4's endpoint draws stop setting every history's bound, and #1329's D5 gets
  the numeric restatement in the same commit. Applied (implementer's math decision under the
  delegation, recorded in attempt 2): (iii) for every word after the fourth, and the first four
  words stepped from the current word, because the proven bound of (iii) applied to every word is
  `P = 6.10e-5` at block size 1, which exceeds the maximum-cutoff design's margin `5.21e-5`: a word
  computed from the target carries a rounding of nearly the whole `target - start` distance, and a
  host restarting every frame compounds it 64 times. With four stepped words the bound is the floor
  `1.419e-5` at every block size, measured excess stays at the (iii) level, and nothing accumulates
  beyond four additions. The rule-2 freeze moves into the kernel's update as "a recursion word with a
  `+0.0` step holds", because a target-relative word with a zero step is the target, not the current
  word. Design-to-design retargets now move rendered bits (attempt 2 lists them). This supersedes
  D2's "No kernel change", D6's "a design-to-design retarget ... render today's bits", the kernels
  item of "Non-goals" and the "render kernel is unchanged" benchmark note.
  - **Root accepted (2026-10-05).** The four-stepped-word refinement of (iii) (the first four ramp
    words `current + step`, every later word `target - step * remaining`, a `+0.0`-step word held)
    is accepted by root: it is what makes the proven bound `||A(w)||_V <= q_design + 1.419e-5 <=
    1 - 3.79e-5` hold at block size 1, which (iii) on every word (`P = 6.10e-5`) cannot.
- **A4. Folded minors.** Gate 2b pins rule 2's unconditional countdown restart (verdict MINOR 3,
  mutant M2c). The "slowest design" wording becomes the maximum-cutoff design (NIT 5); "equals the
  spectral radius" is stated to its precision (NIT 6); the ruling's lines are held to 100 columns and
  the old #808 tail sentences return before the new rules (NIT 8). The release gate-4 sweep workflow
  is a separate follow-up issue the coordinator files (NIT 9); no workflow changes here.

## DSP evidence (AGENTS.md)

- **Equations:** TPT SVF in stored A1 form [SIMPER-SVF] [ZAVALISHIN-TPT]: `v3 = v0 - ic2`,
  `d1 = -c1 ic1 + a2 v3`, `d2 = a3 v3 + a2 ic1`, `n1 = ic1 + 2 d1`, `n2 = ic2 + 2 d2`, joint flush
  (#1328), output `y = m0 v0 + m1 v1 + m2 v2`. With zero input the state step is the affine-in-words
  map `A(w)` above. All builtin designs share `k = sqrt(2)`, hence one eigenbasis `V` and one norm
  `||.||_V`; each design has `||A(d)||_V < 1`. Because `A(w)` is affine in `(c1, a2, a3)`, a word
  `w = (1 - t) d1 + t d2` has `||A(w)||_V <= max(||A(d1)||_V, ||A(d2)||_V)` (triangle
  inequality), and by induction every word reached by ramps, freezes and restarts among designs lies
  in their convex hull in exact arithmetic, whose norm maximum is at a design. In `f32` every word
  lies within a proven componentwise allowance `E` of that hull ("Numerical limits"), and a word
  `w = h + e` with `h` in the hull has `||A(w)||_V <= ||A(h)||_V + 2 ||[[e1, e2], [-e2, e3]]||_V`.
  Under the pre-#1407 law, ramps toward the identity (`A = I`, norm 1) and restart chains reach
  words whose margin `1 - ||A||_V` is below the `f32` inflation; under D1-D4 no history reaches the
  identity's recursion words except through the completion snap, which clears the integrators in
  the same step.
- **Coefficient and update rules:** D1-D4, and Amendment 1's ramp arithmetic: the first four words
  of a ramp are `current + step`, every later word is `target - step * remaining`, a recursion word
  with a `+0.0` step holds, and the 64th update snaps to the target. The disable crossfade is
  `y = (1 - t) y_filtered + t v0` with `t = n / 64` up to the rounding of the interpolated mix words
  (the bypass declick of [SMITH-SASP]'s crossfade form, applied to the mix row only).
- **Numerical limits (proven; Amendment 1 withdraws attempt 1's sampled "within `2.5e-7`").** A
  ramp from word `c` to design `t` has step `s = fl(t - c) / 64`; its word after frame `j` is within
  `rho_j` of the exact mixture `(1 - j/64) c + (j/64) t`, with `rho_j = j h + (j/64)|t - c| u` for
  the four stepped words (`fl(w + s)`) and `rho_j = (1 - j/64)|t - c|(2u + u^2) + h` after them
  (`fl(t - fl(s * (64 - j)))`); `u = 2^-24`; `h` is the word's half-ulp, `2^-25` for `c1` and `a3`
  (never above `1`), `2^-26` for `a2` (never above `1/(2 + sqrt(2))`). Every word lies between its
  ramp's start and target: `64 s = fl(t - c)` is within a half-ulp of `t - c` (it may round past
  it), but a stepped word is at most four steps from the start and a later word at most 59 steps
  from the target, and rounding is monotone. So `|t - c|` is at most the word's range over every
  design, `D = (1, 0.3, 1)`. A restart from a word `e` off the hull carries `(1 - j/64) e` of it
  (the contraction), so with restarts at least `q` frames apart a restart word is off by at most
  `E_start = max over q <= j <= 63 of (64/j) rho_j` and any word by at most
  `E = max over j of (1 - j/64) E_start + rho_j`; a freeze, a rule-3 jump
  and the snap add nothing. At `q = 1..4` this is the floor `E = 64 h + u D =
  (1.967e-6, 9.716e-7, 1.967e-6)`, which any 64-update law has (64 restarts' final roundings at
  contraction `63/64`). The norm it can add, `P = 2 sup ||[[e1, e2], [-e2, e3]]||_V` over
  `|e_i| <= E_i` (attained at a vertex of the box, the norm being convex), is `1.419e-5` at
  `q <= 4`, `3.85e-6` at `q = 16` and `1.18e-6` at `q = 63`. So every reachable word has
  `||A(w)||_V <= q_design + 1.419e-5`; with the maximum-cutoff design, the largest norm
  (`q - 1 = -5.213e-5` at 44.1/88.2 kHz, `-5.241e-5` at 48/96 kHz), `||A(w)||_V <= 1 - 3.79e-5` at
  every rate, `3.79e-5` of margin before #1329 D3's kernel inflation `8.63e-7`. The same argument
  gives `P = 6.10e-5` at `q = 1` when every word is computed from the target (option (iii) as first
  stated), which does not prove `< 1`, and `1.42e-5` at every `q` for the pre-#1407 accumulated
  law. Measured on gate 4 (release, every rate, quanta 1-63): the worst excess of a reached norm
  over its history's largest design is `0` on log-uniform and endpoint histories and `2.62e-7`
  (44.1/88.2 kHz), `1.96e-7` (48/96 kHz) on close-retarget chains at the maximum cutoff; the
  accumulated law reaches `2.569e-6` on the verifier's close-retarget probe, which this law brings
  to `2.62e-7` (`1.13e-7` with every word from the target). For a design, `||A(d)||_V` equals the
  spectral radius only to within `1.97e-8` (44.1/88.2 kHz) and `3.94e-8` (48/96 kHz) at the
  maximum cutoff, because the `f32` words are not exactly `k = sqrt(2)`-consistent; the bound uses
  the norm. No subnormal recursion word is reachable (gate 4 asserts it; the pre-#1407 chains reach
  `0x00000020`).
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
- **Fixtures and objective tests:** gates 1-4 and 2b below, in
  `crates/builtins/tests/filter_liveness.rs`; gate 6 in `crates/host-core/tests/successor_swap.rs`;
  gate 7 in `crates/lane/tests/filter_ramp_line.rs`.
- **Benchmarks:** none. The retarget law is on the control drain (per admitted target). The ramp
  arithmetic changes only the two filter-ramp bodies, which run for at most 64 frames after a
  retarget: per word a multiply, a subtract and a select beside the add, one compare per section and
  a compare and select per recursion word. The settled path is unchanged.
- **Listening:** none run. The audible change is a live enable/disable: a 64-sample crossfade
  instead of a 64-sample coefficient sweep through near-identity words, the standard bypass
  declick.

## Deliverables

1. Rules 1-4 in `InputStage::apply_prepared_filter`, with its doc comment stating them.
2. Gates 1-4 and 2b as tests in `crates/builtins/tests/filter_liveness.rs`; gate 6 in
   `crates/host-core/tests/successor_swap.rs`; gate 7 in `crates/lane/tests/filter_ramp_line.rs`.
3. The #808 paragraph of `docs/rulings/builtins-input-liveness-d2.md` states rules 1-4, that
   sample A+64 uses the exact target under any re-send history, the ramp arithmetic with its proven
   bound, and the collapsed-stage invariant.
4. This spec's evidence: the gate-4 per-rate maxima, recomputed by the implementer.
5. (Amendment 1) The collapsed-stage invariant (A1) and the ramp arithmetic (A3); #1329's D5
   restated numerically.

## Authorized paths

- `crates/builtins/src/lib.rs` (`InputStage::apply_prepared_filter` and its doc comment; by
  Amendment 1 A2 also the collapsed-stage invariant's sites: the `InputStage::collapsed` flag and
  `live_state_channel`, `refresh_filter_plan`, `process_mono`, `desymmetrize`, and a debug
  assertion in `process`; and A3's per-block leading countdown, `load_filter_leading`, passed to
  the two filter-ramp bodies)
- `crates/lane/src/kernels/builtins.rs` (Amendment 1 A2: the filter coefficient-word updates of
  `input_chain_ramp_block_filter` and `input_chain_ramp_block_filter_mono`, and their helper)
- `crates/builtins/tests/filter_liveness.rs`
- `crates/host-core/tests/successor_swap.rs` (gate 6) and `crates/lane/tests/filter_ramp_line.rs`
  (gate 7)
- `docs/rulings/builtins-input-liveness-d2.md` (the #808 amendment paragraphs only)
- `.github/ISSUE_SPECS/1329-*.md` (D5's numeric restatement only, Amendment 1 A3)
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
   **2b (Amendment 1).** An enable from rest and a disable drained at the same boundary (every
   launch rate, both sections): the section reads identity words, `+0.0` integrators and elided on
   both channels exactly from frame 64 on. Red on M2c (rule 2 restarting the countdown only when the
   mix differs).
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
   and evaluate `||A(w)||_V` in `f64`. Assert every value `<= q_design + P(q)`, where
   `q_design` is the largest `||A(d)||_V` over the designs the history used and `P(q)` is the
   proven allowance at that quantum ("Numerical limits", computed in the test, never pinned).
   Record the per-rate maxima in this spec. Red on revert (restart chains exceed it).
   **Amendment 1.** The log-uniform histories draw no endpoints, so each is held at its own largest
   design norm. Two further histories per quantum draw one design in four at exactly 10 Hz or the
   maximum cutoff, and four close-retarget histories start settled at the maximum cutoff and
   retarget every block: alternating the maximum and the next lower `f32` cutoff, dragging the
   cutoff three ulps a block up and down, alternating the maximum and 1 Hz below it, and the first
   pattern with a disable every sixteenth block and an enable back on the next. Every recursion word
   must be normal or zero. Per rate the test also asserts the stability proof's numeric condition,
   maximum-cutoff design norm plus `P(1)` below `1`.
6. **Collapsed equals dual** (Amendment 1 A1; `successor_swap.rs`, `Simd4`, and `Simd8` where
   `avx2` is on): the all-mono filtered nine-track session at quantum 16, 480 blocks, swapped to and
   from its muted-track successor every 40 blocks with each lane's live filter carried, under three
   seeded random record streams: `Both` enables to random designs, disables and re-sends at 5 % per
   strip per block, and every second plan segment one strip driven apart by a `Left` or `Right`
   enable, then both its sections disabled on both channels so it rests again before the swap. The
   collapsing run must equal the same run with `force_mono_collapse_off` on every plan, block for
   block, bit for bit; the generator must reach collapsed blocks, disengages, carried swaps and M3
   proofs. (No scalar run: the scalar backend has no bank and never collapses.) Red on attempt 1's
   code and on a mutant that reads channel `1` while collapsed.
7. **Ramp words on the line** (Amendment 1 A3; `filter_ramp_line.rs`, every width): random ramps
   (fresh, part-way, settled; disabling with `+0.0` recursion steps) through both filter-ramp
   bodies frame by frame; every word after every frame equals the scalar oracle bit for bit: the
   first four words `start + k * step` stepped, later words `target - step * remaining`, a
   recursion word with a zero step held, the target at countdown zero.
5. Commands:
   - `cargo test --locked --all-targets -p lane -p builtins -p dsp-reference --features builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness`
   - (Amendment 1) `cargo test --locked -p host-core --features test-support --test successor_swap`,
     `cargo test --locked -p lane --features test-support --test filter_ramp_line`,
     `bash scripts/run-wasm-gates.sh` (V8 spill gate included, its four held rows clean) and
     `bash scripts/check-cross-targets.sh`
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
  quantum below 64, the histories that reach the near-identity words, and (Amendment 1) close
  retarget chains at the maximum cutoff; any rule that lets a word leave the convex hull of the
  designs its history used by more than the proven rounding allowance exceeds the bound, and each
  history is held at its own largest design.
- Gate 2b (Amendment 1): a rule 2 that restarts the countdown only when the mix differs leaves an
  enable-then-disable drained together settled on the design recursion under the identity mix,
  never cleared, never elided.
- Gate 6 (Amendment 1): any read of channel `1`'s frozen integrators while a stage is collapsed
  (the rule-3 predicate, the elision plan) makes a collapsed strip leave the bits of the same strip
  rendered dual, which no other test drives through live filter records and carried swaps.
- Gate 7 (Amendment 1): a filter-ramp body that steps every word from the previous one, computes
  every word from the target, changes the stepped window or drops the zero-step hold is red at
  rounding level, at every width and in both bodies.

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

### Attempt 2 (implementer, 2026-10-05)

**Change.** Amendment 1 applied.
- *A1, the collapsed-stage invariant* (`crates/builtins/src/lib.rs`). `InputStage::collapsed` (one
  byte in existing padding: `size_of::<InputStage<_>>` stays 688 / 2,064 / 3,904 bytes at `f32` /
  `Simd4` / `Simd8`, measured before and after) is set by `process_mono` and cleared by
  `desymmetrize`; `live_state_channel(channel)` is channel `0` while it is set. Audit of every read
  of channel-`1` state on the drain and record-application path while collapsed:
  - `apply_prepared_filter`'s rule-3 predicate read `state.section[channel]`: now
    `state.section[live_state_channel(channel)]` (the BLOCKER).
  - `refresh_filter_plan` (called by the drain's `apply_prepared_filter`, and by `process_mono`
    after each ramp block) decided channel `1`'s elision over its frozen integrators, which split
    the plan when a disable completed collapsed and tripped `process_mono`'s
    `plan_is_channel_symmetric` assertion: channel `1` is now decided over channel `0`'s
    integrators while collapsed.
  - `apply_prepared_filter`'s other reads (`coef.section`, `filter_target`, `filter_step`,
    `filter_remaining`), `set_trim_signed` (`TrimDb`, `PolarityInvert`) and the symmetry
    predicates read records, not state; `process_mono` mirrors those records onto channel `1` at
    the bottom of every collapsed block, and nothing else writes them, so at a drain they are the
    dual run's.
  - `export_lane` and `channels_agree` read channel `1`'s integrators but are not reached
    collapsed: a carry calls `disengage_for_carry` (the disengage copy) first, and the M3 proof is
    asked only of a chain rendering dual. Left as they are (a redirect in `export_lane` was tried
    and survives every test, mutant C3, so it was dropped as unreachable). A dual `process` after a
    collapsed block without the copy is now a debug assertion.
  - No record writes integrators, so no per-channel record needs to desymmetrize before it
    applies: a one-channel record decides over channel `0`'s integrators, the witness then declines,
    and the disengage copy hands channel `1` exactly those integrators.
  - D3's integrator clause is kept.
- *A3, the ramp arithmetic* (`crates/lane/src/kernels/builtins.rs`, `filter_ramp_words`, shared
  by `input_chain_ramp_block_filter` and `_mono`). The first four words of a ramp are
  `current + step`; every later word is `target - step * remaining`; a recursion word with a `+0.0`
  step holds (rule 2's freeze: the target-relative form would otherwise jump it to `0`); done snaps
  to the target. The stepped window is driven by a per-lane leading countdown the owner builds per
  block (`InputStage::load_filter_leading`, `remaining - 60`, never stored), not by a splatted
  `60.0`: with the splat, the iOS assembly of `builtins` gained two `memset_pattern16` calls (186 to
  188; the first form, with the splat inside the helper, 190), which `check-cross-targets.sh`
  refuses (known defect #1018). With the countdown word it stays at 186. No new state word, no
  sealed size moved, #1408's clamp and sites untouched.
  - *Why not (iii) on every word:* proven allowance at block size 1 is `P = 6.10e-5` against the
    maximum-cutoff margin `5.21e-5`, so it proves nothing there (the target-relative word rounds
    nearly the whole `target - start` distance, compounded 64 times by a per-frame restart). The
    first trial, only the first word stepped, proved `P = 3.01e-5`; four stepped words reach the
    floor `P = 1.419e-5` (`E = 64 h + u D`) at every block size. Measured excess on the verifier's
    close-retarget probe: `1.13e-7` with every word from the target, `2.62e-7` with four stepped
    words, `2.569e-6` for the accumulated law. So (iii) "met its conditions" (no state, no sealed
    size, spill gate clean) but not the bound; the four-word refinement is the implementer's math
    decision under the delegation, recorded in Amendment 1 A3 for root review.
- *Gate 4* (`filter_liveness.rs`): the bound is `q_design + P(q)` with `P(q)` computed in the test
  from the closed form (`word_allowance`, `norm_allowance`); 16 log-uniform histories with no
  endpoint draws, 2 endpoint histories, 4 close-retarget histories per quantum; every recursion word
  normal or zero; per rate the stability condition `q_top + P(1) < 1` is asserted. Gate 3's
  expected frame-0 words are the stepped form (unchanged from attempt 1). New gate 2b
  `a_disable_in_the_enables_drain_still_completes_and_elides`, gate 6
  `a_collapsed_chain_renders_the_forced_dual_bits_at_{four,eight}_lanes`, gate 7
  `every_filter_ramp_word_lies_on_the_line_to_its_target`.
- Ruling (`docs/rulings/builtins-input-liveness-d2.md`): the #808 paragraphs rewrapped to 100
  columns, the old #808 tail returned before the rules, the ramp arithmetic with its proven bound
  and the collapsed-stage invariant added. #1329's D5 restated numerically.

**Gate-4 per-rate results (release, quanta 1-63, 22 histories x 512 blocks, both sections):**

| Rate | max-cutoff design `q - 1` | norm - spectral radius | `P(1)` | `q_ramp - 1` | worst excess: log-uniform / endpoints / close |
|---|---|---|---|---|---|
| 44.1 kHz | `-5.2133e-5` | `1.97e-8` | `1.4188e-5` | `-3.7945e-5` | `0` / `0` / `2.622e-7` |
| 48 kHz | `-5.2411e-5` | `3.94e-8` | `1.4188e-5` | `-3.8223e-5` | `0` / `0` / `1.964e-7` |
| 88.2 kHz | `-5.2133e-5` | `1.97e-8` | `1.4188e-5` | `-3.7945e-5` | `0` / `0` / `2.622e-7` |
| 96 kHz | `-5.2411e-5` | `3.94e-8` | `1.4188e-5` | `-3.8223e-5` | `0` / `0` / `1.964e-7` |

`P(q)`: `1.419e-5` (q <= 4), `3.85e-6` (16), `1.18e-6` (63). Debug runs 48 kHz at quanta
{1, 2, 7, 63}.

**Rendered bits that move (before = `ad96a6327`, after = this attempt; one-off comparison with the
verifier's dump probe, not committed; all four launch rates, two-sine input, block of 256 then 4 x
64 after the target at frame A = 256).**
- Design-to-design retarget (new in this attempt, A3): HPF 1 kHz to 2 kHz moves 78-110 frames
  from A+6 on, max |diff| `9.24e-7` (44.1 kHz), `4.77e-7`, `4.17e-7`, `2.98e-7`; LPF 15 Hz to 30 Hz
  moves 56-227 frames from A+20, max |diff| `7.0e-10` to `4.7e-9`. Frames A to A+4 (the stepped
  words) are bit-identical.
- Disable / enable of the HPF (1 kHz) move 53-57 / 35-40 frames inside A+5..A+63, max |diff|
  `4.17e-7` / `3.58e-7`; the crossfade now tracks the `f64` blend more closely (disable `5.0e-8` to
  `6.2e-8` after against `3.5e-7` to `3.9e-7` before; enable `4.1e-8` to `6.4e-8` against `2.0e-7`
  to `3.2e-7`). The LPF disable/enable (1 kHz, 15 Hz) are bit-identical: their mix words are exact
  in both laws.
- Collapsed runs now equal dual runs where attempt 1 did not (gate 6).
- Pinned artifacts: none moved. `check-builtins-fixtures.sh`: ok (50 files); the wasm G5 corpus
  matches its pins on both legs; `check-browser-expected-resources.py --artifacts`: ok; no re-pin.

**Mutation evidence** (each applied to the working tree, run, reverted; the sweep runs the whole
`builtins` crate debug, gate 6 debug, gate 7 debug and `filter_liveness` release):

| Mutant | Red |
|---|---|
| M0 law reverted to `a18a2652d` | gates 1, 2, 2b, 3, 4 |
| M1 rule 1 removed | gate 1 |
| M1s rule 1 on settled lanes | `tests::trim_refresh_preserves_asymmetric_settled_filter_steps` (D1 canary) |
| M2 rule 2 removed | gates 2, 4 |
| M2c countdown restarts only when the mix differs | gate 2b (verdict MINOR 3) |
| M3 rule 3 removed | gates 2b, 3, 4 |
| M3b predicate ignores integrators | gate 3 |
| C1 rule-3 predicate reads channel `1` while collapsed | gate 6, both widths |
| C2 plan decides channel `1` over its own frozen integrators | gate 6 (debug abort at `process_mono`'s plan-symmetry assertion; release-only it renders the same bits) |
| attempt-1 `lib.rs` (`f44cf54bf`) | gate 6, both widths (seed 0) |
| C3 `export_lane` reads channel `1` while collapsed | none: unreachable (see audit); change not kept |
| K1 every word stepped (accumulated law) | gates 4 (release) and 7 |
| K2 every word from the target | gates 3 and 7 |
| K3 zero-step hold dropped | gates 2, 4 and 7 |
| ML3 mono body alone reverted to the accumulated law | gate 7 |
| stepped window 3 instead of 4 (lane constant) | gate 7 (before the owner-built countdown) |

Test value: gate 2b defends rule 2's unconditional restart (M2c); gate 6 defends the collapsed-stage
invariant (C1, C2, attempt-1 code) under live filter records and carried swaps, which no other test
drives; gate 7 defends the ramp arithmetic in both bodies at every width at rounding level (K1, K2,
K3); gate 4 now holds each history to its own designs and the proven allowance (M0, M2, M3, K1, K3).

**Gate commands and results (x86-64-v3 host, final tree):**
- `cargo test --locked --all-targets -p lane -p builtins -p dsp-reference --features builtins/test-support,lane/test-support`: pass (235 passed, 0 failed).
- `cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness`: pass (14 passed); per-rate table above.
- `cargo test --locked -p host-core --features test-support --test successor_swap`: pass (34 passed, gate 6 at both widths).
- `test-debug-a` workspace command: pass (1,428 passed, 0 failed). `test-debug-b` DSP command: pass (834 passed, 0 failed).
- `cargo build --locked --release -p audit && bash scripts/check-builtins-fixtures.sh . target/release/audit`: `builtins fixtures: ok (50 files)`.
- `bash scripts/run-wasm-gates.sh`: ok (native + wasm simd128 + V8 EQ loops); the four held V8 rows (dual depth-1 tail, mono depth-2 pair, mono depth-1 tail, mono depth-2 masked) all `ok`, no carried stack slot.
- Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`): pass; shipped module sha256 `8d3920eeed352468a4fe2dda1446311a933d919d086a0ff31236a0b6b4f9c95f`.
- `bash scripts/check-cross-targets.sh`: PASS (`builtins` iOS `memset_pattern16` at its ceiling 186; AArch64 rows check/lint only, tests CI-only).
- `check-builtins-policy.sh`, `check-workspace-policy.sh`, `check-lane-policy.sh`, `check-realtime-policy.sh`: ok. `cargo clippy --locked --workspace --all-targets -- -D warnings` and with `builtins/test-support,lane/test-support,host-core/test-support`: clean. `cargo fmt --all -- --check`: clean.

**Open:** root review of the four-stepped-word refinement of option (iii) (Amendment 1 A3); the
decision-15 ruling sentence is relayed by the coordinator; the release gate-4 sweep workflow is the
coordinator's follow-up issue; no listening run (spec: none required); AArch64 test execution is
CI-only. The proof's scope excludes test-support state injection (`set_lane_state_words` restoring
non-zero integrators onto an identity section makes rule 4 ramp from the identity, as gate 3's
second case does on purpose).

### Follow-up (verdict MINOR 1 and NITs)

Attempt 2's verdict was PASS with one MINOR and five NITs. This follow-up changes tests,
test-support and prose only. No render code changes, so no rendered bit moves.

- **MINOR 1, gate 8**
  (`filter_liveness::every_lane_steps_exactly_four_words_from_its_own_countdown`). It renders full
  `Simd4` and `Simd8` banks (`BankWidth::ALL`) through `process` and through `process_mono`, and one
  scalar chain, at 44.1 and 48 kHz. Each lane gets design-to-design retargets, re-sends and restarts
  on its own seeded schedule (about one block in five per lane, with `Left`/`Right`/`Both` on the
  dual path and `Both` on the collapsed one), so the lanes of a bank are out of phase. Blocks are 1
  to 16 frames. After every block, every current word of every lane, channel and section must equal
  an oracle of the A3 law: four words `current + step`, then `target - step * remaining`, a
  `+0.0`-step recursion word held, and the snap at the end. Each subject must also end blocks inside
  a leading window at least `4 x lanes` times (measured: 62 to 856). The new test-support reader
  `test_support::bank_section_words` is `input_section_words` for one bank lane.
- **Mutation evidence (gate 8)**, each applied, run on the whole `builtins` crate in debug and in
  release, then reverted:
  - LF1 (floor `64 - 4 + 1`, three stepped words): only gate 8 is red, debug and release. It is red
    on every subject, the scalar chain included.
  - LF2 (every lane takes lane 0's countdown): only gate 8 is red, debug and release. It is red on
    every bank subject (both widths, both bodies, both rates). The scalar chain stays green, because
    it has one lane.
  - Test value: a leading window off by one word, or one countdown shared across a bank, moves a
    ramp word at rounding level. Gate 8 is the only test that sees it, because gate 7 builds its own
    countdown and gate 3 checks frame 0 alone.
- **Gate 6 seeds.** The committed seeds are now `0, 7, 9` (were `0, 1, 2`). On C1 (the rule-3
  predicate reads channel 1 while collapsed), in debug, each of them is red at both widths. The
  first differing blocks are 61, 56 and 52, the same at `Simd4` and `Simd8`. Seeds 1 and 2 were red
  at `Simd4` only. A per-seed probe over seeds 0, 3, 5, 7 and 9 found all five red at both widths,
  which agrees with the verifier's 16-seed release probe. The three earliest-diverging seeds were
  kept, so the run time stays the same. The committed gate is red at both widths on C1 and green
  without it.
- **NIT 2.** `P(16)` and `P(63)` are now `3.85e-6` and `1.18e-6` (closed form: `3.8457e-6`,
  `1.1820e-6`), in the DSP evidence and in the attempt-2 record.
- **NIT 3.** The sentence about steps that "undershoot" is replaced. `64 s = fl(t - c)` may round
  past `t - c`, but a stepped word is at most four steps from its start and a later word at most
  59 steps from its target, and rounding is monotone. So every word lies between its start and its
  target.
- **NIT 4.** The ruling now states `E_start` and `E` separately, as the spec does, and rounds the
  `a2` floor to `9.716e-7` (`16.3 u = 9.7156e-7`).
- **NIT 5.** The `InputStage::collapsed` doc and the `swapping_run` doc are rewrapped to 100
  columns, and so are the spec's design-norm lines.

Gates (x86-64-v3, final tree):
- `cargo test --locked -p builtins --features test-support`: pass (129 passed).
- The same with `--release`: pass (129 passed). The gate-4 per-rate table is unchanged.
- `cargo test --locked -p host-core --features test-support --test successor_swap`: pass (34
  passed).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, plain and with
  `builtins/test-support,lane/test-support,host-core/test-support`: clean.
- `cargo fmt --all -- --check`: clean.
- `scripts/check-workspace-policy.sh`: ok.
